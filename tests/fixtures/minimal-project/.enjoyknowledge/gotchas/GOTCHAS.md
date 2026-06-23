---
description: 导出功能的踩坑记录
tags:
  - export
  - excel
  - backend
timestamp: 2026-06-20
---

# 导出功能踩坑清单

## 导出

### t_export_record 无 status 字段
- **影响**: 无法直接判断导出状态
- **当前方案**: 用 create_time + 其他字段组合判断

### 大数据量超时
- **影响**: 超过 10 万行时接口超时
- **当前方案**: 分批导出，单次最多 10 万行

## 性能

### 内存溢出
- **影响**: 大文件导出导致 OOM
- **当前方案**: SXSSFWorkbook 流式写入
