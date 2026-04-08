<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
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
    🚀 高性能 · 🏗️ 结构优良 · 🌐 原生 WASM <br/>
    功能完整的 osu! 数据库解析库，支持解析 osu!.db, collection.db 与 scores.db 。
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

<hr />

# 📚 目录

- [✨ 特性](#-特性)
- [📦 安装](#-安装)
- [🚀 快速开始](#-快速开始)
- [📖 使用指南](#-使用指南)
- [⚠️ 特别注意](#️-特别注意)
- [🤝 贡献指南](#-贡献指南)
- [📜 开源协议](#-开源协议)

# ✨ 特性

- **🌐 WASM 原生支持**: 编译为 WebAssembly，在浏览器中以接近原生的速度运行
- **📱 多框架支持**: 支持 Vue、React、Svelte 等流行前端框架
- **📦 完整格式支持**: osu!.db、collection.db、scores.db 所有数据库文件
- **🎨 TypeScript 友好**: 完整的类型定义，提供最佳的开发体验
- **⚡ 零配置**: 通过 npm 安装即可使用，集成简单快速
- **🚀 高性能解析**: 基于 nom8 解析器，高效可靠
- **📊 轻量级**: WASM 包体积优化，快速加载和初始化

# 📦 安装

## 步骤一：安装依赖

使用 npm、yarn 或 pnpm 安装：

```bash
# npm
npm install @osynicite/osynic-osudb

# yarn
yarn add @osynicite/osynic-osudb

# pnpm
pnpm add @osynicite/osynic-osudb
```

## 步骤二：Vite 配置

确保您的 Vite 项目配置支持 WebAssembly：

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

> 📝 如果使用 Tailwind CSS，请在 `tailwind.config.js` 中配置模板路径以支持动态类名

# 🚀 快速开始

## Node.js 环境

```javascript
import { OsuDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('osu!.db');
const osudb = OsuDB.from_bytes(new Uint8Array(data));

// 遍历前3个谱面
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

## Vue 3 示例（使用 Tailwind CSS）

```vue
<template>
  <div class="min-h-screen bg-linear-to-br from-gray-900 to-gray-800 p-6 text-white">
    <div class="max-w-6xl mx-auto space-y-6">
      <!-- Header -->
      <div class="text-center space-y-2 mb-8">
        <h1 class="text-4xl font-bold">🎵 osu! 谱面浏览器</h1>
        <p class="text-gray-400">使用 WASM 解析您的数据库文件</p>
      </div>

      <!-- Upload Section -->
      <div class="bg-gray-800 rounded-lg p-6 border border-gray-700 space-y-3">
        <label class="block">
          <span class="text-sm font-medium mb-2 inline-block">选择 osu!.db 文件</span>
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
          {{ loading ? '⏳ 解析中...' : '📂 加载数据库' }}
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
          <div class="text-gray-400 text-sm">谱面总数</div>
        </div>
        <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-center">
          <div class="text-3xl font-bold text-green-400">{{ calcTotalLength() }}</div>
          <div class="text-gray-400 text-sm">总时长</div>
        </div>
        <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-center">
          <div class="text-3xl font-bold text-purple-400">{{ calcAvgDiff() }}</div>
          <div class="text-gray-400 text-sm">平均难度</div>
        </div>
      </div>

      <!-- Beatmaps Grid -->
      <div v-if="beatmaps.length > 0" class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div 
          v-for="(beatmap, index) in beatmaps.slice(0, 20)" 
          :key="index"
          class="bg-linear-to-br from-gray-800 to-gray-900 border border-gray-700 rounded-lg p-4 hover:border-blue-500/50 transition group cursor-pointer"
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
          <p class="text-gray-400">正在解析数据库...</p>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-12 text-gray-400">
        <div class="text-5xl mb-4">📂</div>
        <p>选择并加载您的 osu!.db 文件开始</p>
      </div>

      <!-- Footer -->
      <div v-if="beatmaps.length > 0" class="text-center text-sm text-gray-500 pt-4 border-t border-gray-700">
        显示前 20 个谱面
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
    error.value = `解析失败: ${err.message}`;
    beatmaps.value = [];
  } finally {
    loading.value = false;
  }
};

const formatTime = (timestamp: number) => {
  return new Date(timestamp).toLocaleString('zh-CN');
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

# 📖 使用指南

## 解析 osu!.db（谱面数据库）

```typescript
import { OsuDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('osu!.db');
const db = OsuDB.from_bytes(new Uint8Array(data));

console.log(`总谱面数: ${db.beatmaps.length}`);
db.beatmaps.forEach(beatmap => {
  console.log(`${beatmap.artist} - ${beatmap.title} [${beatmap.difficulty_name}]`);
  console.log(`  ★${beatmap.overall_difficulty.toFixed(2)}`);
});
```

## 解析 collection.db（收藏库）

```typescript
import { CollectionDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('collection.db');
const db = CollectionDB.from_bytes(new Uint8Array(data));

db.collections.forEach(collection => {
  console.log(`📁 ${collection.name}: ${collection.beatmap_hashes.length} 个谱面`);
});
```

## 解析 scores.db（成绩数据库）

```typescript
import { ScoresDB } from '@osynicite/osynic-osudb';
import fs from 'fs';

const data = fs.readFileSync('scores.db');
const db = ScoresDB.from_bytes(new Uint8Array(data));

for (const [hash, scores] of Object.entries(db.scores)) {
  console.log(`谱面 ${hash} 有 ${scores.length} 个成绩`);
  scores.forEach((score, idx) => {
    console.log(`  ${idx + 1}. ${score.player_name}: ${score.score} 分`);
  });
}
```

# ⚠️ 特别注意

## 浏览器支持

现代浏览器均支持 WebAssembly，但请确保：

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 79+
- ✅ Node.js 8+

## 文件大小限制

处理大文件时可能遇到内存问题。建议：

- 对于超过 100MB 的文件，考虑流式处理或分块读取
- 在生产环境中，建议在 Web Worker 中处理文件解析

## 常见问题

### Q: 可以在 Node.js 中使用吗？

A: 是的，完全支持。只需确保已安装 `@osynicite/osynic-osudb` 包并正确配置构建工具。

### Q: 支持写入操作吗？

A: 目前库主要支持读取操作。写入功能暂未绑定 WASM。

### Q: 性能如何？

A: WASM 版本比纯 JavaScript 实现快 10-100 倍，取决于具体操作。对于大多数数据库文件，解析时间在 100ms 以内。

### Q: 如何处理解析错误？

A: 使用 try-catch 捕获异常：

```typescript
try {
  const db = OsuDB.from_bytes(data);
} catch (error) {
  console.error('Parse error:', error.message);
}
```

# 🤝 贡献指南

欢迎提交 PR 或 Issue！如果您发现任何问题或有改进建议，请遵循以下规则：

## 如何贡献

1. **Fork 项目** - 在 GitHub 上 fork 该项目
2. **创建分支** - `git checkout -b feature/your-feature`
3. **提交更改** - `git commit -am 'Add your feature'`
4. **推送到分支** - `git push origin feature/your-feature`
5. **提交 Pull Request** - 创建一个新的 Pull Request

## 开发指南

### 编译 WASM 库

```bash
# 从项目根目录
cd wasm

# 构建 WASM 库
wasm-pack build --release --target bundler --out-dir pkg --scope osynicite
```

### 运行测试

```bash
# 安装依赖
npm install

# 运行测试
npm run test
```

## 代码标准

- 遵循 Rust 官方编码规范
- 新增功能需附带测试用例
- 提交前运行 `cargo fmt` 和 `cargo clippy`
- 更新相关文档和示例

# ❤️ 鸣谢

本项目基于 [osu-db](https://crates.io/crates/osu-db) 库重构，进行了架构优化、性能提升和版本兼容性改进。

感谢 [osu-db](https://crates.io/crates/osu-db) 的作者们！

`osu-db` 项目基于 [Unlicense](../licenses/LICENSE-osu-db)，项目证书放置在 `licenses/` 目录。

# 📜 开源协议

本项目基于 [MIT License](../LICENSE) 开源，请尊重原作者的著作权。

---

## 相关资源

- 📚 [Rust 官方文档](https://github.com/osynicite/osynic_osudb) - 完整的 Rust 库
- 🌐 [在线文档](https://hakochest.github.io/osynic-osudb) - TypeDoc 文档
- 📦 [NPM 包](https://www.npmjs.com/package/@osynicite/osynic-osudb) - npm 发布页面
- 💬 [Discord 社区](https://discord.gg/DRnZSES3BC) - 加入我们的社区
