export interface StaticMemoryInfo {
  total: number;
  totalSwap: number;
};

export interface DynamicMemoryInfo {
  used: number;
  freeMemory: number;
  usedSwap: number;
  usagePercent: number;
};