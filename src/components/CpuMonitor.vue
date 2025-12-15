<template>
  <Card class="cpu-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-microchip text-blue-500"></i>
        <h3 class="text-sm font-semibold">CPU</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-3">
        <!-- CPU 使用率 -->
        <div>
          <div class="flex justify-between items-center mb-1">
            <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">使用率</span>
            <span class="text-xs font-medium">{{ Math.round(cpu.usage) }}%</span>
          </div>
          <ProgressBar
            :value="cpu.usage"
            :showValue="false"
            class="cpu-progress"
            :style="`--progress-color: ${getCpuColor(cpu.usage)}`"
          />
        </div>

        <!-- 频率信息 -->
        <div class="flex justify-between items-center">
          <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">频率</span>
          <span class="text-xs font-medium">{{ formatFrequency(cpu.frequency) }}</span>
        </div>

        <!-- 核心数 -->
        <div class="flex justify-between items-center">
          <span class="text-xs" style="color: rgba(156, 163, 175, 1.0)">核心</span>
          <span class="text-xs font-medium">{{ cpu.cores }}核</span>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import ProgressBar from 'primevue/progressbar';
import { formatFrequency } from '../utils/format';
import type { CpuInfo } from '../types/system';

const { cpu } = defineProps<{
  cpu: CpuInfo;
}>();

function getCpuColor(usage: number): string {
  if (usage < 50) return '#22c55e'; // green
  if (usage < 75) return '#eab308'; // yellow
  if (usage < 90) return '#f97316'; // orange
  return '#ef4444'; // red
}
</script>

<style scoped>
.cpu-monitor {
  background-color: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.cpu-progress {
  height: 6px;
  background-color: #e5e7eb;
}

.cpu-progress :deep(.p-progressbar-value) {
  border-radius: 3px;
  background-color: var(--progress-color) !important;
}

@media (prefers-color-scheme: dark) {
  .cpu-monitor {
    background-color: rgba(31, 41, 55, 0.75);
  }
  .cpu-progress {
    background-color: #374151;
  }
}
</style>