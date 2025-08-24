@@ .. @@
 import React, { useState } from 'react'
+import { motion } from 'framer-motion'
 import { getActor } from '../ic/agent'
 import storageIDL from '../ic/storage.idl'

@@ .. @@
 export default function DownloadPage() {
   const [fileId, setFileId] = useState('')
   const [status, setStatus] = useState('')
   const [isLoading, setIsLoading] = useState(false)
   const [fileInfo, setFileInfo] = useState<any>(null)

@@ .. @@
   }

   return (
-    <div className="p-6 space-y-4 bg-white dark:bg-neutral-800 rounded-lg shadow">
-      <h2 className="text-xl font-semibold">Download File</h2>
+    <div className="bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden">
+      {/* Header */}
+      <div className="bg-gradient-to-r from-emerald-600 to-teal-600 px-6 py-4">
+        <div className="flex items-center space-x-3">
+          <div className="w-8 h-8 bg-white/20 rounded-lg flex items-center justify-center">
+            <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
+            </svg>
+          </div>
+          <h2 className="text-xl font-semibold text-white">Download Files</h2>
+        </div>
+      </div>
       
-      <div className="space-y-3">
-        <input 
-          className="w-full border border-gray-300 dark:border-neutral-600 px-3 py-2 rounded bg-white dark:bg-neutral-700 disabled:opacity-50" 
-          placeholder="Enter file ID" 
-          value={fileId} 
-          onChange={e => setFileId(e.target.value)}
-          disabled={isLoading}
-        />
+      <div className="p-6 space-y-6">
+        {/* File ID Input */}
+        <div className="space-y-2">
+          <label htmlFor="file-id" className="block text-sm font-medium text-gray-700">
+            File ID
+          </label>
+          <div className="relative">
+            <input 
+              id="file-id"
+              className="w-full border border-gray-300 px-4 py-3 rounded-xl bg-white focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 disabled:opacity-50 disabled:bg-gray-50 transition-colors" 
+              placeholder="Enter your file ID to retrieve the file" 
+              value={fileId} 
+              onChange={e => setFileId(e.target.value)}
+              disabled={isLoading}
+            />
+            <div className="absolute inset-y-0 right-0 flex items-center pr-3">
+              <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a.997.997 0 01-.707.293H7a4 4 0 01-4-4V7a4 4 0 014-4z" />
+              </svg>
+            </div>
+          </div>
+        </div>
         
-        <div className="flex flex-wrap gap-2">
-          <button 
-            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed" 
+        {/* Action Buttons */}
+        <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
+          <motion.button 
+            whileHover={{ scale: 1.02 }}
+            whileTap={{ scale: 0.98 }}
+            className="flex items-center justify-center space-x-2 px-4 py-3 bg-blue-600 text-white rounded-xl hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium" 
             onClick={getFileInfo}
             disabled={!fileId || isLoading}
           >
-            Get Info
-          </button>
-          <button 
-            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed" 
+            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
+            </svg>
+            <span>Get Info</span>
+          </motion.button>
+          <motion.button 
+            whileHover={{ scale: 1.02 }}
+            whileTap={{ scale: 0.98 }}
+            className="flex items-center justify-center space-x-2 px-4 py-3 bg-emerald-600 text-white rounded-xl hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium" 
             onClick={downloadFile}
             disabled={!fileId || isLoading}
           >
-            Download
-          </button>
-          <button 
-            className="px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed" 
+            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
+            </svg>
+            <span>Download</span>
+          </motion.button>
+          <motion.button 
+            whileHover={{ scale: 1.02 }}
+            whileTap={{ scale: 0.98 }}
+            className="flex items-center justify-center space-x-2 px-4 py-3 bg-purple-600 text-white rounded-xl hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium" 
             onClick={previewChunk}
             disabled={!fileId || isLoading}
           >
-            Preview Chunk
-          </button>
+            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
+            </svg>
+            <span>Preview</span>
+          </motion.button>
         </div>
-      </div>

-      {fileInfo && (
-        <div className="p-3 bg-blue-50 dark:bg-blue-900/20 rounded text-sm">
-          <h3 className="font-semibold mb-2">File Information:</h3>
-          <ul className="space-y-1">
-            <li><strong>Size:</strong> {(Number(fileInfo.total_size) / 1024 / 1024).toFixed(2)} MB</li>
-            <li><strong>Chunks:</strong> {fileInfo.num_chunks}</li>
-            <li><strong>Chunk Size:</strong> {(fileInfo.chunk_size / 1024 / 1024).toFixed(2)} MB</li>
-            <li><strong>Created:</strong> {new Date(Number(fileInfo.created_ns) / 1_000_000).toLocaleString()}</li>
-          </ul>
-        </div>
-      )}
+        {/* File Information Card */}
+        {fileInfo && (
+          <motion.div 
+            initial={{ opacity: 0, y: 10 }}
+            animate={{ opacity: 1, y: 0 }}
+            className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-200 rounded-xl p-6"
+          >
+            <div className="flex items-center space-x-3 mb-4">
+              <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
+                <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
+                </svg>
+              </div>
+              <h3 className="text-lg font-semibold text-gray-900">File Information</h3>
+            </div>
+            <div className="grid grid-cols-2 gap-4">
+              <div className="space-y-3">
+                <div>
+                  <p className="text-sm text-gray-500 mb-1">File Size</p>
+                  <p className="text-lg font-semibold text-gray-900">
+                    {(Number(fileInfo.total_size) / 1024 / 1024).toFixed(2)} MB
+                  </p>
+                </div>
+                <div>
+                  <p className="text-sm text-gray-500 mb-1">Total Chunks</p>
+                  <p className="text-lg font-semibold text-gray-900">{fileInfo.num_chunks}</p>
+                </div>
+              </div>
+              <div className="space-y-3">
+                <div>
+                  <p className="text-sm text-gray-500 mb-1">Chunk Size</p>
+                  <p className="text-lg font-semibold text-gray-900">
+                    {(fileInfo.chunk_size / 1024 / 1024).toFixed(2)} MB
+                  </p>
+                </div>
+                <div>
+                  <p className="text-sm text-gray-500 mb-1">Created</p>
+                  <p className="text-lg font-semibold text-gray-900">
+                    {new Date(Number(fileInfo.created_ns) / 1_000_000).toLocaleDateString()}
+                  </p>
+                </div>
+              </div>
+            </div>
+          </motion.div>
+        )}

-      {status && (
-        <div className={`p-3 rounded text-sm ${
-          status.includes('✅') ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' :
-          status.includes('❌') ? 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200' :
-          'bg-gray-100 dark:bg-neutral-700'
-        }`}>
-          {status}
-        </div>
-      )}
+        {/* Status Message */}
+        {status && (
+          <motion.div 
+            initial={{ opacity: 0, y: 10 }}
+            animate={{ opacity: 1, y: 0 }}
+            className={`p-4 rounded-xl text-sm font-medium ${
+              status.includes('✅') 
+                ? 'bg-green-50 text-green-800 border border-green-200' :
+              status.includes('❌') 
+                ? 'bg-red-50 text-red-800 border border-red-200' :
+              'bg-blue-50 text-blue-800 border border-blue-200'
+            }`}
+          >
+            <div className="flex items-start space-x-2">
+              <div className="flex-shrink-0 mt-0.5">
+                {status.includes('✅') ? (
+                  <svg className="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
+                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
+                  </svg>
+                ) : status.includes('❌') ? (
+                  <svg className="w-4 h-4 text-red-600" fill="currentColor" viewBox="0 0 20 20">
+                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
+                  </svg>
+                ) : (
+                  <svg className="w-4 h-4 text-blue-600 animate-spin" fill="none" viewBox="0 0 24 24">
+                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
+                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
+                  </svg>
+                )}
+              </div>
+              <div className="flex-1">
+                {status}
+              </div>
+            </div>
+          </motion.div>
+        )}
+      </div>
     </div>
   )
 }