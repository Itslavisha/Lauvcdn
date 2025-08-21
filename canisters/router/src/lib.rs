use candid::Principal;
use ic_cdk_macros::*;

#[query]
fn health() -> String { "ok".to_string() }

#[query]
fn route_for(_file_id: String, _region_hint: Option<String>) -> Vec<Principal> {
	Vec::new()
}
