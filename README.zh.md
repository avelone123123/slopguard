<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  [English](README.md) | [Русский](README.ru.md) | **简体中文** | [**Website &amp; Docs**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Join%20Chat-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="开源许可证: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="极低延迟: <5ms" src="https://img.shields.io/badge/latency-%3C5ms-2f6f4f?style=flat-square">
  <img alt="MCP 协议: 1.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="跨平台" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>防止 AI 编程智能体用未经检验的代码垃圾腐蚀您的项目。</h3>

  <p>
    <b>SlopGuard</b> 是一款基于<b>纯 Rust</b>构建的超高速 Git 预提交防火墙和 <b>Model Context Protocol (MCP)</b> 引擎，<br>
    在 AI 幻觉、未完成占位符、影子工具函数和类型退化进入 Git 历史<b>之前</b>将其拦截。
  </p>

  <code>cargo install slopguard</code>

</div>

---

## 核心痛点：AI 代码垃圾堆积效应 (Code Slop)

在使用 Cursor、Claude Code、Windsurf 或 Antigravity 进行连续多轮编程时，AI 往往会带来以下隐蔽问题：

1. **未完成占位符**：`// TODO: implement later`、`throw new Error("not implemented")` 或静默吞没异常的空 `catch (e) {}` 块。
2. **重复影子工具函数**：AI 遗忘项目中已有 `formatDate()`，并在其它文件中偷偷新建重复函数。
3. **类型安全退化**：为了通过编译器检查，AI 将领域类型粗暴替换为 `any`、`Record<string, any>`、`@ts-ignore` 或 `as unknown as any`。
4. **幽灵依赖 (Ghost Dependencies)**：AI 引入了 `package.json` 中根本未声明的第三方包。
5. **范围失控 (Scope Drift)**：修改一个按钮样式，AI 却无意识篡改了 `.env` 与关键配置文件。
6. **对话式注释污染**：代码中充斥着 `// Here is the updated code:` 等 AI 废话注释。

> **传统 Linter (ESLint、Ruff、Biome) 无法拦截此类问题**。它们仅做语法与格式检查，不理解 AI 智能体的行为退化。

---

## 核心特性

- **100% 纯 Rust**：独立原生二进制文件，启动时间低于 5ms。
- **Git Diff 极速扫描**：仅针对暂存区或工作区变动行进行 AST Token 流比对。
- **内置 MCP 协议服务**：直接集成至 Cursor 与 Claude Code，在智能体生成完代码后即刻自检。
- **一键清理**：`slopguard fix` 自动剥离 AI 冗余对话注释。
- **一秒安装 Git Hook**：`slopguard hook install` 自动配置预提交守卫。
- **代码卫生评分 (0-100)**：直观终端报告与 SVG 徽章导出。

---

## 快速开始

### 安装

```bash
cargo install slopguard
```

或从源码构建：

```bash
git clone https://github.com/avelone123123/slopguard.git
cd slopguard
cargo build --release
```

### 常用命令

```bash
# 启动交互式终端软件监控程序 (或双击 slopguard.exe)
slopguard monitor

# 扫描未提交变动
slopguard check

# 启动实时 Web 可视化遥测面板
slopguard dashboard

# 仅扫描 Git 暂存区 (适合 pre-commit)
slopguard check --staged

# 递归扫描全仓库
slopguard all

# 自动清理 AI 注释垃圾
slopguard fix

# 安装 Git 预提交钩子
slopguard hook install

# 导出 SVG 评分徽章
slopguard badge ./slopguard-badge.svg
```

---

## MCP 配置 (Cursor, Claude Code, Windsurf)

在项目的 `.cursor/mcp.json` 中加入：

```json
{
  "mcpServers": {
    "slopguard": {
      "command": "slopguard",
      "args": ["mcp"]
    }
  }
}
```

---

## 原生终端软件监控程序 (`slopguard monitor`)

SlopGuard 内置全屏**原生交互式终端 TUI 程序**，直接在控制台中运行，无需任何浏览器：

```bash
slopguard monitor
# 或无需参数直接运行:
slopguard
```

- **MCP 实时活动流**：微秒级精度直观记录 Cursor、Claude Code、Windsurf 或 Antigravity 的每一次工具调用。
- **实时洁净度进度条**：显示 `[████████████████████] 100/100` 与健康等级。
- **控制台单键交互**：
  - `[S]` 立即执行全工作区代码审计
  - `[F]` 一键自动清洗 AI 冗余注释与垃圾
  - `[R]` 强制刷新遥测记录
  - `[Q]` 退出程序

---

## 实时可视化监控面板 (`slopguard ui`)

实时观察 AI 编程助手（Cursor、Claude Code、Windsurf、Antigravity）对项目代码的操作：

```bash
slopguard dashboard
# 或
slopguard ui
```

在 `http://127.0.0.1:4545` 开启纯本地超轻量监控页面：
- **MCP 实时活动流**：微秒级精度记录 AI 工具调用与审计结果。
- **AI 洁净度评分计**：随着代码生成动态评估代码健康度与评级。
- **拦截垃圾计数器**：实时显示已拦截的未完成占位符、假数据与类型降级。
- **毫秒级延迟图表**：验证 Rust 极致响应速度（<2ms），零开发卡顿。

---

## 社区与支持

- **Discord 社区**: [discord.gg/z7C9dprye](https://discord.gg/z7C9dprye)
- **Telegram 频道**: [@SlopGuard](https://t.me/SlopGuard)

---

## 开源许可

MIT © [avelone123123](https://github.com/avelone123123)
