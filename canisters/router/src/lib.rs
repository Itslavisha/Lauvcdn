use candid::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(Clone, Debug, candid::CandidType, serde::Deserialize, serde::Serialize)]
pub struct Replica { pub canister: Principal, pub region: String }

#[derive(Clone, Debug, candid::CandidType, serde::Deserialize, serde::Serialize)]
pub struct FileRecord {
    pub file_id: String,
    pub owner: Principal,
    pub replicas: Vec<Replica>,
    pub merkle_root: Vec<u8>,
    pub allowed: Vec<Principal>,
    pub created_ns: u64,
}

thread_local! {
    static REGISTRY: RefCell<Option<Principal>> = RefCell::new(None);
}

#[update]
fn set_registry(p: Principal) { REGISTRY.with(|r| *r.borrow_mut() = Some(p)); }

#[init]
fn init() {
    // Initialize with empty registry - will be set via set_registry call
}

#[query]
fn health() -> String { "ok".to_string() }

#[query]
fn route_for(file_id: String, region_hint: Option<String>) -> Vec<Principal> {
    let registry = REGISTRY.with(|r| *r.borrow());
    // For now, return hardcoded storage canisters - in production this would query registry
    // This is a simplified implementation for the MVP
    vec![]
}
