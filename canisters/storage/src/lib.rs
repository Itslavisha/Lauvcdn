@@ .. @@
 thread_local! {
 	static CHUNKS: RefCell<HashMap<(String, u32), Chunk>> = RefCell::new(HashMap::new());
 	static FILES: RefCell<HashMap<String, FileMeta>> = RefCell::new(HashMap::new());
 	static CERT: RefCell<CertMap<Vec<u8>, Vec<u8>>> = RefCell::new(CertMap::new());
 	static REGISTRY: RefCell<Option<candid::Principal>> = RefCell::new(None);
-	static REGION: RefCell<String> = RefCell::new("us".to_string());
 }
 
 #[init]
 fn init() {
-    // Initialize with default region
-    REGION.with(|r| *r.borrow_mut() = "us".to_string());
+    // Initialize storage canister
 }
 
 #[query]
@@ .. @@
 #[update]
 fn set_registry(registry: candid::Principal) { REGISTRY.with(|r| *r.borrow_mut() = Some(registry)); }
 
-#[update]
-fn set_region(region: String) { REGION.with(|r| *r.borrow_mut() = region); }
-
 #[update]
@@ .. @@
 	finalize_file(meta.clone());
 	let registry = REGISTRY.with(|r| *r.borrow());
 	if let Some(reg) = registry {
-		let region = REGION.with(|r| r.borrow().clone());
 		let me = ic_cdk::id();
 		let caller = ic_cdk::caller();
 		let record = FileRecord {
 			file_id: meta.file_id,
 			owner: caller,
-			replicas: vec![Replica { canister: me, region }],
+			replicas: vec![Replica { canister: me, region: "global".to_string() }],
 			merkle_root: meta.merkle_root,
 			allowed: vec![],
 			created_ns: meta.created_ns,