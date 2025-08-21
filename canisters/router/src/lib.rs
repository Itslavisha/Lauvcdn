use candid::Principal;
use ic_cdk_macros::*;

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

#[query]
fn health() -> String { "ok".to_string() }

#[query]
async fn route_for(file_id: String, _region_hint: Option<String>) -> Vec<Principal> {
    let registry: Principal = ic_cdk::caller(); // placeholder; pass registry via env/config later
    let rec: Option<FileRecord> = ic_cdk::call(registry, "get_file", (file_id,)).await.unwrap_or((None,)).0;
    rec.map(|r| r.replicas.into_iter().map(|x| x.canister).collect()).unwrap_or_default()
}
