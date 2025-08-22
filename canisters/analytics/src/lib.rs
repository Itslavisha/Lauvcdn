use candid::CandidType;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Point { pub ts_ns: u64, pub value: u64 }

#[init]
fn init() {
    // Initialize analytics system
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn ingest(_blob: Vec<u8>) { /* no-op for skeleton */ }

#[query]
fn rps(_user: candid::Principal, _start_ns: u64, _end_ns: u64, _step_ns: u64) -> Vec<Point> { vec![] }

#[query]
fn bandwidth(_user: candid::Principal, _start_ns: u64, _end_ns: u64, _step_ns: u64) -> Vec<Point> { vec![] }

#[query]
fn heatmap(_user: candid::Principal, _start_ns: u64, _end_ns: u64) -> Vec<(String, u64)> { vec![] }

#[query]
fn hit_ratio(_user: candid::Principal, _start_ns: u64, _end_ns: u64, _step_ns: u64) -> Vec<Point> { vec![] }
