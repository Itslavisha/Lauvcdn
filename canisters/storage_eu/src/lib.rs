use candid::CandidType;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
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
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn put_chunk(file_id: String, index: u32, data: Vec<u8>, sha256: Vec<u8>) {
	CHUNKS.with(|m| { m.borrow_mut().insert((file_id, index), Chunk { data, sha256 }); });
}

#[update]
fn finalize_file(meta: FileMeta) {
	FILES.with(|m| { m.borrow_mut().insert(meta.file_id.clone(), meta); });
}

#[query]
fn get_chunk(file_id: String, index: u32) -> Option<Chunk> {
	CHUNKS.with(|m| m.borrow().get(&(file_id, index)).cloned())
}

#[query]
fn get_file_meta(file_id: String) -> Option<FileMeta> {
	FILES.with(|m| m.borrow().get(&file_id).cloned())
}
