# Lauv: Decentralized CDN on Internet Computer

A simple, efficient decentralized Content Delivery Network built on the Internet Computer Protocol (ICP). Upload files, store them across the IC network, and access them via HTTP.

## 🏗️ Architecture

**Simple Two-Canister Design:**
- **Registry**: Manages file metadata, ownership, and permissions
- **Storage**: Handles file storage with chunking and HTTP access

## 🚀 Quick Start

### Prerequisites
- [DFX CLI](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) 
- [Rust toolchain](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (for frontend)

### Local Development

1. **Deploy canisters:**
   ```powershell
   .\scripts\deploy_local.ps1
   ```

2. **Start frontend:**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

3. **Access at:** http://localhost:5173

### Full Stack (Windows + WSL)
```powershell
.\scripts\run_full_stack.ps1
```

## 📡 Deployment

### Local Network
```powershell
.\scripts\deploy_local.ps1
```

### Internet Computer Mainnet
```powershell
.\scripts\deploy_ic.ps1 -Network ic
```

## 🔧 Usage

### Upload Files
1. Open the frontend dashboard
2. Select a file to upload
3. Files are automatically chunked and stored
4. Get a unique file ID for retrieval

### Download Files
- **Single chunk:** `https://{storage-canister}.ic0.app/file/{file-id}?chunk=0`
- **Full file:** `https://{storage-canister}.ic0.app/file/{file-id}/download`

## 🛠️ Development

### Project Structure
```
├── canisters/
│   ├── registry/     # File metadata & permissions
│   └── storage/      # File storage & HTTP access
├── frontend/         # React dashboard
├── sdk/             # Client SDKs (JS, Python, Rust)
└── scripts/         # Deployment scripts
```

### Building
```bash
# Build all canisters
cargo build --target wasm32-unknown-unknown --release

# Deploy locally
dfx deploy
```

## 📚 SDKs

Client libraries available for:
- **JavaScript/TypeScript** (`sdk/js/`)
- **Python** (`sdk/python/`)
- **Rust** (`sdk/rust/`)

## 🔐 Security

- Files are owned by the uploader's principal
- Access control via allow/deny lists
- Cryptographic integrity with SHA-256 hashing
- Merkle tree verification for large files

## 🌐 HTTP Access

Files are accessible via standard HTTP:
- Supports range requests for streaming
- Automatic content-type detection
- CDN-friendly caching headers

## 📄 License

Apache 2.0 - See LICENSE file for details.

---

Built with ❤️ on the Internet Computer