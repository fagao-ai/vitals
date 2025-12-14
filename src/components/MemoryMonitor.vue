<template>
  <Card class="memory-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-database text-green-500"></i>
        <h3 class="text-sm font-semibold">内存</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-3">
        <!-- 内存使用率 -->
        <div>
          <div class="flex justify-between items-center mb-1">
            <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">使用率</span>
            <span class="text-xs font-medium">{{ Math.round(memory.usagePercent) }}%</span>
          </div>
          <ProgressBar
            :value="memory.usagePercent"
            :showValue="false"
            class="memory-progress"
            :style="`--progress-color: ${getMemoryColor(memory.usagePercent)}`"
          />
        </div>

        <!-- 已用内存 -->
        <div class="flex justify-between items-center">
          <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">已用</span>
          <span class="text-xs font-medium">{{ formatBytes(memory.used) }}</span>
        </div>

        <!-- 可用内存 -->
        <div class="flex justify-between items-center">
          <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">可用</span>
          <span class="text-xs font-medium text-green-600">{{ formatBytes(memory.available) }}</span>
        </div>

        <!-- 总内存 -->
        <div class="flex justify-between items-center">
          <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">总计</span>
          <span class="text-xs font-medium">{{ formatBytes(memory.total) }}</span>
        </div>

        <!-- 交换分区 -->
        <div v-if="memory.swapTotal && memory.swapTotal > 0" class="pt-2 border-t border-gray-200 dark:border-gray-700">
          <div class="flex justify-between items-center mb-1">
            <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">Swap</span>
            <span class="text-xs font-medium">{{ Math.round(((memory.swapUsed ?? 0) / memory.swapTotal) * 100) }}%</span>
          </div>
          <ProgressBar
            :value="((memory.swapUsed ?? 0) / memory.swapTotal) * 100"
            :showValue="false"
            class="swap-progress"
          />
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Card from 'primevue/card';
import ProgressBar from 'primevue/progressbar';
import { formatBytes } from '../utils/format';
import type { MemoryInfo } from '../types/system';

const props = defineProps<{
  memory: MemoryInfo;
}>();

function getMemoryColor(usage: number): string {
  if (usage < 50) return '#22c55e'; // green
  if (usage < 75) return '#eab308'; // yellow
  if (usage < 90) return '#f97316'; // orange
  return '#ef4444'; // red
}
</script>

<style scoped>
.memory-monitor {
  background-color: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.memory-progress {
  height: 6px;
  background-color: #e5e7eb;
}

.memory-progress :deep(.p-progressbar-value) {
  border-radius: 3px;
  background-color: var(--progress-color) !important;
}

.swap-progress {
  height: 4px;
  background-color: #e5e7eb;
}

.swap-progress :deep(.p-progressbar-value) {
  border-radius: 2px;
  background-color: #9ca3af;
}

@media (prefers-color-scheme: dark) {
  .memory-monitor {
    background-color: rgba(31, 41, 55, 0.75);
  }
  .memory-progress,
  .swap-progress {
    background-color: #374151;
  }
}
</style>