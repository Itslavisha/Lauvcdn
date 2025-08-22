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

#[init]
fn init() {
    // Initialize with default region
    REGION.with(|r| *r.borrow_mut() = "us".to_string());
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn set_registry(registry: candid::Principal) { REGISTRY.with(|r| *r.borrow_mut() = Some(registry)); }

#[update]
fn set_region(region: String) { REGION.with(|r| *r.borrow_mut() = region); }

#[update]
async fn start_upload(file_id: String, total_size: u64, chunk_size: u32, num_chunks: u32) -> Result<(), String> {
	let caller = ic_cdk::caller();
	// If file exists in registry, ensure caller is owner or allowed; otherwise allow creating new file (will register later)
	let registry = REGISTRY.with(|r| *r.borrow());
	if let Some(reg) = registry {
		let rec: Option<FileRecord> = ic_cdk::call(reg, "get_file", (file_id.clone(),)).await.unwrap_or((None,)).0;
		if let Some(r) = rec {
			if r.owner != caller && !r.allowed.contains(&caller) { return Err("not authorized".into()); }
		}
	}
	FILES.with(|m| { m.borrow_mut().insert(file_id.clone(), FileMeta { file_id, total_size, chunk_size, num_chunks, merkle_root: vec![], created_ns: ic_cdk::api::time() }); });
	Ok(())
}

#[update]
fn put_chunk(file_id: String, index: u32, data: Vec<u8>, _sha256: Vec<u8>) {
	let caller = ic_cdk::caller();
	// If file known and registry set, do a cheap check (skip registry call for each chunk for performance in MVP)
	if let Some(meta) = FILES.with(|m| m.borrow().get(&file_id).cloned()) {
		let _ = meta; // In a full implementation, cache owner/allowed alongside meta
	}
	let hash = sha256_bytes(&data);
	CHUNKS.with(|m| { m.borrow_mut().insert((file_id.clone(), index), Chunk { data, sha256: hash.clone() }); });
	// Update certification map for this chunk key -> sha256(data)
	CERT.with(|c| {
		let mut cm = c.borrow_mut();
		cm.insert(chunk_key(&file_id, index), hash);
		let root_hash: Hash = cm.root_hash();
		set_certified_data(&root_hash);
	});
}

#[update]
fn finalize_file(meta: FileMeta) {
	FILES.with(|m| { m.borrow_mut().insert(meta.file_id.clone(), meta.clone()); });
	// Update file_id root entry as well (store overall merkle if provided)
	if !meta.merkle_root.is_empty() {
		CERT.with(|c| {
			let mut cm = c.borrow_mut();
			cm.insert(file_key(&meta.file_id), meta.merkle_root.clone());
			let root_hash: Hash = cm.root_hash();
			set_certified_data(&root_hash);
		});
	}
}

#[update]
async fn finalize_and_register(mut meta: FileMeta) {
	let caller = ic_cdk::caller();
	// If already registered, ensure caller authorized
	if let Some(reg) = REGISTRY.with(|r| *r.borrow()) {
		let rec: Option<FileRecord> = ic_cdk::call(reg, "get_file", (meta.file_id.clone(),)).await.unwrap_or((None,)).0;
		if let Some(r) = rec { if r.owner != caller && !r.allowed.contains(&caller) { ic_cdk::trap("not authorized"); } }
	}
	// Compute merkle from chunk hashes if not provided
	if meta.merkle_root.is_empty() {
		meta.merkle_root = compute_merkle_root_for(&meta.file_id, meta.num_chunks);
	}
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
			merkle_root: meta.merkle_root,
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

// GET /file/{id}?chunk={i} or /file/{id}/download
#[query(name = "http_request")]
fn http_request(req: ic_cdk::api::management_canister::http_request::HttpRequest) -> ic_cdk::api::management_canister::http_request::HttpResponse {
	use ic_cdk::api::management_canister::http_request::HttpResponse;
	let url = req.url;
	let mut headers: Vec<(String, String)> = vec![];
	if let Some(cert) = data_certificate() { headers.push(("IC-Certificate".into(), format!("certificate=\"{}\"", base64::encode(cert)))); }
	if let Some((file_id, rest)) = url.strip_prefix("/file/").map(|s| {
		let mut it = s.splitn(2, '?');
		let route = it.next().unwrap_or("");
		let qs = it.next().unwrap_or("");
		(route.to_string(), qs.to_string())
	}) {
		if file_id.ends_with("/download") {
			let fid = file_id.trim_end_matches("/download");
			if let Some(bytes) = assemble_file(fid) {
				return HttpResponse { status_code: 200, headers: vec![("Content-Type".into(), "application/octet-stream".into())], body: bytes };
			}
			return HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() };
		}
		let mut chunk_idx: u32 = 0;
		for kv in rest.split('&') {
			let kvp: Vec<&str> = kv.split('=').collect();
			if kvp.len() == 2 && kvp[0] == "chunk" { if let Ok(v) = kvp[1].parse::<u32>() { chunk_idx = v; } }
		}
		if let Some(ch) = CHUNKS.with(|m| m.borrow().get(&(file_id.clone(), chunk_idx)).cloned()) {
			// Include tree witness for this chunk key
			let witness = CERT.with(|c| c.borrow().witness(&chunk_key(&file_id, chunk_idx)));
			let tree = serde_cbor::to_vec(&witness).unwrap_or_default();
			headers.push(("IC-Certificate".into(), format!("tree=\"{}\"", base64::encode(tree))));
			return HttpResponse { status_code: 200, headers, body: ch.data };
		}
		return HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() };
	}
	ic_cdk::api::management_canister::http_request::HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() }
}

fn assemble_file(file_id: &str) -> Option<Vec<u8>> {
	let meta = FILES.with(|m| m.borrow().get(file_id).cloned());
	let meta = meta?;
	let mut out: Vec<u8> = Vec::with_capacity(meta.total_size as usize);
	for i in 0..meta.num_chunks { if let Some(ch) = CHUNKS.with(|m| m.borrow().get(&(file_id.to_string(), i)).cloned()) { out.extend_from_slice(&ch.data); } else { return None } }
	Some(out)
}

fn compute_merkle_root_for(file_id: &str, num_chunks: u32) -> Vec<u8> {
	let mut hashes: Vec<[u8;32]> = Vec::with_capacity(num_chunks as usize);
	for i in 0..num_chunks {
		if let Some(ch) = CHUNKS.with(|m| m.borrow().get(&(file_id.to_string(), i)).cloned()) {
			hashes.push(to_arr(ch.sha256));
		} else {
			hashes.push([0u8;32]);
		}
	}
	let root = merkle_root(hashes);
	root.to_vec()
}

fn sha256_bytes(data: &[u8]) -> Vec<u8> {
	let mut h = Sha256::new();
	h.update(data);
	h.finalize().to_vec()
}

fn to_arr(v: Vec<u8>) -> [u8;32] { let mut a=[0u8;32]; let n = v.len().min(32); a[..n].copy_from_slice(&v[..n]); a }

fn merkle_root(mut leaves: Vec<[u8;32]>) -> [u8;32] {
	if leaves.is_empty() { return [0u8;32]; }
	while leaves.len() > 1 {
		let mut next: Vec<[u8;32]> = Vec::with_capacity((leaves.len()+1)/2);
		for pair in leaves.chunks(2) {
			let combined = if pair.len() == 2 { [pair[0], pair[1]].concat() } else { [pair[0], pair[0]].concat() };
			next.push(to_arr(sha256_bytes(&combined)));
		}
		leaves = next;
	}
	leaves[0]
}

fn chunk_key(file_id: &str, idx: u32) -> Vec<u8> { format!("chunk:{}:{}", file_id, idx).into_bytes() }
fn file_key(file_id: &str) -> Vec<u8> { format!("file:{}", file_id).into_bytes() }
