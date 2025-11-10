<template>
  <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
    <div class="flex items-center gap-3 mb-6">
      <Database class="w-6 h-6 text-purple-400" />
      <h2 class="text-xl font-semibold text-white">osu!.db Analysis</h2>
    </div>

    <!-- Summary Stats -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
      <div class="bg-purple-500/10 rounded-lg p-4 border border-purple-500/20">
        <div class="flex items-center gap-2 mb-2">
          <User class="w-4 h-4 text-purple-400" />
          <span class="text-purple-300 text-sm">Player</span>
        </div>
        <div class="text-white font-medium">{{ data.player_name || 'Unknown' }}</div>
      </div>
      
      <div class="bg-blue-500/10 rounded-lg p-4 border border-blue-500/20">
        <div class="flex items-center gap-2 mb-2">
          <Hash class="w-4 h-4 text-blue-400" />
          <span class="text-blue-300 text-sm">Version</span>
        </div>
        <div class="text-white font-medium">{{ data.version }}</div>
      </div>
      
      <div class="bg-green-500/10 rounded-lg p-4 border border-green-500/20">
        <div class="flex items-center gap-2 mb-2">
          <Music class="w-4 h-4 text-green-400" />
          <span class="text-green-300 text-sm">Beatmaps</span>
        </div>
        <div class="text-white font-medium">{{ data.beatmaps?.length || 0 }}</div>
      </div>
      
      <div class="bg-orange-500/10 rounded-lg p-4 border border-orange-500/20">
        <div class="flex items-center gap-2 mb-2">
          <Folder class="w-4 h-4 text-orange-400" />
          <span class="text-orange-300 text-sm">Folders</span>
        </div>
        <div class="text-white font-medium">{{ data.folder_count || 0 }}</div>
      </div>
    </div>

    <!-- Beatmaps List -->
    <div v-if="data.beatmaps && data.beatmaps.length > 0">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold text-white flex items-center gap-2">
          <Music class="w-5 h-5 text-green-400" />
          Beatmaps ({{ data.beatmaps.length }})
        </h3>
        <button
          @click="showAllBeatmaps = !showAllBeatmaps"
          class="flex items-center gap-2 px-3 py-1 bg-white/10 hover:bg-white/20 rounded-lg border border-white/20 text-white text-sm transition-colors"
        >
          <ChevronDown :class="{ 'rotate-180': showAllBeatmaps }" class="w-4 h-4 transition-transform" />
          {{ showAllBeatmaps ? 'Show Less' : 'Show All' }}
        </button>
      </div>

      <div class="bg-black/20 rounded-lg border border-white/10 overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead class="bg-white/5 border-b border-white/10">
              <tr class="text-left">
                <th class="px-4 py-3 text-gray-300 text-sm font-medium">Title</th>
                <th class="px-4 py-3 text-gray-300 text-sm font-medium">Artist</th>
                <th class="px-4 py-3 text-gray-300 text-sm font-medium">Creator</th>
                <th class="px-4 py-3 text-gray-300 text-sm font-medium">Difficulty</th>
                <th class="px-4 py-3 text-gray-300 text-sm font-medium">Stars</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="(beatmap, index) in displayedBeatmaps"
                :key="index"
                class="border-b border-white/5 hover:bg-white/5 transition-colors"
              >
                <td class="px-4 py-3 text-white">{{ beatmap.title_unicode || 'Unknown' }}</td>
                <td class="px-4 py-3 text-gray-300">{{ beatmap.artist_unicode || 'Unknown' }}</td>
                <td class="px-4 py-3 text-gray-300">{{ beatmap.creator || 'Unknown' }}</td>
                <td class="px-4 py-3 text-gray-300">{{ beatmap.difficulty_name || 'Unknown' }}</td>
                <td class="px-4 py-3">
                  <div class="flex items-center gap-2">
                    <Star class="w-4 h-4 text-yellow-400" />
                    <span class="text-white">{{ beatmap.difficulty_name || 'N/A' }}</span>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Database, User, Hash, Music, Folder, ChevronDown, Star } from 'lucide-vue-next'
import type { OsuDB } from '@osynicite/osynic-osudb'

const props = defineProps<{
  data: OsuDB
}>()

const showAllBeatmaps = ref(false)

const displayedBeatmaps = computed(() => {
  if (!props.data.beatmaps) return []
  return showAllBeatmaps.value ? props.data.beatmaps : props.data.beatmaps.slice(0, 10)
})
</script>