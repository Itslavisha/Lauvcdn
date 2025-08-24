@@ .. @@
 import React from 'react'
+import { motion } from 'framer-motion'
 import UploadPage from './pages/Upload'
 import DownloadPage from './pages/Download'

 function App() {
   return (
-    <div className="min-h-screen bg-neutral-100 text-neutral-900 dark:bg-neutral-900 dark:text-neutral-50">
-      <div className="p-6 space-y-6">
-        <h1 className="text-2xl font-bold">Lauv CDN Dashboard</h1>
-        <UploadPage />
-        <DownloadPage />
+    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
+      <div className="container mx-auto px-4 py-8 max-w-6xl">
+        {/* Header */}
+        <motion.div 
+          initial={{ opacity: 0, y: -20 }}
+          animate={{ opacity: 1, y: 0 }}
+          className="text-center mb-12"
+        >
+          <div className="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-r from-blue-600 to-indigo-600 rounded-2xl mb-6 shadow-lg">
+            <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
+              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
+            </svg>
+          </div>
+          <h1 className="text-4xl font-bold bg-gradient-to-r from-gray-900 to-gray-600 bg-clip-text text-transparent mb-3">
+            Lauv CDN
+          </h1>
+          <p className="text-lg text-gray-600 max-w-2xl mx-auto">
+            Decentralized file storage powered by the Internet Computer. Upload, store, and share your files securely across the network.
+          </p>
+        </motion.div>
+
+        {/* Main Content */}
+        <div className="grid lg:grid-cols-2 gap-8">
+          <motion.div
+            initial={{ opacity: 0, x: -20 }}
+            animate={{ opacity: 1, x: 0 }}
+            transition={{ delay: 0.1 }}
+          >
+            <UploadPage />
+          </motion.div>
+          <motion.div
+            initial={{ opacity: 0, x: 20 }}
+            animate={{ opacity: 1, x: 0 }}
+            transition={{ delay: 0.2 }}
+          >
+            <DownloadPage />
+          </motion.div>
+        </div>
+
+        {/* Footer */}
+        <motion.footer 
+          initial={{ opacity: 0 }}
+          animate={{ opacity: 1 }}
+          transition={{ delay: 0.3 }}
+          className="mt-16 text-center text-gray-500 text-sm"
+        >
+          <p>Built with ❤️ on the Internet Computer Protocol</p>
+        </motion.footer>
       </div>
     </div>
   )
 }