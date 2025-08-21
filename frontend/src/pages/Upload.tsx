import React, { useState } from 'react'

async function sha256(buf: Uint8Array) {
  const hash = await crypto.subtle.digest('SHA-256', buf)
  return new Uint8Array(hash)
}

export default function UploadPage() {
  const [status, setStatus] = useState('')

  async function onUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0]
    if (!file) return
    setStatus('Uploading...')

    const chunkSize = 1024 * 1024 * 2
    const chunks = Math.ceil(file.size / chunkSize)
    const fileId = `${file.name}-${Date.now()}`

    for (let i = 0; i < chunks; i++) {
      const part = file.slice(i * chunkSize, Math.min(file.size, (i + 1) * chunkSize))
      const buf = new Uint8Array(await part.arrayBuffer())
      const hash = await sha256(buf)
      // TODO: call storage_us actor: put_chunk(fileId, i, [...buf], [...hash])
    }

    // TODO: compute merkle root and call finalize_file(meta)
    setStatus('Done (skeleton)')
  }

  return (
    <div className="p-6 space-y-4">
      <h2 className="text-xl font-semibold">Upload Content</h2>
      <input type="file" onChange={onUpload} />
      <div className="opacity-70">{status}</div>
    </div>
  )
}
