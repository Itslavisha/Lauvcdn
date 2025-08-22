import React, { useState } from 'react'

const STORAGE_US_CANISTER_ID = import.meta.env.VITE_STORAGE_US_CANISTER_ID || 'rdmx6-jaaaa-aaaaa-aaadq-cai'
const BASE = `https://${STORAGE_US_CANISTER_ID}.localhost:4943`

export default function DownloadPage() {
  const [fileId, setFileId] = useState('')
  const [status, setStatus] = useState('')
  const [isLoading, setIsLoading] = useState(false)

  async function fetchChunk() {
    if (!fileId || isLoading) return
    
    setIsLoading(true)
    try {
      const url = `${BASE}/file/${encodeURIComponent(fileId)}?chunk=0`
      setStatus('Fetching chunk 0...')
      const res = await fetch(url)
      setStatus(`${res.status} ${res.statusText}`)
    } catch (error) {
      setStatus(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsLoading(false)
    }
  }

  async function fetchAll() {
    if (!fileId || isLoading) return
    
    setIsLoading(true)
    try {
      const url = `${BASE}/file/${encodeURIComponent(fileId)}/download`
      setStatus('Downloading...')
      const res = await fetch(url)
      
      if (!res.ok) {
        setStatus(`Error: ${res.status} ${res.statusText}`)
        return
      }
      
      const blob = await res.blob()
      const a = document.createElement('a')
      a.href = URL.createObjectURL(blob)
      a.download = fileId
      a.click()
      setStatus('Downloaded')
    } catch (error) {
      setStatus(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="p-6 space-y-4 bg-white dark:bg-neutral-800 rounded-lg shadow">
      <h2 className="text-xl font-semibold">Download</h2>
      <input 
        className="w-full border border-gray-300 dark:border-neutral-600 px-3 py-2 rounded bg-white dark:bg-neutral-700" 
        placeholder="Enter file_id" 
        value={fileId} 
        onChange={e => setFileId(e.target.value)}
        disabled={isLoading}
      />
      <div className="space-x-2">
        <button 
          className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50" 
          onClick={fetchChunk}
          disabled={!fileId || isLoading}
        >
          Fetch chunk 0
        </button>
        <button 
          className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50" 
          onClick={fetchAll}
          disabled={!fileId || isLoading}
        >
          Download all
        </button>
      </div>
      {status && (
        <div className="p-3 bg-gray-100 dark:bg-neutral-700 rounded text-sm">
          {status}
        </div>
      )}
    </div>
  )
}