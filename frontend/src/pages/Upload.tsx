import React, { useState } from 'react'
import { getActor } from '../ic/agent'
import storageUsIDL from '../ic/storage_us.idl'

async function sha256(buf: Uint8Array) {
  const hash = await crypto.subtle.digest('SHA-256', buf)
  return new Uint8Array(hash)
}

const STORAGE_US_CANISTER_ID = import.meta.env.VITE_STORAGE_US_CANISTER_ID || '<update-after-deploy>'

export default function UploadPage() {
  const [status, setStatus] = useState('')

  async function onUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0]
    if (!file) return
    setStatus('Uploading...')

    const actor = await getActor(STORAGE_US_CANISTER_ID, storageUsIDL)

    const chunkSize = 1024 * 1024 * 2
    const chunks = Math.ceil(file.size / chunkSize)
    const fileId = `${file.name}-${Date.now()}`

    for (let i = 0; i < chunks; i++) {
      const part = file.slice(i * chunkSize, Math.min(file.size, (i + 1) * chunkSize))
      const buf = new Uint8Array(await part.arrayBuffer())
      const hash = await sha256(buf)
      await actor.put_chunk(fileId, i, [...buf], [...hash])
    }

    // Minimal finalize (no merkle): use zero merkle_root for now
    const meta = {
      file_id: fileId,
      total_size: BigInt(file.size),
      chunk_size: chunkSize,
      num_chunks: chunks,
      merkle_root: [],
      created_ns: BigInt(Date.now()) * 1_000_000n,
    }
    await actor.finalize_file(meta)

    setStatus(`Done. file_id=${fileId}`)
  }

  return (
    <div className="p-6 space-y-4">
      <h2 className="text-xl font-semibold">Upload Content</h2>
      <input type="file" onChange={onUpload} />
      <div className="opacity-70">{status}</div>
    </div>
  )
}
