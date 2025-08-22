import React from 'react'
import UploadPage from './pages/Upload'
import DownloadPage from './pages/Download'

function App() {
  return (
    <div className="min-h-screen bg-neutral-100 text-neutral-900 dark:bg-neutral-900 dark:text-neutral-50">
      <div className="p-6 space-y-6">
        <h1 className="text-2xl font-bold">Lauv CDN Dashboard</h1>
        <UploadPage />
        <DownloadPage />
      </div>
    </div>
  )
}

export default App