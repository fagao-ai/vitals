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
        <!-- 网络活动折线图 -->
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
                  :data="downloadChartData"
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
                  :data="uploadChartData"
                  :options="chartOptionsSingle"
                  class="w-full h-full"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 底部统计数据 (保持不变) -->
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
import { shallowRef, watch } from 'vue'; // 引入 shallowRef
import Card from 'primevue/card';
import Chart from 'primevue/chart';
import { formatBytes } from '../utils/format';
import type { NetworkInfo } from '../types/system';

const props = defineProps<{
  network: NetworkInfo;
}>();

// 常量定义：将样式配置提取出来，避免在 watch 中重复定义
const MAX_DATA_POINTS = 30;

const COMMON_DATASET_OPTS = {
  borderWidth: 2,
  tension: 0.3,
  fill: true,
  pointRadius: 0,
  pointHoverRadius: 4,
};

const DOWNLOAD_STYLE = {
  ...COMMON_DATASET_OPTS,
  label: '下载速度',
  borderColor: '#22c55e',
  backgroundColor: 'rgba(34, 197, 94, 0.1)',
};

const UPLOAD_STYLE = {
  ...COMMON_DATASET_OPTS,
  label: '上传速度',
  borderColor: '#3b82f6',
  backgroundColor: 'rgba(59, 130, 246, 0.1)',
};

// 1. 使用普通数组存储历史数据（不需要响应式，因为我们是替换整个 ChartData）
const downloadHistory: number[] = Array(MAX_DATA_POINTS).fill(0);
const uploadHistory: number[] = Array(MAX_DATA_POINTS).fill(0);
const timeLabels: string[] = Array(MAX_DATA_POINTS).fill('');

// 2. 使用 shallowRef 定义 chartData
// shallowRef 意味着 Vue 不会深度监听对象内部属性，只监听 .value 的替换
// 这对于 Chart.js 集成非常重要，因为 Chart.js 对象内部结构很深且有循环引用
const downloadChartData = shallowRef({
  labels: timeLabels,
  datasets: [{ ...DOWNLOAD_STYLE, data: downloadHistory }]
});

const uploadChartData = shallowRef({
  labels: timeLabels,
  datasets: [{ ...UPLOAD_STYLE, data: uploadHistory }]
});

// Chart 选项
const chartOptionsSingle = {
  responsive: true,
  maintainAspectRatio: false,
  animation: false, // 关闭动画，防止闪烁
  plugins: {
    legend: { display: false },
    tooltip: {
      callbacks: {
        label: function (context: any) {
          return context.parsed.y !== null ? formatBytes(context.parsed.y) + '/s' : '';
        }
      }
    }
  },
  scales: {
    x: {
      display: true,
      grid: { display: false },
      ticks: { maxTicksLimit: 4, font: { size: 9 } }
    },
    y: {
      display: true,
      beginAtZero: true,
      grid: { color: 'rgba(156, 163, 175, 0.1)' },
      ticks: {
        font: { size: 9 },
        callback: function (value: any) { return formatBytes(value) + '/s'; }
      }
    }
  },
  interaction: {
    mode: 'nearest',
    axis: 'x',
    intersect: false
  },
  elements: {
    point: { radius: 0 }
  }
};

watch(() => props.network, (newNetwork) => {
  const currentTime = new Date().toLocaleTimeString('zh-CN', {
    hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit'
  });

  // 更新原始数据数组
  downloadHistory.push(newNetwork.downloadSpeed);
  uploadHistory.push(newNetwork.uploadSpeed);
  timeLabels.push(currentTime);

  if (downloadHistory.length > MAX_DATA_POINTS) {
    downloadHistory.shift();
    uploadHistory.shift();
    timeLabels.shift();
  }

  // 关键修复：构造全新的对象，而不是展开旧对象 (...downloadChartData.value)
  // 这避免了复制 Chart.js 内部添加的循环引用属性

  downloadChartData.value = {
    labels: [...timeLabels], // 浅拷贝数组，确保引用变化
    datasets: [{
      ...DOWNLOAD_STYLE, // 使用静态配置
      data: [...downloadHistory] // 浅拷贝数据
    }]
  };

  uploadChartData.value = {
    labels: [...timeLabels],
    datasets: [{
      ...UPLOAD_STYLE,
      data: [...uploadHistory]
    }]
  };
});
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