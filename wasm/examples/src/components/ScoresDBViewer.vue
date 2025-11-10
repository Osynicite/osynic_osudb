<template>
  <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
    <div class="flex items-center gap-3 mb-6">
      <BarChart3 class="w-6 h-6 text-blue-400" />
      <h2 class="text-xl font-semibold text-white">scores.db Analysis</h2>
    </div>

    <!-- Summary Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
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
          <span class="text-green-300 text-sm">Beatmaps with Scores</span>
        </div>
        <div class="text-white font-medium">{{ data.beatmaps?.length || 0 }}</div>
      </div>
      
      <div class="bg-purple-500/10 rounded-lg p-4 border border-purple-500/20">
        <div class="flex items-center gap-2 mb-2">
          <Trophy class="w-4 h-4 text-purple-400" />
          <span class="text-purple-300 text-sm">Total Scores</span>
        </div>
        <div class="text-white font-medium">{{ totalScores }}</div>
      </div>
    </div>

    <!-- Beatmaps with Scores -->
    <div v-if="data.beatmaps && data.beatmaps.length > 0">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold text-white flex items-center gap-2">
          <BarChart3 class="w-5 h-5 text-blue-400" />
          Beatmaps with Scores
        </h3>
      </div>

      <div class="space-y-4">
        <div
          v-for="(beatmap, index) in data.beatmaps.slice(0, showAllScores ? data.beatmaps.length : 5)"
          :key="index"
          class="bg-black/20 rounded-lg border border-white/10 p-4"
        >
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <Music class="w-4 h-4 text-green-400" />
              <span class="text-white font-medium">Beatmap Hash: {{ beatmap.hash?.substring(0, 8) }}...</span>
            </div>
            <div class="flex items-center gap-2 text-gray-300 text-sm">
              <Trophy class="w-4 h-4" />
              {{ beatmap.scores?.length || 0 }} scores
            </div>
          </div>

          <div v-if="beatmap.scores && beatmap.scores.length > 0" class="bg-white/5 rounded-lg overflow-hidden">
            <div class="overflow-x-auto">
              <table class="w-full">
                <thead class="bg-white/5 border-b border-white/10">
                  <tr class="text-left">
                    <th class="px-3 py-2 text-gray-300 text-sm">Score</th>
                    <th class="px-3 py-2 text-gray-300 text-sm">Combo</th>
                    <th class="px-3 py-2 text-gray-300 text-sm">Date</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="(score, scoreIndex) in beatmap.scores.slice(0, 3)"
                    :key="scoreIndex"
                    class="border-b border-white/5"
                  >
                    <td class="px-3 py-2 text-white">{{ score.score?.toLocaleString() || 'N/A' }}</td>
                    <td class="px-3 py-2 text-gray-300">{{ score.max_combo || 'N/A' }}</td>
                    <td class="px-3 py-2 text-gray-300">{{ formatDate(score.timestamp) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <div v-if="data.beatmaps.length > 5" class="mt-4 text-center">
        <button
          @click="showAllScores = !showAllScores"
          class="flex items-center gap-2 mx-auto px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg border border-white/20 text-white transition-colors"
        >
          <ChevronDown :class="{ 'rotate-180': showAllScores }" class="w-4 h-4 transition-transform" />
          {{ showAllScores ? 'Show Less' : `Show All ${data.beatmaps.length} Beatmaps` }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { BarChart3, Hash, Music, Trophy, ChevronDown } from 'lucide-vue-next'
import type { ScoresDB } from '@osynicite/osynic-osudb';

const props = defineProps<{
  data: ScoresDB
}>()

const showAllScores = ref(false)

const totalScores = computed(() => {
  if (!props.data.beatmaps) return 0
  return props.data.beatmaps.reduce((total: number, beatmap: any) => {
    return total + (beatmap.scores?.length || 0)
  }, 0)
})

const formatDate = (timestamp: any) => {
  if (!timestamp) return 'N/A'
  try {
    return new Date(timestamp).toLocaleDateString()
  } catch {
    return 'Invalid Date'
  }
}
</script>