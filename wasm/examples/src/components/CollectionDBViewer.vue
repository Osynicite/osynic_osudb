<template>
  <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
    <div class="flex items-center gap-3 mb-6">
      <FolderOpen class="w-6 h-6 text-green-400" />
      <h2 class="text-xl font-semibold text-white">collection.db Analysis</h2>
    </div>

    <!-- Summary Stats -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
      <div class="bg-green-500/10 rounded-lg p-4 border border-green-500/20">
        <div class="flex items-center gap-2 mb-2">
          <Hash class="w-4 h-4 text-green-400" />
          <span class="text-green-300 text-sm">Version</span>
        </div>
        <div class="text-white font-medium">{{ data.version }}</div>
      </div>
      
      <div class="bg-blue-500/10 rounded-lg p-4 border border-blue-500/20">
        <div class="flex items-center gap-2 mb-2">
          <FolderOpen class="w-4 h-4 text-blue-400" />
          <span class="text-blue-300 text-sm">Collections</span>
        </div>
        <div class="text-white font-medium">{{ data.collections?.length || 0 }}</div>
      </div>
    </div>

    <!-- Collections List -->
    <div v-if="data.collections && data.collections.length > 0">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2 mb-4">
        <FolderOpen class="w-5 h-5 text-green-400" />
        Collections
      </h3>

      <div class="space-y-4">
        <div
          v-for="(collection, index) in data.collections"
          :key="index"
          class="bg-black/20 rounded-lg border border-white/10 p-4"
        >
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <Folder class="w-4 h-4 text-blue-400" />
              <span class="text-white font-medium">{{ collection.name || `Collection ${index + 1}` }}</span>
            </div>
            <div class="flex items-center gap-2 text-gray-300 text-sm">
              <Music class="w-4 h-4" />
              {{ collection.beatmap_hashes?.length || 0 }} beatmaps
            </div>
          </div>

          <div v-if="collection.beatmap_hashes && collection.beatmap_hashes.length > 0">
            <button
              @click="toggleCollection(index)"
              class="flex items-center gap-2 text-blue-400 hover:text-blue-300 text-sm mb-2 transition-colors"
            >
              <ChevronDown :class="{ 'rotate-180': expandedCollections.has(index) }" class="w-4 h-4 transition-transform" />
              {{ expandedCollections.has(index) ? 'Hide' : 'Show' }} beatmap hashes
            </button>

            <div v-if="expandedCollections.has(index)" class="bg-white/5 rounded-lg p-3">
              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
                <div
                  v-for="(hash, hashIndex) in collection.beatmap_hashes.slice(0, showAllHashes.has(index) ? collection.beatmap_hashes.length : 6)"
                  :key="hashIndex"
                  class="bg-black/20 rounded px-2 py-1 font-mono text-xs text-gray-300 border border-white/10"
                >
                  {{ hash.substring(0, 8) }}...
                </div>
              </div>
              
              <div v-if="collection.beatmap_hashes.length > 6" class="mt-3 text-center">
                <button
                  @click="toggleShowAllHashes(index)"
                  class="text-blue-400 hover:text-blue-300 text-sm transition-colors"
                >
                  {{ showAllHashes.has(index) ? 'Show Less' : `Show All ${collection.beatmap_hashes.length} Hashes` }}
                </button>
              </div>
            </div>
          </div>

          <div v-else class="text-gray-400 text-sm">
            No beatmaps in this collection
          </div>
        </div>
      </div>
    </div>

    <div v-else class="text-center py-8 text-gray-400">
      <FolderOpen class="w-12 h-12 mx-auto mb-3 opacity-50" />
      <p>No collections found in this database</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FolderOpen, Hash, Folder, Music, ChevronDown } from 'lucide-vue-next'

defineProps<{
  data: any
}>()

const expandedCollections = ref(new Set<number>())
const showAllHashes = ref(new Set<number>())

const toggleCollection = (index: number) => {
  if (expandedCollections.value.has(index)) {
    expandedCollections.value.delete(index)
  } else {
    expandedCollections.value.add(index)
  }
}

const toggleShowAllHashes = (index: number) => {
  if (showAllHashes.value.has(index)) {
    showAllHashes.value.delete(index)
  } else {
    showAllHashes.value.add(index)
  }
}
</script>