import React, { useState } from 'react'
import { getActor } from '../ic/agent'
import storageIDL from '../ic/storage.idl'

const STORAGE_CANISTER_ID = import.meta.env.VITE_STORAGE_CANISTER_ID || 'rdmx6-jaaaa-aaaaa-aaadq-cai'
const BASE_URL = `https://${STORAGE_CANISTER_ID}.localhost:4943`

export default function DownloadPage() {
  const [fileId, setFileId] = useState('')
  const [status, setStatus] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [fileInfo, setFileInfo] = useState<any>(null)

  async function getFileInfo() {
    if (!fileId || isLoading) return
    
    setIsLoading(true)
    setStatus('Getting file info...')
    
    try {
      const actor = await getActor(STORAGE_CANISTER_ID, storageIDL, false)
      const meta = await actor.get_file_meta(fileId)
      
      if (meta && meta.length > 0) {
        setFileInfo(meta[0])
        setStatus(`✅ File found: ${(Number(meta[0].total_size) / 1024 / 1024).toFixed(2)} MB, ${meta[0].num_chunks} chunks`)
      } else {
        setFileInfo(null)
        setStatus('❌ File not found')
      }
    } catch (error) {
      setStatus(`❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
      setFileInfo(null)
    } finally {
      setIsLoading(false)
    }
  }

  async function downloadFile() {
    if (!fileId || isLoading) return
    
    setIsLoading(true)
    setStatus('Downloading...')
    
    try {
      const url = `${BASE_URL}/file/${encodeURIComponent(fileId)}/download`
      const response = await fetch(url)
      
      if (!response.ok) {
        setStatus(`❌ Download failed: ${response.status} ${response.statusText}`)
        return
      }
      
      const blob = await response.blob()
      const downloadUrl = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = downloadUrl
      a.download = fileId.split('-')[0] || fileId // Use original filename if available
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(downloadUrl)
      
      setStatus('✅ Download complete!')
    } catch (error) {
      setStatus(`❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsLoading(false)
    }
  }

  async function previewChunk() {
    if (!fileId || isLoading) return
    
    setIsLoading(true)
    setStatus('Fetching first chunk...')
    
    try {
      const url = `${BASE_URL}/file/${encodeURIComponent(fileId)}?chunk=0`
      const response = await fetch(url)
      
      if (response.ok) {
        const size = response.headers.get('content-length') || 'unknown'
        setStatus(`✅ First chunk loaded (${size} bytes)`)
      } else {
        setStatus(`❌ Chunk fetch failed: ${response.status} ${response.statusText}`)
      }
    } catch (error) {
      setStatus(`❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="p-6 space-y-4 bg-white dark:bg-neutral-800 rounded-lg shadow">
      <h2 className="text-xl font-semibold">Download File</h2>
      
      <div className="space-y-3">
        <input 
          className="w-full border border-gray-300 dark:border-neutral-600 px-3 py-2 rounded bg-white dark:bg-neutral-700 disabled:opacity-50" 
          placeholder="Enter file ID" 
          value={fileId} 
          onChange={e => setFileId(e.target.value)}
          disabled={isLoading}
        />
        
        <div className="flex flex-wrap gap-2">
          <button 
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed" 
            onClick={getFileInfo}
            disabled={!fileId || isLoading}
          >
            Get Info
          </button>
          <button 
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed" 
            onClick={downloadFile}
            disabled={!fileId || isLoading}
          >
            Download
          </button>
          <button 
            className="px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed" 
            onClick={previewChunk}
            disabled={!fileId || isLoading}
          >
            Preview Chunk
          </button>
        </div>
      </div>

      {fileInfo && (
        <div className="p-3 bg-blue-50 dark:bg-blue-900/20 rounded text-sm">
          <h3 className="font-semibold mb-2">File Information:</h3>
          <ul className="space-y-1">
            <li><strong>Size:</strong> {(Number(fileInfo.total_size) / 1024 / 1024).toFixed(2)} MB</li>
            <li><strong>Chunks:</strong> {fileInfo.num_chunks}</li>
            <li><strong>Chunk Size:</strong> {(fileInfo.chunk_size / 1024 / 1024).toFixed(2)} MB</li>
            <li><strong>Created:</strong> {new Date(Number(fileInfo.created_ns) / 1_000_000).toLocaleString()}</li>
          </ul>
        </div>
      )}

      {status && (
        <div className={`p-3 rounded text-sm ${
          status.includes('✅') ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' :
          status.includes('❌') ? 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200' :
          'bg-gray-100 dark:bg-neutral-700'
        }`}>
          {status}
        </div>
      )}
    </div>
  )
}