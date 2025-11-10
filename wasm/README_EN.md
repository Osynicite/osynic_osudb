<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo escaped!"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsudb (WebAssembly)</h1>

<p align="center">
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osudb" target="_blank"><img src="https://img.shields.io/npm/v/@osynicite/osynic-osudb"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osudb" target="_blank"><img src="https://img.shields.io/npm/dm/@osynicite/osynic-osudb"/></a>
  <a href="https://hakochest.github.io/osynic-osudb/" target="_blank"><img src="https://img.shields.io/badge/Typedoc-Documentation-blue"/></a>
  <a href="https://osynic-osudb.deno.dev" target="_blank"><img src="https://img.shields.io/badge/Deno-white?logo=deno&logoColor=black"/></a>
  <a href="https://github.com/osynicite/osynic_osudb" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
</p>

<p align="center">
    🚀 High Performance · 🏗️ Well-Structured · 🌐 Native WASM <br/>
    A feature-complete osu! database parsing library that supports parsing osu!.db, collection.db, and scores.db.
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

<hr />

# 📚 Table of Contents

- [✨ Features](#-features)
- [📦 Installation](#-installation)
- [🚀 Quick Start](#-quick-start)
- [📖 Usage Guide](#-usage-guide)
- [⚠️ Important Notes](#️-important-notes)
- [🤝 Contributing Guide](#-contributing-guide)
- [📜 License](#-license)

# ✨ Features

- **🌐 Native WASM Support**: Compiled to WebAssembly, runs at near-native speed in browsers
- **📱 Multi-Framework Support**: Compatible with popular front-end frameworks like Vue, React, and Svelte
- **📦 Complete Format Support**: All database files - osu!.db, collection.db, scores.db
- **🎨 TypeScript Friendly**: Full type definitions with excellent development experience
- **⚡ Zero Configuration**: Install via npm and use immediately, simple and fast integration
- **🚀 High-Performance Parsing**: Based on nom8 parser, efficient and reliable
- **📊 Lightweight**: Optimized WASM package size for fast loading and initialization

# 📦 Installation

## Step 1: Install Dependencies

Install using npm, yarn, or pnpm:

```bash
# npm
npm install @osynicite/osynic-osudb

# yarn
yarn add @osynicite/osynic-osudb

# pnpm
pnpm add @osynicite/osynic-osudb
```

## Step 2: Vite Configuration

Ensure your Vite project is configured to support WebAssembly:

```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    vue(),
  ],
  optimizeDeps: {
    exclude: ['@osynicite/osynic-osudb'],
  },
})
```

> 📝 If using Tailwind CSS, configure the template path in `tailwind.config.js` to support dynamic class names

# 🚀 Quick Start

## Node.js Environment

```javascript
import { OsuDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('osu!.db');
const osudb = OsuDB.from_bytes(new Uint8Array(data));

// Iterate through the first 3 beatmaps
for (let i = 0; i < Math.min(3, osudb.beatmaps.length); i++) {
    const beatmap = osudb.beatmaps[i];
    console.log(`Song ${i + 1}: ${beatmap.artist_unicode || beatmap.artist} - ${beatmap.title_unicode || beatmap.title}`);
    console.log(`Creator: ${beatmap.creator}`);
    console.log(`Difficulty: ${beatmap.difficulty_name}`);
    console.log(`AR: ${beatmap.approach_rate.toFixed(2)}`);
    console.log(`CS: ${beatmap.circle_size.toFixed(2)}`);
    console.log(`OD: ${beatmap.overall_difficulty.toFixed(2)}`);
    console.log('---------------------------------');
}
```

## Vue 3 Example (with Tailwind CSS)

```vue
<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 p-6 text-white">
    <div class="max-w-6xl mx-auto space-y-6">
      <!-- Header -->
      <div class="text-center space-y-2 mb-8">
        <h1 class="text-4xl font-bold">🎵 osu! Beatmap Browser</h1>
        <p class="text-gray-400">Parse your database files with WASM</p>
      </div>

      <!-- Upload Section -->
      <div class="bg-gray-800 rounded-lg p-6 border border-gray-700 space-y-3">
        <label class="block">
          <span class="text-sm font-medium mb-2 inline-block">Select osu!.db file</span>
          <input 
            type="file" 
            @change="handleFileUpload" 
            accept=".db"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded hover:border-blue-500 transition file:mr-4 file:py-2 file:px-4 file:rounded file:border-0 file:text-sm file:font-semibold file:bg-blue-600 file:text-white hover:file:bg-blue-700"
          />
        </label>
        <button 
          @click="loadDatabase" 
          :disabled="!fileData || loading"
          class="w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded-lg font-semibold transition duration-200"
        >
          {{ loading ? '⏳ Parsing...' : '📂 Load Database' }}
        </button>
      </div>

      <!-- Error Message -->
      <div v-if="error" class="bg-red-900/30 border border-red-500/50 rounded-lg p-4 text-red-200">
        ❌ {{ error }}
      </div>

      <!-- Stats Summary -->
      <div v-if="beatmaps.length > 0" class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-center">
          <div class="text-3xl font-bold text-blue-400">{{ beatmaps.length }}</div>
          <div class="text-gray-400 text-sm">Total Beatmaps</div>
        </div>
        <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-center">
          <div class="text-3xl font-bold text-green-400">{{ calcTotalLength() }}</div>
          <div class="text-gray-400 text-sm">Total Duration</div>
        </div>
        <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-center">
          <div class="text-3xl font-bold text-purple-400">{{ calcAvgDiff() }}</div>
          <div class="text-gray-400 text-sm">Average Difficulty</div>
        </div>
      </div>

      <!-- Beatmaps Grid -->
      <div v-if="beatmaps.length > 0" class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div 
          v-for="(beatmap, index) in beatmaps.slice(0, 20)" 
          :key="index"
          class="bg-gradient-to-br from-gray-800 to-gray-900 border border-gray-700 rounded-lg p-4 hover:border-blue-500/50 transition group cursor-pointer"
        >
          <!-- Title -->
          <h3 class="text-lg font-bold text-blue-300 group-hover:text-blue-200 transition truncate">
            {{ beatmap.artist_unicode || beatmap.artist }} - {{ beatmap.title_unicode || beatmap.title }}
          </h3>
          <p class="text-sm text-gray-400 truncate">[{{ beatmap.difficulty_name }}]</p>

          <!-- Creator & File -->
          <div class="mt-3 space-y-1 text-sm text-gray-300">
            <p>👤 {{ beatmap.creator || 'Unknown' }}</p>
            <p class="truncate">📁 {{ beatmap.file_name }}</p>
            <p>🔐 {{ beatmap.hash?.substring(0, 12) }}...</p>
          </div>

          <!-- Objects -->
          <div class="mt-3 flex gap-2 text-xs font-medium">
            <span class="bg-red-500/20 text-red-300 px-2 py-1 rounded">🔴 {{ beatmap.hitcircle_count }}</span>
            <span class="bg-orange-500/20 text-orange-300 px-2 py-1 rounded">↔️ {{ beatmap.slider_count }}</span>
            <span class="bg-yellow-500/20 text-yellow-300 px-2 py-1 rounded">⚙️ {{ beatmap.spinner_count }}</span>
          </div>

          <!-- Difficulty Stats -->
          <div class="mt-3 grid grid-cols-4 gap-1 text-xs">
            <div class="bg-gray-700/50 rounded p-2 text-center">
              <div class="font-bold text-cyan-300">{{ beatmap.approach_rate.toFixed(1) }}</div>
              <div class="text-gray-400">AR</div>
            </div>
            <div class="bg-gray-700/50 rounded p-2 text-center">
              <div class="font-bold text-cyan-300">{{ beatmap.circle_size.toFixed(1) }}</div>
              <div class="text-gray-400">CS</div>
            </div>
            <div class="bg-gray-700/50 rounded p-2 text-center">
              <div class="font-bold text-cyan-300">{{ beatmap.hp_drain.toFixed(1) }}</div>
              <div class="text-gray-400">HP</div>
            </div>
            <div class="bg-gray-700/50 rounded p-2 text-center">
              <div class="font-bold text-cyan-300">{{ beatmap.overall_difficulty.toFixed(1) }}</div>
              <div class="text-gray-400">OD</div>
            </div>
          </div>

          <!-- Footer -->
          <p class="mt-3 text-xs text-gray-500">⏱️ {{ formatTime(beatmap.last_modified) }}</p>
        </div>
      </div>

      <!-- Loading State -->
      <div v-else-if="loading" class="text-center py-12">
        <div class="inline-block">
          <div class="animate-spin text-3xl mb-2">⌛</div>
          <p class="text-gray-400">Parsing database...</p>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-12 text-gray-400">
        <div class="text-5xl mb-4">📂</div>
        <p>Select and load your osu!.db file to start</p>
      </div>

      <!-- Footer -->
      <div v-if="beatmaps.length > 0" class="text-center text-sm text-gray-500 pt-4 border-t border-gray-700">
        Showing first 20 beatmaps
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { OsuDB } from '@osynicite/osynic-osudb';

const fileData = ref<Uint8Array | null>(null);
const beatmaps = ref<any[]>([]);
const loading = ref(false);
const error = ref('');

const handleFileUpload = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      fileData.value = new Uint8Array(e.target?.result as ArrayBuffer);
      error.value = '';
    };
    reader.readAsArrayBuffer(file);
  }
};

const loadDatabase = () => {
  if (!fileData.value) return;
  
  loading.value = true;
  error.value = '';
  try {
    const osudb = OsuDB.from_bytes(fileData.value);
    beatmaps.value = osudb.beatmaps || [];
  } catch (err: any) {
    error.value = `Parse failed: ${err.message}`;
    beatmaps.value = [];
  } finally {
    loading.value = false;
  }
};

const formatTime = (timestamp: number) => {
  return new Date(timestamp).toLocaleString('en-US');
};

const calcTotalLength = () => {
  const seconds = beatmaps.value.reduce((sum, b) => sum + parseInt(b.total_length || 0), 0);
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${mins}m`;
};

const calcAvgDiff = () => {
  if (!beatmaps.value.length) return '0';
  const avg = beatmaps.value.reduce((sum, b) => sum + parseFloat(b.overall_difficulty || 0), 0) / beatmaps.value.length;
  return avg.toFixed(2);
};
</script>
```

# 📖 Usage Guide

## Parsing osu!.db (Beatmap Database)

```typescript
import { OsuDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('osu!.db');
const db = OsuDB.from_bytes(new Uint8Array(data));

console.log(`Total beatmaps: ${db.beatmaps.length}`);
db.beatmaps.forEach(beatmap => {
  console.log(`${beatmap.artist} - ${beatmap.title} [${beatmap.difficulty_name}]`);
  console.log(`  ★${beatmap.overall_difficulty.toFixed(2)}`);
});
```

## Parsing collection.db (Collection Database)

```typescript
import { CollectionDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('collection.db');
const db = CollectionDB.from_bytes(new Uint8Array(data));

db.collections.forEach(collection => {
  console.log(`📁 ${collection.name}: ${collection.beatmap_hashes.length} beatmaps`);
});
```

## Parsing scores.db (Scores Database)

```typescript
import { ScoresDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('scores.db');
const db = ScoresDB.from_bytes(new Uint8Array(data));

for (const [hash, scores] of Object.entries(db.scores)) {
  console.log(`Beatmap ${hash} has ${scores.length} scores`);
  scores.forEach((score, idx) => {
    console.log(`  ${idx + 1}. ${score.player_name}: ${score.score} points`);
  });
}
```

# ⚠️ Important Notes

## Browser Support

Modern browsers support WebAssembly, but please ensure:

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 79+
- ✅ Node.js 8+

## File Size Limitations

When processing large files, you may encounter memory issues. Recommendations:

- For files exceeding 100MB, consider streaming or chunked reading
- In production environments, it's recommended to handle file parsing in a Web Worker

## Frequently Asked Questions

### Q: Can I use this in Node.js?

A: Yes, it's fully supported. Just make sure you've installed the `@osynicite/osynic-osudb` package and configured your build tool correctly.

### Q: Does it support write operations?

A: Currently the library primarily supports read operations. Write functionality is not yet bound to WASM.

### Q: How's the performance?

A: The WASM version is 10-100x faster than pure JavaScript implementations, depending on the operation. For most database files, parsing takes less than 100ms.

### Q: How do I handle parse errors?

A: Use try-catch to capture exceptions:

```typescript
try {
  const db = OsuDB.from_bytes(data);
} catch (error) {
  console.error('Parse error:', error.message);
}
```

# 🤝 Contributing Guide

We welcome PRs and Issues! If you find any issues or have improvement suggestions, please follow these guidelines:

## How to Contribute

1. **Fork the project** - Fork this project on GitHub
2. **Create a branch** - `git checkout -b feature/your-feature`
3. **Commit changes** - `git commit -am 'Add your feature'`
4. **Push to branch** - `git push origin feature/your-feature`
5. **Submit Pull Request** - Create a new Pull Request

## Development Guide

### Building the WASM Library

```bash
# From the project root directory
cd wasm

# Build the WASM library
wasm-pack build --release --target bundler --out-dir pkg --scope osynicite
```

### Running Tests

```bash
# Install dependencies
npm install

# Run tests
npm run test
```

## Code Standards

- Follow official Rust coding conventions
- New features should include test cases
- Run `cargo fmt` and `cargo clippy` before committing
- Update relevant documentation and examples

# ❤️ Credits

This project is a reconstruction based on the [osu-db](https://crates.io/crates/osu-db) library, with architectural optimization, performance improvements, and version compatibility enhancements.

Thanks to the authors of [osu-db](https://crates.io/crates/osu-db)!

The `osu-db` project is licensed under [Unlicense](../licenses/LICENSE-osu-db), with the license certificate located in the `licenses/` directory.

# 📜 License

This project is open-sourced under the [MIT License](../LICENSE). Please respect the original author's copyright.

---

## Related Resources

- 📚 [Official Rust Documentation](https://github.com/osynicite/osynic_osudb) - Complete Rust library
- 🌐 [Online Documentation](https://hakochest.github.io/osynic-osudb) - TypeDoc documentation
- 📦 [NPM Package](https://www.npmjs.com/package/@osynicite/osynic-osudb) - NPM publication page
- 💬 [Discord Community](https://discord.gg/DRnZSES3BC) - Join our community
