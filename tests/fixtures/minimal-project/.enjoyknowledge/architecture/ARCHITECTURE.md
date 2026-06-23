---
description: IoT 水务平台架构概览
tags:
  - architecture
timestamp: 2026-06-21
---

# IoT 水务平台架构

## 模块划分

- `backend/export` — 数据导出模块
- `backend/billing` — 水费计算模块
- `frontend/admin` — 管理后台

## 技术栈

Vue3 + Java Spring Boot + MyBatis + MySQL

## 部署拓扑

Nginx → Spring Boot (3 实例) → MySQL (主从)
