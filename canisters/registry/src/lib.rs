use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Replica { pub canister: Principal, pub region: String }

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FileRecord {
	pub file_id: String,	pub owner: Principal,	pub replicas: Vec<Replica>,
	pub merkle_root: Vec<u8>,	pub allowed: Vec<Principal>,	pub created_ns: u64,
}

thread_local! {
	static FILES: RefCell<HashMap<String, FileRecord>> = RefCell::new(HashMap::new());
}

#[init]
fn init() {
    // Initialize registry
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn register_file(record: FileRecord) { FILES.with(|m| { m.borrow_mut().insert(record.file_id.clone(), record); }); }

#[query]
fn get_file(file_id: String) -> Option<FileRecord> { FILES.with(|m| m.borrow().get(&file_id).cloned()) }

#[update]
fn add_allowed(file_id: String, p: Principal) {
	FILES.with(|m| {
		let mut map = m.borrow_mut();
		if let Some(rec) = map.get_mut(&file_id) {
			if ic_cdk::caller() == rec.owner && !rec.allowed.contains(&p) { rec.allowed.push(p); }
		}
	});
}

#[update]
fn remove_allowed(file_id: String, p: Principal) {
	FILES.with(|m| {
		let mut map = m.borrow_mut();
		if let Some(rec) = map.get_mut(&file_id) {
			if ic_cdk::caller() == rec.owner { rec.allowed.retain(|x| x != &p); }
		}
	});
}
