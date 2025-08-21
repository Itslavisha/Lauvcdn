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

#[query]
fn health() -> String { "ok".to_string() }

#[query]
async fn route_for(file_id: String, region_hint: Option<String>) -> Vec<Principal> {
    let registry = REGISTRY.with(|r| *r.borrow());
    let Some(reg) = registry else { return vec![] };
    let rec: Option<FileRecord> = ic_cdk::call(reg, "get_file", (file_id,)).await.unwrap_or((None,)).0;
    let mut reps: Vec<Replica> = rec.map(|r| r.replicas).unwrap_or_default();
    if let Some(hint) = region_hint {
        reps.sort_by_key(|r| if r.region == hint { 0 } else { 1 });
    }
    reps.into_iter().map(|x| x.canister).collect()
}
