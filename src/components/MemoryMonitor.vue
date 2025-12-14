<template>
  <Card class="memory-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-database text-green-500"></i>
        <h3 class="text-lg font-semibold">内存监控</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-6">
        <!-- 总体内存使用率 - 使用Knob组件 -->
        <div class="flex justify-center">
          <div class="text-center">
            <Knob
              :modelValue="Math.round(memory.usagePercent)"
              :size="120"
              :strokeWidth="10"
              valueTemplate="{value}%"
              :valueColor="getMemoryColor(memory.usagePercent)"
            />
            <div class="mt-2 space-y-1">
              <div class="text-lg font-semibold">{{ formatBytes(memory.total) }}</div>
              <div class="text-sm text-gray-600">总内存</div>
            </div>
          </div>
        </div>

        <!-- 内存分布 - 使用横条进度条 -->
        <div class="space-y-3">
          <h4 class="text-sm font-medium text-gray-700">内存分布</h4>
          <div class="space-y-3">
            <!-- 已使用内存 -->
            <div class="bg-gray-100 dark:bg-gray-800 rounded-lg p-3">
              <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-700">已使用</span>
                <span class="text-sm font-semibold text-blue-600">{{ formatBytes(memory.used) }}</span>
              </div>
              <div class="relative">
                <div class="absolute inset-0 bg-gray-200 dark:bg-gray-700 rounded-full h-2"></div>
                <div
                  class="relative h-2 rounded-full transition-all duration-300 ease-out bg-blue-500"
                  :style="{ width: `${(memory.used / memory.total) * 100}%` }"
                ></div>
              </div>
            </div>

            <!-- 可用内存 -->
            <div class="bg-gray-100 dark:bg-gray-800 rounded-lg p-3">
              <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-gray-700">可用</span>
                <span class="text-sm font-semibold text-green-600">{{ formatBytes(memory.available) }}</span>
              </div>
              <div class="relative">
                <div class="absolute inset-0 bg-gray-200 dark:bg-gray-700 rounded-full h-2"></div>
                <div
                  class="relative h-2 rounded-full transition-all duration-300 ease-out bg-green-500"
                  :style="{ width: `${(memory.available / memory.total) * 100}%` }"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- 可用内存显示 -->
        <div class="bg-green-50 dark:bg-green-900/20 p-3 rounded-lg">
          <div class="flex justify-between items-center">
            <span class="text-green-700 dark:text-green-300 font-medium flex items-center">
              <i class="pi pi-check-circle mr-2"></i>可用内存
            </span>
            <Tag :value="formatBytes(memory.available)" severity="success" />
          </div>
        </div>

        <!-- 交换分区 -->
        <div v-if="memory.swapTotal && memory.swapTotal > 0" class="space-y-3 pt-3 border-t border-gray-200 dark:border-gray-700">
          <h4 class="text-sm font-medium text-gray-700">交换分区</h4>
          <div class="space-y-2">
            <div class="flex justify-between text-sm">
              <span class="text-gray-600">已使用</span>
              <span>{{ formatBytes(memory.swapUsed) }}</span>
            </div>
            <ProgressBar
              :value="(memory.swapUsed / memory.swapTotal) * 100"
              :showValue="false"
              class="swap-progress h-1"
            />
            <div class="flex justify-between text-sm">
              <span class="text-gray-600">总大小</span>
              <span>{{ formatBytes(memory.swapTotal) }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Card from 'primevue/card';
import ProgressBar from 'primevue/progressbar';
import Knob from 'primevue/knob';
import Tag from 'primevue/tag';
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
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.memory-progress {
  height: 0.5rem;
}

.memory-progress.warning {
  background-color: #eab308;
}

.memory-progress.danger {
  background-color: #ef4444;
}

.swap-progress {
  background-color: #9ca3af;
  height: 0.25rem;
}

@media (prefers-color-scheme: dark) {
  .memory-monitor {
    background-color: #1f2937;
  }
}
</style>