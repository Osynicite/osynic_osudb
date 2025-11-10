<template>
  <div class="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
    <!-- Header -->
    <header class="bg-black/20 backdrop-blur-sm border-b border-white/10">
      <div class="container mx-auto px-4 py-6">
        <div class="flex items-center gap-3">
          <Database class="w-8 h-8 text-purple-400" />
          <div>
            <h1 class="text-2xl font-bold text-white">Osynic OsuDB Playground</h1>
            <p class="text-purple-200/80">WebAssembly-powered osu! database parser</p>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="container mx-auto px-4 py-8">
      <!-- Library Info -->
      <div class="mb-8 bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
        <div class="flex items-center gap-3 mb-4">
          <Info class="w-6 h-6 text-blue-400" />
          <h2 class="text-xl font-semibold text-white">Library Information</h2>
        </div>
        <div v-if="libraryInfo" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="bg-purple-500/10 rounded-lg p-4 border border-purple-500/20">
            <div class="text-purple-300 text-sm">Name</div>
            <div class="text-white font-medium">{{ libraryInfo.name }}</div>
          </div>
          <div class="bg-blue-500/10 rounded-lg p-4 border border-blue-500/20">
            <div class="text-blue-300 text-sm">Version</div>
            <div class="text-white font-medium">{{ libraryInfo.version }}</div>
          </div>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="wasmError" class="mb-8 bg-red-500/10 backdrop-blur-sm rounded-xl border border-red-500/20 p-6">
        <div class="flex items-center gap-3">
          <AlertCircle class="w-6 h-6 text-red-400" />
          <div>
            <h3 class="text-lg font-semibold text-red-200">WASM Loading Error</h3>
            <p class="text-red-300">{{ wasmError }}</p>
          </div>
        </div>
      </div>

      <!-- File Upload Section -->
      <div class="mb-8">
        <FileUpload 
          @file-parsed="handleFileParsed" 
          @parsing-error="handleParsingError" 
        />
      </div>

      <!-- Error Display -->
      <div v-if="error" class="mb-8 bg-red-500/10 backdrop-blur-sm rounded-xl border border-red-500/20 p-6">
        <div class="flex items-center gap-3">
          <AlertCircle class="w-6 h-6 text-red-400" />
          <div>
            <h3 class="text-lg font-semibold text-red-200">Parsing Error</h3>
            <p class="text-red-300">{{ error }}</p>
          </div>
        </div>
      </div>

      <!-- Parsed Data Display -->
      <div v-if="parsedData">
        <OsuDBViewer v-if="parsedData.type === 'osudb'" :data="parsedData.data" />
        <ScoresDBViewer v-else-if="parsedData.type === 'scoresdb'" :data="parsedData.data" />
        <CollectionDBViewer v-else-if="parsedData.type === 'collectiondb'" :data="parsedData.data" />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Database, Info, AlertCircle } from 'lucide-vue-next'
import FileUpload from './components/FileUpload.vue'
import OsuDBViewer from './components/OsuDBViewer.vue'
import ScoresDBViewer from './components/ScoresDBViewer.vue'
import CollectionDBViewer from './components/CollectionDBViewer.vue'
import { 
  getVersionConstants
} from '@osynicite/osynic-osudb'
const wasmError = ref<string>('')
const parsedData = ref<any>(null)
const error = ref<string>('')

const libraryInfo = computed(() => {
  try {
    const info = getVersionConstants()
    return {
      name: info.get('name'),
      version: info.get('version'),
      description: info.get('description')
    }
  } catch (err) {
    console.error('Failed to get library info:', err)
    return null
  }
})

const handleFileParsed = (data: any) => {
  parsedData.value = data
  error.value = ''
}

const handleParsingError = (errorMessage: string) => {
  error.value = errorMessage
  parsedData.value = null
}
</script>