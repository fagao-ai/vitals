// 格式化字节数为可读格式
export function formatBytes(bytes?: number): string {
  if (!bytes || bytes === 0) return '0 B';

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }

  return `${size.toFixed(1)} ${units[unitIndex]}`;
}

// 格式化百分比
export function formatPercentage(value?: number): string {
  if (value === undefined || value === null) return '0%';
  return `${value.toFixed(1)}%`;
}

// 格式化频率
export function formatFrequency(gHz?: number): string {
  if (!gHz || gHz === 0) return '0 GHz';
  return `${gHz.toFixed(2)} GHz`;
}