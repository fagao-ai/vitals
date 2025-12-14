<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Card from 'primevue/card';
import Button from 'primevue/button';
import CpuMonitor from './components/CpuMonitor.vue';
import MemoryMonitor from './components/MemoryMonitor.vue';
import NetworkMonitor from './components/NetworkMonitor.vue';
import type { SystemStats } from './types/system';

// 系统状态数据
const systemStats = ref<SystemStats | null>(null);
const lastUpdate = ref<Date>(new Date());
const isPolling = ref(false);
const error = ref<string | null>(null);
let pollingInterval: number | null = null;

// 获取系统数据
async function fetchSystemStats() {
  try {
    error.value = null;
    const stats = await invoke<SystemStats>('get_system_stats');
    console.log('Received system stats:', stats); // 调试日志
    systemStats.value = stats;
    lastUpdate.value = new Date();
  } catch (err) {
    console.error('Failed to fetch system stats:', err);
    error.value = err as string;
  }
}

// 开始/停止轮询
function togglePolling() {
  if (isPolling.value) {
    stopPolling();
  } else {
    startPolling();
  }
}

function startPolling() {
  if (pollingInterval) {
    clearInterval(pollingInterval);
  }

  isPolling.value = true;
  fetchSystemStats(); // 立即获取一次

  // 每秒更新一次
  pollingInterval = setInterval(fetchSystemStats, 1000) as unknown as number;
}

function stopPolling() {
  if (pollingInterval) {
    clearInterval(pollingInterval);
    pollingInterval = null;
  }
  isPolling.value = false;
}

// 组件挂载时开始轮询
onMounted(() => {
  startPolling();
});

// 组件卸载时停止轮询
onUnmounted(() => {
  stopPolling();
});
</script>

<template>
  <div class="min-h-screen bg-gray-100/80 dark:bg-gray-900/80 backdrop-blur-sm">
    <!-- 紧凑的顶部栏 -->
    <header class="bg-white/70 dark:bg-gray-800/70 backdrop-blur-md border-b border-gray-200/50 dark:border-gray-700/50 sticky top-0 z-10">
      <div class="max-w-6xl mx-auto px-3 py-2">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-chart-line text-blue-500"></i>
            <h1 class="text-base font-semibold text-gray-900 dark:text-white">Vitals</h1>
          </div>

          <div class="flex items-center gap-2">
            <div v-if="lastUpdate" class="text-xs text-gray-500 dark:text-gray-400 hidden sm:block">
              {{ lastUpdate.toLocaleTimeString() }}
            </div>
            <Button
              :icon="isPolling ? 'pi pi-pause' : 'pi pi-play'"
              @click="togglePolling"
              size="small"
              text
              severity="secondary"
            />
          </div>
        </div>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="max-w-6xl mx-auto p-3">
      <!-- 错误提示 -->
      <div v-if="error" class="mb-3">
        <Card class="border-red-200 dark:border-red-800 bg-red-50/80 dark:bg-red-900/20 backdrop-blur-sm">
          <template #content>
            <div class="flex items-center gap-2 text-red-700 dark:text-red-400 text-sm">
              <i class="pi pi-exclamation-triangle text-sm"></i>
              <span>{{ error }}</span>
            </div>
          </template>
        </Card>
      </div>

      <!-- 加载状态 -->
      <div v-else-if="!systemStats" class="flex items-center justify-center h-32">
        <div class="text-center">
          <i class="pi pi-spin pi-spinner text-2xl text-blue-500 mb-2"></i>
          <p class="text-sm text-gray-600 dark:text-gray-400">加载中...</p>
        </div>
      </div>

      <!-- 系统监控面板 - 紧凑布局 -->
      <div v-else class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <!-- CPU 监控 -->
        <CpuMonitor :cpu="systemStats.cpu" />

        <!-- 内存监控 -->
        <MemoryMonitor :memory="systemStats.memory" />

        <!-- 网络监控 -->
        <NetworkMonitor :network="systemStats.network" />
      </div>
    </main>
  </div>
</template>

<style>
/* PrimeVue 图标 */
@import 'primeicons/primeicons.css';

/* PrimeVue 组件基础样式 - 小组件风格 */
.p-card {
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.75rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  padding: 0.75rem;
  transition: all 0.2s ease;
}

.p-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.08);
}

.p-card-header {
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.p-progressbar {
  height: 0.5rem;
  background-color: #f3f4f6;
  border-radius: 0.25rem;
  overflow: hidden;
}

.p-progressbar-value {
  background-color: #3b82f6;
  height: 100%;
  transition: width 0.3s ease;
}

.p-button {
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 0.375rem;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.p-button:hover {
  background-color: #2563eb;
}

.p-button.p-button-success {
  background-color: #22c55e;
}

.p-button.p-button-success:hover {
  background-color: #16a34a;
}

.p-button.p-button-warning {
  background-color: #f59e0b;
}

.p-button.p-button-warning:hover {
  background-color: #d97706;
}

.p-tag {
  display: inline-flex;
  align-items: center;
  padding: 0.25rem 0.5rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 500;
}

.p-tag.p-tag-info {
  background-color: #dbeafe;
  color: #1e40af;
}

.p-tag.p-tag-success {
  background-color: #dcfce7;
  color: #166534;
}

.p-tag.p-tag-warning {
  background-color: #fef3c7;
  color: #92400e;
}

/* 全局样式 */
:root {
  --primary-50: #eff6ff;
  --primary-100: #dbeafe;
  --primary-200: #bfdbfe;
  --primary-300: #93c5fd;
  --primary-400: #60a5fa;
  --primary-500: #3b82f6;
  --primary-600: #2563eb;
  --primary-700: #1d4ed8;
  --primary-800: #1e40af;
  --primary-900: #1e3a8a;
}

/* 暗色主题支持 */
@media (prefers-color-scheme: dark) {
  .p-card {
    background-color: #1f2937;
    border-color: #374151;
  }

  .p-progressbar {
    background-color: #374151;
  }

  .p-tag.p-tag-info {
    background-color: #1e3a8a;
    color: #dbeafe;
  }
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: #f1f5f9;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

/* 动画效果 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.fade-in {
  animation: fadeIn 0.3s ease-out;
}

/* 暗色主题支持 */
@media (prefers-color-scheme: dark) {
  .p-card {
    background: rgba(31, 41, 55, 0.85);
    border-color: rgba(55, 65, 81, 0.5);
  }

  .p-progressbar {
    background-color: #374151;
  }

  .p-tag.p-tag-info {
    background-color: #1e3a8a;
    color: #dbeafe;
  }
}

/* 窗口拖动区域 */
header {
  -webkit-app-region: drag;
}

header button {
  -webkit-app-region: no-drag;
}

/* 滚动条美化 */
::-webkit-scrollbar {
  width: 4px;
  height: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.2);
}

@media (prefers-color-scheme: dark) {
  ::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
  }

  ::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.2);
  }
}
</style>