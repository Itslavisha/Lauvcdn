use candid::CandidType;
use ic_cdk::api::{data_certificate, set_certified_data};
use ic_cdk_macros::*;
use ic_certified_map::{Hash, RbTree as CertMap};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Chunk { pub data: Vec<u8>, pub sha256: Vec<u8> }

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FileMeta {
	pub file_id: String,
	pub total_size: u64,
	pub chunk_size: u32,
	pub num_chunks: u32,
	pub merkle_root: Vec<u8>,
	pub created_ns: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Replica { pub canister: candid::Principal, pub region: String }

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FileRecord {
	pub file_id: String,
	pub owner: candid::Principal,
	pub replicas: Vec<Replica>,
	pub merkle_root: Vec<u8>,
	pub allowed: Vec<candid::Principal>,
	pub created_ns: u64,
}

thread_local! {
	static CHUNKS: RefCell<HashMap<(String, u32), Chunk>> = RefCell::new(HashMap::new());
	static FILES: RefCell<HashMap<String, FileMeta>> = RefCell::new(HashMap::new());
	static CERT: RefCell<CertMap<Vec<u8>, Vec<u8>>> = RefCell::new(CertMap::new());
	static REGISTRY: RefCell<Option<candid::Principal>> = RefCell::new(None);
	static REGION: RefCell<String> = RefCell::new("us".to_string());
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn set_registry(registry: candid::Principal) { REGISTRY.with(|r| *r.borrow_mut() = Some(registry)); }

#[update]
fn set_region(region: String) { REGION.with(|r| *r.borrow_mut() = region); }

#[update]
fn put_chunk(file_id: String, index: u32, data: Vec<u8>, sha256: Vec<u8>) {
	CHUNKS.with(|m| { m.borrow_mut().insert((file_id, index), Chunk { data, sha256 }); });
}

#[update]
fn finalize_file(meta: FileMeta) {
	// Keep for compatibility (no registry call). Will compute merkle if not provided.
	let computed_root = compute_merkle_root_for(&meta.file_id, meta.num_chunks);
	let merkle_root = if meta.merkle_root.is_empty() { computed_root } else { meta.merkle_root.clone() };
	let meta2 = FileMeta { merkle_root: merkle_root.clone(), ..meta };
	FILES.with(|m| { m.borrow_mut().insert(meta2.file_id.clone(), meta2.clone()); });
	CERT.with(|c| {
		let mut cm = c.borrow_mut();
		cm.insert(meta2.file_id.as_bytes().to_vec(), merkle_root);
		let root_hash: Hash = cm.root_hash();
		set_certified_data(&root_hash);
	});
}

#[update]
async fn finalize_and_register(mut meta: FileMeta) {
	// Compute merkle, store and certify, then call registry.register_file
	let merkle_root = if meta.merkle_root.is_empty() { compute_merkle_root_for(&meta.file_id, meta.num_chunks) } else { meta.merkle_root.clone() };
	meta.merkle_root = merkle_root.clone();
	finalize_file(meta.clone());
	let registry = REGISTRY.with(|r| *r.borrow());
	if let Some(reg) = registry {
		let region = REGION.with(|r| r.borrow().clone());
		let me = ic_cdk::id();
		let caller = ic_cdk::caller();
		let record = FileRecord {
			file_id: meta.file_id,
			owner: caller,
			replicas: vec![Replica { canister: me, region }],
			merkle_root,
			allowed: vec![],
			created_ns: meta.created_ns,
		};
		let _: () = ic_cdk::call(reg, "register_file", (record,)).await.unwrap_or_default();
	}
}

#[query]
fn get_chunk(file_id: String, index: u32) -> Option<Chunk> {
	CHUNKS.with(|m| m.borrow().get(&(file_id, index)).cloned())
}

#[query]
fn get_file_meta(file_id: String) -> Option<FileMeta> {
	FILES.with(|m| m.borrow().get(&file_id).cloned())
}

// GET /file/{id}?chunk={i}
#[query(name = "http_request")]
fn http_request(req: ic_cdk::api::management_canister::http_request::HttpRequest) -> ic_cdk::api::management_canister::http_request::HttpResponse {
	use ic_cdk::api::management_canister::http_request::HttpResponse;
	let url = req.url;
	let parts: Vec<&str> = url.split('?').collect();
	let (route, qs) = (parts.get(0).unwrap_or(&""), parts.get(1).unwrap_or(&""));
	if route.starts_with("/file/") {
		let file_id = route.trim_start_matches("/file/");
		let mut chunk_idx: u32 = 0;
		for kv in qs.split('&') {
			let kvp: Vec<&str> = kv.split('=').collect();
			if kvp.len() == 2 && kvp[0] == "chunk" {
				if let Ok(v) = kvp[1].parse::<u32>() { chunk_idx = v; }
			}
		}
		if let Some(ch) = CHUNKS.with(|m| m.borrow().get(&(file_id.to_string(), chunk_idx)).cloned()) {
			let mut headers: Vec<(String, String)> = vec![("Content-Type".into(), "application/octet-stream".into())];
			if let Some(cert) = data_certificate() {
				let witness = CERT.with(|c| c.borrow().witness(file_id.as_bytes()));
				let tree = serde_cbor::to_vec(&witness).unwrap_or_default();
				let ic_cert = format!("certificate=\"{}\", tree=\"{}\"", base64::encode(cert), base64::encode(tree));
				headers.push(("IC-Certificate".into(), ic_cert));
			}
			return HttpResponse { status_code: 200, headers, body: ch.data };
		}
		return HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() };
	}
	HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() }
}

fn compute_merkle_root_for(file_id: &str, num_chunks: u32) -> Vec<u8> {
	let mut hashes: Vec<[u8;32]> = Vec::with_capacity(num_chunks as usize);
	for i in 0..num_chunks {
		if let Some(ch) = CHUNKS.with(|m| m.borrow().get(&(file_id.to_string(), i)).cloned()) {
			hashes.push(hash_bytes(&ch.data));
		} else {
			hashes.push([0u8;32]);
		}
	}
	let root = merkle_root(hashes);
	root.to_vec()
}

fn hash_bytes(data: &[u8]) -> [u8;32] {
	let mut h = Sha256::new();
	h.update(data);
	let out = h.finalize();
	let mut arr = [0u8;32];
	arr.copy_from_slice(&out);
	arr
}

fn merkle_root(mut leaves: Vec<[u8;32]>) -> [u8;32] {
	if leaves.is_empty() { return [0u8;32]; }
	while leaves.len() > 1 {
		let mut next: Vec<[u8;32]> = Vec::with_capacity((leaves.len()+1)/2);
		for pair in leaves.chunks(2) {
			let combined = if pair.len() == 2 { [pair[0], pair[1]].concat() } else { [pair[0], pair[0]].concat() };
			next.push(hash_bytes(&combined));
		}
		leaves = next;
	}
	leaves[0]
}
