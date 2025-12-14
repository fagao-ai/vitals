<template>
  <Card class="network-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-globe text-purple-500"></i>
        <h3 class="text-sm font-semibold">网络</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-3">
        <!-- 网络活动折线图 - 拆分为两个 -->
        <div>
          <div class="grid grid-cols-1 gap-3">
            <!-- 下载速度图表 -->
            <div class="bg-gray-50/80 dark:bg-gray-800/50 p-2 rounded-lg">
              <div class="flex justify-between items-center mb-1">
                <span class="text-xs font-medium text-green-600 flex items-center">
                  <i class="pi pi-download mr-1"></i>下载
                </span>
                <span class="text-xs font-semibold text-green-600">{{ formatBytes(network.downloadSpeed) }}/s</span>
              </div>
              <div class="h-20">
                <Chart
                  type="line"
                  :data="chartData.download"
                  :options="chartOptionsSingle"
                  class="w-full h-full"
                />
              </div>
            </div>

            <!-- 上传速度图表 -->
            <div class="bg-gray-50/80 dark:bg-gray-800/50 p-2 rounded-lg">
              <div class="flex justify-between items-center mb-1">
                <span class="text-xs font-medium text-blue-600 flex items-center">
                  <i class="pi pi-upload mr-1"></i>上传
                </span>
                <span class="text-xs font-semibold text-blue-600">{{ formatBytes(network.uploadSpeed) }}/s</span>
              </div>
              <div class="h-20">
                <Chart
                  type="line"
                  :data="chartData.upload"
                  :options="chartOptionsSingle"
                  class="w-full h-full"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 总传输量统计 -->
        <div class="grid grid-cols-3 gap-2 text-center">
          <div class="bg-gray-50/80 dark:bg-gray-800/50 p-2 rounded">
            <div class="text-xs text-gray-600 flex items-center justify-center mb-1">
              <i class="pi pi-download mr-1 text-green-500"></i>
            </div>
            <span class="text-xs font-medium">{{ formatBytes(network.totalDownload) }}</span>
          </div>
          <div class="bg-gray-50/80 dark:bg-gray-800/50 p-2 rounded">
            <div class="text-xs text-gray-600 flex items-center justify-center mb-1">
              <i class="pi pi-upload mr-1 text-blue-500"></i>
            </div>
            <span class="text-xs font-medium">{{ formatBytes(network.totalUpload) }}</span>
          </div>
          <div class="bg-gray-50/80 dark:bg-gray-800/50 p-2 rounded">
            <div class="text-xs text-gray-600 flex items-center justify-center mb-1">
              <i class="pi pi-sync mr-1 text-purple-500"></i>
            </div>
            <span class="text-xs font-medium">{{ formatBytes(network.totalDownload + network.totalUpload) }}</span>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import Card from 'primevue/card';
import Chart from 'primevue/chart';
import { formatBytes } from '../utils/format';
import type { NetworkInfo } from '../types/system';

const props = defineProps<{
  network: NetworkInfo;
}>();

// 历史数据存储 - 保存最近30个数据点
const maxDataPoints = 30;
const downloadHistory: number[] = Array(maxDataPoints).fill(0);
const uploadHistory: number[] = Array(maxDataPoints).fill(0);
const timeLabels: string[] = Array(maxDataPoints).fill('');

const chartOptionsSingle = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false
    },
    tooltip: {
      callbacks: {
        label: function (context: any) {
          if (context.parsed.y !== null) {
            return formatBytes(context.parsed.y) + '/s';
          }
          return '';
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      grid: {
        display: false
      },
      ticks: {
        maxTicksLimit: 4,
        font: {
          size: 9
        }
      }
    },
    y: {
      display: true,
      beginAtZero: true,
      grid: {
        color: 'rgba(156, 163, 175, 0.1)'
      },
      ticks: {
        font: {
          size: 9
        },
        callback: function (value: any) {
          return formatBytes(value) + '/s';
        }
      }
    }
  },
  interaction: {
    mode: 'nearest',
    axis: 'x',
    intersect: false
  }
};

const chartData = computed(() => {
  const currentTime = new Date().toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });

  const downloadSpeed = props.network.downloadSpeed;
  const uploadSpeed = props.network.uploadSpeed;
  // 更新历史数据
  downloadHistory.push(downloadSpeed);
  uploadHistory.push(uploadSpeed);
  timeLabels.push(currentTime);

  // 保持固定长度
  if (downloadHistory.length > maxDataPoints) {
    downloadHistory.shift();
  }
  if (uploadHistory.length > maxDataPoints) {
    uploadHistory.shift();
  }
  if (timeLabels.length > maxDataPoints) {
    timeLabels.shift();
  }
  return {
    download: {
      labels: timeLabels ?? Array(maxDataPoints).fill(''),
      datasets: [
        {
          label: '下载速度',
          data: downloadHistory ?? Array(maxDataPoints).fill(0),
          borderColor: '#22c55e',
          backgroundColor: 'rgba(34, 197, 94, 0.1)',
          borderWidth: 2,
          tension: 0.3,
          fill: true,
          pointRadius: 0,
          pointHoverRadius: 4,
        }
      ]
    },
    upload: {
      labels: timeLabels ?? Array(maxDataPoints).fill(''),
      datasets: [
        {
          label: '上传速度',
          data: uploadHistory ?? Array(maxDataPoints).fill(0),
          borderColor: '#3b82f6',
          backgroundColor: 'rgba(59, 130, 246, 0.1)',
          borderWidth: 2,
          tension: 0.3,
          fill: true,
          pointRadius: 0,
          pointHoverRadius: 4,
        }
      ]
    }
  }

})
</script>

<style scoped>
.network-monitor {
  background-color: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.interface-item {
  background-color: #f9fafb;
  padding: 0.5rem;
  border-radius: 0.375rem;
}

.interface-item:hover {
  background-color: #f3f4f6;
}

@media (prefers-color-scheme: dark) {
  .network-monitor {
    background-color: rgba(31, 41, 55, 0.75);
  }

  .interface-item {
    background-color: rgba(55, 65, 81, 0.75);
  }

  .interface-item:hover {
    background-color: rgba(75, 85, 99, 0.75);
  }
}
</style>