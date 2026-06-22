# 安全策略

## 报告漏洞

如果你发现安全漏洞，**请不要公开提 Issue**。

请通过以下方式私下报告：

- 邮箱: security@enjoyknowledge.dev（待配置）

我们将在 48 小时内确认收到报告，并在 7 天内提供修复时间表。

## 支持版本

| 版本 | 支持状态 |
|---|---|
| 0.x (pre-release) | 安全修复 |
| 1.x | 完整支持 |

## 安全最佳实践

enjoyknowledge 是一个本地 CLI 工具，不涉及网络通信（除可选的 `--describe` LLM 调用）。主要安全关注点：

1. **config.yaml 中的密钥**: 环境配置（A11 environment）不应包含生产密钥。使用环境变量替代。
2. **LLM API 密钥**: `--describe` 功能需要的 API 密钥应从环境变量读取，不写入配置文件。
3. **文件权限**: `.enjoyknowledge/` 目录应继承项目仓库的权限控制。
