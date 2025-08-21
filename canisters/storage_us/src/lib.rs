use candid::CandidType;
use ic_cdk::api::set_certified_data;
use ic_cdk_macros::*;
use ic_certified_map::{labeled_hash, Hash, RbTree as CertMap};
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

thread_local! {
	static CHUNKS: RefCell<HashMap<(String, u32), Chunk>> = RefCell::new(HashMap::new());
	static FILES: RefCell<HashMap<String, FileMeta>> = RefCell::new(HashMap::new());
	static CERT: RefCell<CertMap<Vec<u8>, Vec<u8>>> = RefCell::new(CertMap::new());
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn put_chunk(file_id: String, index: u32, data: Vec<u8>, sha256: Vec<u8>) {
	CHUNKS.with(|m| { m.borrow_mut().insert((file_id, index), Chunk { data, sha256 }); });
}

#[update]
fn finalize_file(meta: FileMeta) {
	// Record metadata and set certification for file_id -> merkle_root
	FILES.with(|m| { m.borrow_mut().insert(meta.file_id.clone(), meta.clone()); });
	CERT.with(|c| {
		let mut cm = c.borrow_mut();
		cm.insert(meta.file_id.as_bytes().to_vec(), meta.merkle_root.clone());
		let root_hash: Hash = cm.root_hash();
		set_certified_data(&root_hash);
	});
}

#[query]
fn get_chunk(file_id: String, index: u32) -> Option<Chunk> {
	CHUNKS.with(|m| m.borrow().get(&(file_id, index)).cloned())
}

#[query]
fn get_file_meta(file_id: String) -> Option<FileMeta> {
	FILES.with(|m| m.borrow().get(&file_id).cloned())
}

// Simplified http_request: GET /file/{id}?chunk={i}
#[query(name = "http_request")]
fn http_request(req: ic_cdk::api::management_canister::http_request::HttpRequest) -> ic_cdk::api::management_canister::http_request::HttpResponse {
	use ic_cdk::api::management_canister::http_request::HttpResponse;
	let path = req.url;
	// Very basic parsing
	let parts: Vec<&str> = path.split('?').collect();
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
			let body = ch.data;
			let cert_header = certified_header_for_file(file_id);
			return HttpResponse { status_code: 200, headers: cert_header, body };
		}
		return HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() };
	}
	HttpResponse { status_code: 404, headers: vec![], body: b"not found".to_vec() }
}

fn certified_header_for_file(file_id: &str) -> Vec<(String, String)> {
	// Provide IC certification headers (simplified; boundary nodes verify)
	let root_hash = CERT.with(|c| c.borrow().root_hash());
	let tree_label = labeled_hash(b"file", &hash_bytes(file_id.as_bytes()));
	let _ = tree_label; // placeholder to show labeling; full tree witness omitted here
	vec![
		("IC-Certificate".to_string(), base64::encode(root_hash)),
	]
}

fn hash_bytes(data: &[u8]) -> Hash {
	let mut h = Sha256::new();
	h.update(data);
	h.finalize().into()
}
