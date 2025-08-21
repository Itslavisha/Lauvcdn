import React, { useState } from 'react'

const STORAGE_US_CANISTER_ID = import.meta.env.VITE_STORAGE_US_CANISTER_ID || '<update-after-deploy>'
const BASE = `https://${STORAGE_US_CANISTER_ID}.localhost:4943`

export default function DownloadPage() {
  const [fileId, setFileId] = useState('')
  const [status, setStatus] = useState('')

  async function fetchChunk() {
    const url = `${BASE}/file/${encodeURIComponent(fileId)}?chunk=0`
    setStatus('Fetching chunk 0...')
    const res = await fetch(url)
    setStatus(`${res.status} ${res.statusText}`)
  }

  async function fetchAll() {
    const url = `${BASE}/file/${encodeURIComponent(fileId)}/download`
    setStatus('Downloading...')
    const res = await fetch(url)
    const blob = await res.blob()
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = fileId
    a.click()
    setStatus('Downloaded')
  }

  return (
    <div className="p-6 space-y-4">
      <h2 className="text-xl font-semibold">Download</h2>
      <input className="border px-2 py-1" placeholder="file_id" value={fileId} onChange={e=>setFileId(e.target.value)} />
      <div className="space-x-2">
        <button className="px-3 py-1 bg-blue-600 text-white" onClick={fetchChunk}>Fetch chunk 0</button>
        <button className="px-3 py-1 bg-green-600 text-white" onClick={fetchAll}>Download all</button>
      </div>
      <div className="opacity-70">{status}</div>
    </div>
  )
}
