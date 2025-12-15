// 系统监控数据类型定义

export interface CpuInfo {
  name: string;
  cores: number;
  usage: number; // 总使用率百分比
  coreUsage: number[]; // 每个核心的使用率
  frequency: number; // 当前频率 GHz
  temperature?: number; // 温度（如果可用）
}

export interface MemoryInfo {
  total: number; // 总内存 (bytes)
  used: number; // 已使用 (bytes)
  available: number; // 可用 (bytes)
  usagePercent: number; // 使用率百分比
  swapTotal?: number; // 交换分区总大小
  swapUsed?: number; // 交换分区已使用
}

export interface NetworkInfo {
  interfaces: NetworkInterface[];
  totalDownload: number; // 总下载量
  totalUpload: number; // 总上传量
  downloadSpeed: number; // 当前下载速度 (bytes/s)
  uploadSpeed: number; // 当前上传速度 (bytes/s)
}

export interface NetworkInterface {
  name: string;
  displayName: string; // 显示名称（如 Wi-Fi、以太网）
  ipAddress?: string; // IP 地址
  isUp: boolean;
  downloadSpeed: number;
  uploadSpeed: number;
  totalDownloaded: number;
  totalUploaded: number;
}

export interface SystemStats {
  timestamp: number; // 时间戳
  cpu: CpuInfo;
  memory: MemoryInfo;
  network: NetworkInfo;
}