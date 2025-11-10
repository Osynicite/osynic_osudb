<template>
  <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
    <div class="flex items-center gap-3 mb-6">
      <Upload class="w-6 h-6 text-green-400" />
      <h2 class="text-xl font-semibold text-white">Upload osu! Database File</h2>
    </div>

    <!-- Drop Zone -->
    <div
      class="relative border-2 border-dashed border-white/20 rounded-xl p-8 text-center transition-all duration-200"
      :class="{
        'border-green-400/50 bg-green-400/5': isDragging,
        'hover:border-white/40 hover:bg-white/5': !isDragging
      }"
      @dragover.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
      @drop.prevent="handleDrop"
    >
      <input
        ref="fileInput"
        type="file"
        accept=".db,.osr"
        @change="handleFileSelect"
        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
      />
      
      <div class="flex flex-col items-center gap-4">
        <div class="w-16 h-16 bg-gradient-to-br from-purple-500 to-blue-500 rounded-full flex items-center justify-center">
          <FolderOpen class="w-8 h-8 text-white" />
        </div>
        
        <div>
          <h3 class="text-lg font-medium text-white mb-2">
            Drop your file here or click to browse
          </h3>
          <p class="text-gray-300 text-sm">
            Supports: osu!.db, scores.db, collection.db, replay files (.osr)
          </p>
        </div>
        
        <div class="flex flex-wrap gap-2 justify-center">
          <div class="flex items-center gap-2 bg-purple-500/20 rounded-full px-3 py-1 border border-purple-500/30">
            <Database class="w-4 h-4 text-purple-400" />
            <span class="text-purple-200 text-sm">osu!.db</span>
          </div>
          <div class="flex items-center gap-2 bg-blue-500/20 rounded-full px-3 py-1 border border-blue-500/30">
            <BarChart3 class="w-4 h-4 text-blue-400" />
            <span class="text-blue-200 text-sm">scores.db</span>
          </div>
          <div class="flex items-center gap-2 bg-green-500/20 rounded-full px-3 py-1 border border-green-500/30">
            <FolderOpen class="w-4 h-4 text-green-400" />
            <span class="text-green-200 text-sm">collection.db</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Processing Status -->
    <div v-if="isProcessing" class="mt-6 flex items-center justify-center gap-3 text-blue-300">
      <Loader2 class="w-5 h-5 animate-spin" />
      <span>Parsing file...</span>
    </div>

    <!-- File Info -->
    <div v-if="selectedFile && !isProcessing" class="mt-6 bg-white/5 rounded-lg p-4 border border-white/10">
      <div class="flex items-center gap-3">
        <FileText class="w-5 h-5 text-gray-400" />
        <div class="flex-1">
          <div class="text-white font-medium">{{ selectedFile.name }}</div>
          <div class="text-gray-400 text-sm">{{ formatFileSize(selectedFile.size) }}</div>
        </div>
        <div class="flex items-center gap-2 text-green-400">
          <CheckCircle class="w-5 h-5" />
          <span class="text-sm">Ready to parse</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Upload, FolderOpen, Database, BarChart3, FileText, CheckCircle, Loader2 } from 'lucide-vue-next'
import { 
  OsuDB, 
  ScoresDB, 
  CollectionDB, 
} from '@osynicite/osynic-osudb'

const emit = defineEmits<{
  fileParsed: [data: any]
  parsingError: [error: string]
}>()

const isDragging = ref(false)
const isProcessing = ref(false)
const selectedFile = ref<File | null>(null)
const fileInput = ref<HTMLInputElement>()

const handleDrop = (event: DragEvent) => {
  isDragging.value = false
  const files = event.dataTransfer?.files
  if (files && files.length > 0 && files[0] instanceof File) {
    processFile(files[0])
  }
}

const handleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0 && target.files[0] instanceof File) {
    processFile(target.files[0])
  }
}

const processFile = async (file: File) => {
  selectedFile.value = file
  isProcessing.value = true

  try {
    const arrayBuffer = await file.arrayBuffer()
    const uint8Array = new Uint8Array(arrayBuffer)

    let result: any
    let fileType: string

    // Determine file type based on name and content
    const fileName = file.name.toLowerCase()
    
    if (fileName.includes('osu!.db') || fileName.endsWith('osu.db')) {
      const osuDB = OsuDB.fromUint8Array(uint8Array)
      result = osuDB.toObject()
      fileType = 'osudb'
    } else if (fileName.includes('scores.db')) {
      const scoresDB = ScoresDB.fromUint8Array(uint8Array)
      result = scoresDB.toObject()
      fileType = 'scoresdb'
    } else if (fileName.includes('collection.db')) {
      const collectionDB = CollectionDB.fromUint8Array(uint8Array)
      result = collectionDB.toObject()
      fileType = 'collectiondb'
    } else {
      throw new Error('Unsupported file type. Please upload osu!.db, scores.db, collection.db, or .osr files.')
    }

    emit('fileParsed', { type: fileType, data: result, fileName: file.name })
  } catch (error) {
    console.error('Parsing error:', error)
    emit('parsingError', error instanceof Error ? error.message : 'Unknown parsing error')
  } finally {
    isProcessing.value = false
  }
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>