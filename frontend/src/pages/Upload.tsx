import React, { useState } from 'react'
import { getActor } from '../ic/agent'
import storageIDL from '../ic/storage.idl'

async function sha256(buf: Uint8Array) {
  const hash = await crypto.subtle.digest('SHA-256', buf)
  return new Uint8Array(hash)
}

const STORAGE_CANISTER_ID = import.meta.env.VITE_STORAGE_CANISTER_ID || 'rdmx6-jaaaa-aaaaa-aaadq-cai'

export default function UploadPage() {
  const [status, setStatus] = useState('')
  const [isUploading, setIsUploading] = useState(false)

  async function onUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0]
    if (!file || isUploading) return
    
    setIsUploading(true)
    setStatus('Preparing...')

    try {
      const actor = await getActor(STORAGE_CANISTER_ID, storageIDL, false)

      const chunkSize = 1024 * 1024 * 2 // 2MB chunks
      const chunks = Math.ceil(file.size / chunkSize)
      const fileId = `${file.name}-${Date.now()}`

      setStatus('Starting upload...')
      const res = await actor.start_upload(fileId, BigInt(file.size), chunkSize, chunks)
      if ('err' in res) {
        setStatus(`Error: ${res.err}`)
        return
      }

      setStatus('Uploading chunks...')
      for (let i = 0; i < chunks; i++) {
        const start = i * chunkSize
        const end = Math.min(file.size, (i + 1) * chunkSize)
        const part = file.slice(start, end)
        const buf = new Uint8Array(await part.arrayBuffer())
        const hash = await sha256(buf)
        
        await actor.put_chunk(fileId, i, [...buf], [...hash])
        setStatus(`Uploading... ${Math.round((i + 1) / chunks * 100)}%`)
      }

      setStatus('Finalizing...')
      const meta = {
        file_id: fileId,
        total_size: BigInt(file.size),
        chunk_size: chunkSize,
        num_chunks: chunks,
        merkle_root: [],
        created_ns: BigInt(Date.now()) * 1_000_000n,
      }
      await actor.finalize_and_register(meta)

      setStatus(`✅ Upload complete! File ID: ${fileId}`)
    } catch (error) {
      console.error('Upload error:', error)
      setStatus(`❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsUploading(false)
    }
  }

  return (
    <div className="p-6 space-y-4 bg-white dark:bg-neutral-800 rounded-lg shadow">
      <h2 className="text-xl font-semibold">Upload File</h2>
      <div className="space-y-2">
        <input 
          type="file" 
          onChange={onUpload} 
          disabled={isUploading}
          className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100 disabled:opacity-50"
        />
        {isUploading && (
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div className="bg-blue-600 h-2 rounded-full animate-pulse"></div>
          </div>
        )}
      </div>
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