<template>
  <Card class="network-monitor">
    <template #header>
      <div class="flex items-center gap-2">
        <i class="pi pi-globe text-purple-500"></i>
        <h3 class="text-lg font-semibold">网络监控</h3>
      </div>
    </template>
    <template #content>
      <div class="space-y-6">
        <!-- 网络活动折线图 - 拆分为两个 -->
        <div>
          <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
            <i class="pi pi-chart-line mr-2"></i>实时速度趋势
          </h4>
          <div class="grid grid-cols-2 gap-4">
            <!-- 下载速度图表 -->
            <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
              <div class="text-center mb-2">
                <span class="text-sm font-medium text-green-600 flex items-center justify-center">
                  <i class="pi pi-download mr-1"></i>下载速度
                </span>
              </div>
              <div class="h-32">
                <Chart
                  type="line"
                  :data="downloadChartData"
                  :options="chartOptionsSingle"
                  class="w-full h-full"
                />
              </div>
              <div class="text-center mt-2">
                <span class="text-xs font-semibold text-green-600">{{ formatBytes(network.downloadSpeed) }}/s</span>
              </div>
            </div>

            <!-- 上传速度图表 -->
            <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
              <div class="text-center mb-2">
                <span class="text-sm font-medium text-blue-600 flex items-center justify-center">
                  <i class="pi pi-upload mr-1"></i>上传速度
                </span>
              </div>
              <div class="h-32">
                <Chart
                  type="line"
                  :data="uploadChartData"
                  :options="chartOptionsSingle"
                  class="w-full h-full"
                />
              </div>
              <div class="text-center mt-2">
                <span class="text-xs font-semibold text-blue-600">{{ formatBytes(network.uploadSpeed) }}/s</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 总传输量统计 -->
        <div class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
          <h4 class="text-sm font-medium text-gray-700 mb-3 flex items-center">
            <i class="pi pi-chart-bar mr-2"></i>总传输量
          </h4>
          <div class="grid grid-cols-3 gap-3 text-center">
            <div class="bg-white dark:bg-gray-800 p-3 rounded-lg">
              <div class="text-xs text-gray-600 mb-1 flex items-center justify-center">
                <i class="pi pi-download mr-1 text-green-500"></i>下载
              </div>
              <Tag :value="formatBytes(network.totalDownload)" severity="success" class="w-full" />
            </div>
            <div class="bg-white dark:bg-gray-800 p-3 rounded-lg">
              <div class="text-xs text-gray-600 mb-1 flex items-center justify-center">
                <i class="pi pi-upload mr-1 text-blue-500"></i>上传
              </div>
              <Tag :value="formatBytes(network.totalUpload)" severity="info" class="w-full" />
            </div>
            <div class="bg-white dark:bg-gray-800 p-3 rounded-lg">
              <div class="text-xs text-gray-600 mb-1 flex items-center justify-center">
                <i class="pi pi-sync mr-1 text-purple-500"></i>总计
              </div>
              <Tag :value="formatBytes(network.totalDownload + network.totalUpload)" severity="warning" class="w-full" />
            </div>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import Card from 'primevue/card';
import Tag from 'primevue/tag';
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

// 下载速度图表数据 - 使用响应式数组
const downloadChartData = ref({
  labels: Array(maxDataPoints).fill(''),
  datasets: [
    {
      label: '下载速度',
      data: Array(maxDataPoints).fill(0),
      borderColor: '#22c55e',
      backgroundColor: 'rgba(34, 197, 94, 0.1)',
      borderWidth: 2,
      tension: 0.3,
      fill: true,
      pointRadius: 0,
      pointHoverRadius: 4,
    }
  ]
});

// 上传速度图表数据 - 使用响应式数组
const uploadChartData = ref({
  labels: Array(maxDataPoints).fill(''),
  datasets: [
    {
      label: '上传速度',
      data: Array(maxDataPoints).fill(0),
      borderColor: '#3b82f6',
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
      borderWidth: 2,
      tension: 0.3,
      fill: true,
      pointRadius: 0,
      pointHoverRadius: 4,
    }
  ]
});

// 单个图表的选项配置 - 使用静态常量避免无限循环
const chartOptionsSingle = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false
    },
    tooltip: {
      callbacks: {
        label: function(context: any) {
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
        callback: function(value: any) {
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

let intervalId: number | null = null;



// 更新历史数据
function updateHistory() {
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

  // 直接更新图表数据
  try {
    downloadChartData.value.labels.splice(0, downloadChartData.value.labels.length, ...timeLabels);
    downloadChartData.value.datasets[0].data.splice(0, downloadChartData.value.datasets[0].data.length, ...downloadHistory);

    uploadChartData.value.labels.splice(0, uploadChartData.value.labels.length, ...timeLabels);
    uploadChartData.value.datasets[0].data.splice(0, uploadChartData.value.datasets[0].data.length, ...uploadHistory);
  } catch (error) {
    console.error('Chart update error:', error);
  }

  }


onMounted(() => {
  intervalId = setInterval(updateHistory, 1000) as unknown as number;
});

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId);
  }
});
</script>

<style scoped>
.network-monitor {
  background-color: white;
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
    background-color: #1f2937;
  }
  .interface-item {
    background-color: #374151;
  }
  .interface-item:hover {
    background-color: #4b5563;
  }
}
</style>