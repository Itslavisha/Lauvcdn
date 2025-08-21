use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Account { pub user: Principal, pub credits_nanos: u128, pub plan: String }

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Usage { pub user: Principal, pub requests: u64, pub egress_bytes: u64, pub ingress_bytes: u64 }

thread_local! {
	static ACCOUNTS: RefCell<HashMap<Principal, Account>> = RefCell::new(HashMap::new());
	static USAGE: RefCell<HashMap<Principal, Usage>> = RefCell::new(HashMap::new());
}

#[query]
fn health() -> String { "ok".to_string() }

#[update]
fn record_usage(user: Principal, requests: u64, egress_bytes: u64, ingress_bytes: u64) {
	USAGE.with(|m| {
		let mut map = m.borrow_mut();
		let entry = map.entry(user).or_insert(Usage { user, requests: 0, egress_bytes: 0, ingress_bytes: 0 });
		entry.requests += requests;
		entry.egress_bytes += egress_bytes;
		entry.ingress_bytes += ingress_bytes;
	});
}

#[update]
fn recharge(user: Principal, credits_nanos: u128) {
	ACCOUNTS.with(|m| {
		let mut map = m.borrow_mut();
		let entry = map.entry(user).or_insert(Account { user, credits_nanos: 0, plan: "standard".to_string() });
		entry.credits_nanos += credits_nanos;
	});
}

#[query]
fn account(user: Principal) -> Option<Account> { ACCOUNTS.with(|m| m.borrow().get(&user).cloned()) }

#[query]
fn usage(user: Principal) -> Option<Usage> { USAGE.with(|m| m.borrow().get(&user).cloned()) }
