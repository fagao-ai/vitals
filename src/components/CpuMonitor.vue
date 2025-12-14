<template>
  <Card class="cpu-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-microchip text-blue-500"></i>
        <h3 class="text-lg font-semibold">CPU 监控</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-6">
        <!-- CPU信息概览 -->
        <div class="flex justify-between items-center">
          <span class="text-gray-600 text-sm">{{ cpu.name }}</span>
          <Tag :value="`${cpu.cores} 核心`" severity="info" />
        </div>

        <!-- 总体CPU使用率 - 使用Knob组件 -->
        <div class="flex justify-center">
          <div class="text-center">
            <Knob
              :modelValue="Math.round(cpu.usage)"
              :size="120"
              :strokeWidth="10"
              valueTemplate="{value}%"
              :valueColor="getCpuColor(cpu.usage)"
            />
            <div class="mt-2 text-sm text-gray-600">
              <span>总使用率</span>
            </div>
          </div>
        </div>

        <!-- 频率信息 -->
        <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-600">
              <i class="pi pi-clock mr-2"></i>频率
            </span>
            <Tag :value="formatFrequency(cpu.frequency)" severity="secondary" />
          </div>
        </div>

        </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { formatFrequency } from '../utils/format';
import type { CpuInfo } from '../types/system';

const props = defineProps<{
  cpu: CpuInfo;
}>();

function getCoreColor(usage: number): string {
  if (usage < 50) return '#22c55e'; // green
  if (usage < 75) return '#eab308'; // yellow
  if (usage < 90) return '#f97316'; // orange
  return '#ef4444'; // red
}

function getCpuColor(usage: number): string {
  return getCoreColor(usage);
}
</script>

<style scoped>
.cpu-monitor {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.cpu-progress {
  height: 0.5rem;
}

.core-item {
  background-color: #f9fafb;
  padding: 0.5rem;
  border-radius: 0.375rem;
}

@media (prefers-color-scheme: dark) {
  .cpu-monitor {
    background-color: #1f2937;
  }
  .core-item {
    background-color: #374151;
  }
}
</style>