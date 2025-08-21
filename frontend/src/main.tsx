import React from 'react'
import { createRoot } from 'react-dom/client'
import './styles.css'

function App() {
  return (
    <div className="min-h-screen bg-neutral-100 text-neutral-900 dark:bg-neutral-900 dark:text-neutral-50">
      <div className="p-6">
        <h1 className="text-2xl font-bold">Lauv CDN Dashboard</h1>
        <p className="opacity-70">Scaffold in progress.</p>
      </div>
    </div>
  )
}

const root = createRoot(document.getElementById('root')!)
root.render(<App />)
