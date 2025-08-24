use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use ic_certified_map::{CertifiedMap as CertMap, Hash};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Chunk {
    pub data: Vec<u8>,
    pub sha256: Vec<u8>,
}

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
pub struct Replica {
    pub canister: Principal,
    pub region: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FileRecord {
    pub file_id: String,
    pub owner: Principal,
    pub replicas: Vec<Replica>,
    pub merkle_root: Vec<u8>,
    pub allowed: Vec<Principal>,
    pub created_ns: u64,
}

thread_local! {
    static CHUNKS: RefCell<HashMap<(String, u32), Chunk>> = RefCell::new(HashMap::new());
    static FILES: RefCell<HashMap<String, FileMeta>> = RefCell::new(HashMap::new());
    static CERT: RefCell<CertMap<Vec<u8>, Vec<u8>>> = RefCell::new(CertMap::new());
    static REGISTRY: RefCell<Option<candid::Principal>> = RefCell::new(None);
}

#[init]
fn init() {
    // Initialize storage canister
}

#[query]
fn health() -> String {
    "ok".to_string()
}

#[update]
fn set_registry(registry: candid::Principal) {
    REGISTRY.with(|r| *r.borrow_mut() = Some(registry));
}

#[update]
fn start_upload(file_id: String, total_size: u64, chunk_size: u32, num_chunks: u32) -> Result<(), String> {
    let meta = FileMeta {
        file_id: file_id.clone(),
        total_size,
        chunk_size,
        num_chunks,
        merkle_root: vec![],
        created_ns: ic_cdk::api::time(),
    };
    FILES.with(|f| f.borrow_mut().insert(file_id, meta));
    Ok(())
}

#[update]
fn put_chunk(file_id: String, chunk_index: u32, data: Vec<u8>, sha256: Vec<u8>) {
    let chunk = Chunk { data, sha256 };
    CHUNKS.with(|c| c.borrow_mut().insert((file_id, chunk_index), chunk));
}

#[update]
fn finalize_file(meta: FileMeta) {
    FILES.with(|f| f.borrow_mut().insert(meta.file_id.clone(), meta));
}

#[update]
async fn finalize_and_register(meta: FileMeta) {
    finalize_file(meta.clone());
    let registry = REGISTRY.with(|r| *r.borrow());
    if let Some(reg) = registry {
        let me = ic_cdk::id();
        let caller = ic_cdk::caller();
        let record = FileRecord {
            file_id: meta.file_id,
            owner: caller,
            replicas: vec![Replica { canister: me, region: "global".to_string() }],
            merkle_root: meta.merkle_root,
            allowed: vec![],
            created_ns: meta.created_ns,
        };
        let _: () = ic_cdk::call(reg, "register_file", (record,)).await.unwrap();
    }
}

#[query]
fn get_chunk(file_id: String, chunk_index: u32) -> Option<Chunk> {
    CHUNKS.with(|c| c.borrow().get(&(file_id, chunk_index)).cloned())
}

#[query]
fn get_file_meta(file_id: String) -> Option<FileMeta> {
    FILES.with(|f| f.borrow().get(&file_id).cloned())
}

#[query]
fn http_request(req: ic_cdk::api::management_canister::http_request::HttpRequest) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    let path = req.url;
    
    if path.starts_with("/file/") {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 3 {
            let file_id = urlencoding::decode(parts[2]).unwrap_or_default().to_string();
            
            if path.contains("?chunk=") {
                // Single chunk request
                if let Some(chunk_str) = path.split("?chunk=").nth(1) {
                    if let Ok(chunk_index) = chunk_str.parse::<u32>() {
                        if let Some(chunk) = get_chunk(file_id, chunk_index) {
                            return ic_cdk::api::management_canister::http_request::HttpResponse {
                                status_code: 200,
                                headers: vec![
                                    ("Content-Type".to_string(), "application/octet-stream".to_string()),
                                ],
                                body: chunk.data,
                            };
                        }
                    }
                }
            } else if path.ends_with("/download") {
                // Full file download
                let file_id = file_id.trim_end_matches("/download");
                if let Some(meta) = get_file_meta(file_id.to_string()) {
                    let mut full_data = Vec::new();
                    for i in 0..meta.num_chunks {
                        if let Some(chunk) = get_chunk(file_id.to_string(), i) {
                            full_data.extend(chunk.data);
                        }
                    }
                    return ic_cdk::api::management_canister::http_request::HttpResponse {
                        status_code: 200,
                        headers: vec![
                            ("Content-Type".to_string(), "application/octet-stream".to_string()),
                            ("Content-Disposition".to_string(), format!("attachment; filename=\"{}\"", file_id)),
                        ],
                        body: full_data,
                    };
                }
            }
        }
    }
    
    ic_cdk::api::management_canister::http_request::HttpResponse {
        status_code: 404,
        headers: vec![],
        body: b"Not found".to_vec(),
    }
}