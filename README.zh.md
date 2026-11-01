<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  [English](README.md) | [Русский](README.ru.md) | **简体中文** | [**Website &amp; Docs**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Join%20Chat-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="版本: v3.0.0" src="https://img.shields.io/badge/release-v3.0.0-10b981?style=flat-square">
  <img alt="测试: 202 通过" src="https://img.shields.io/badge/tests-202%20passed%20(100%25)-10b981?style=flat-square">
  <img alt="开源许可证: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="极低延迟: <5ms" src="https://img.shields.io/badge/latency-%3C0.35ms-2f6f4f?style=flat-square">
  <img alt="MCP 协议: 2.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="防御防火墙: 54" src="https://img.shields.io/badge/firewalls-54%20active-blue?style=flat-square">
  <img alt="跨平台" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>防止 AI 编程智能体用未经检验的代码垃圾腐蚀您的项目。</h3>

  <p>
    <b>SlopGuard</b> 是一款基于<b>纯 Rust</b>构建的超高速 Git 预提交防火墙和 <b>Model Context Protocol (MCP)</b> 引擎，<br>
    在 AI 幻觉、未完成占位符、影子工具函数和类型退化进入 Git 历史<b>之前</b>将其拦截。
  </p>

  <code>cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard</code>

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

- **100% 纯 Rust**：独立原生二进制文件，启动时间低于 2.5ms。
- **54 个活跃 AST 防火墙**：全面拦截幻觉包导入、未等待异步循环、弱伪随机数、多态反序列化利用与深层路径穿越。
- **Git Diff 极速扫描**：仅针对暂存区或工作区变动行进行 AST Token 流比对。
- **内置 MCP 协议服务**：直接集成至 Cursor 与 Claude Code，在智能体生成完代码后即刻自检。
- **一键清理**：`slopguard fix` 自动剥离 AI 冗余对话注释。
- **一秒安装 Git Hook**：`slopguard hook install` 自动配置预提交守卫。
- **代码卫生评分 (0-100)**：直观终端报告与 SVG 徽章导出。

---

## 快速开始

### 安装

```bash
# 通过 Cargo Git 安装最新发布版本:
cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard

# 或从本地工作区安装:
# cargo install --path .
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

---

## 配置文件 (`.slopguardrc.json`)

在项目根目录下创建 `.slopguardrc.json` 即可深度自定义防护行为：

```json
{
  "ignore": ["legacy/**", "scripts/**"],
  "rules": {
    "no-duplicate-shadow-utilities": "error",
    "no-ai-conversational-bloat": "warning",
    "no-ghost-dependencies": "error"
  },
  "maxDriftFiles": 10,
  "allowedGhostDeps": ["@types/node"]
}
```

---

## 规则与检测矩阵

| 规则 ID | 严重级别 | 拦截危害说明 |
| :--- | :--- | :--- |
| `no-hardcoded-secrets` | `ERROR` | 泄露的 OpenAI/Anthropic 密钥、AWS 凭证、RSA 私钥 |
| `no-unsafe-dynamic-eval` | `ERROR` | 危险动态求值 `eval()`、`new Function()`、`pickle.loads`、`shell=True` |
| `no-ai-todo-placeholders` | `ERROR` | 未完成的 TODO 占位桩、`throw new Error("not implemented")` |
| `no-silent-catch-pass` | `ERROR` | 空 `catch(e) {}` 块以及 Python 静默 `except: pass` |
| `no-dummy-secrets-mocks` | `ERROR` | 模拟凭证与虚拟令牌 (`sk-proj-dummy...`、`dummy_api_key`) |
| `no-mock-hallucinations` | `ERROR` | 公共假数据 API (`jsonplaceholder`、假 JWT、测试邮箱) |
| `no-hallucinated-imports` | `ERROR` | 幻觉生成的虚构库导出 (`useLocalStorage`、`createClient`) |
| `no-phantom-api-routes` | `ERROR` | 缺少后端对应端点的幽灵客户端路由调用 |
| `no-circular-dependencies` | `ERROR` | 引发运行时死锁与 TDZ 崩溃的模块循环引用 (`A -> B -> A`) |
| `no-ai-sycophancy-watermarks` | `WARN` | AI 谄媚与对话水印 (`// As an AI...`、`// Hope this helps`) |
| `no-ghost-env-vars` | `ERROR` | `.env.example` 或模板中未声明的环境变量引用 |
| `no-deep-nesting-sprawl` | `WARN` | 超过 5 层的深层嵌套金字塔流程控制 |
| `no-type-safety-degradation`| `ERROR` | 类型系统绕过：`@ts-ignore`、`@ts-nocheck`、`: any`、`as unknown as any` |
| `no-ghost-dependencies` | `ERROR` | `package.json` 中未声明就直接导入的第三方依赖 |
| `no-duplicate-shadow-utilities`| `WARN` | 重新发明项目中已有的重复辅助工具函数 |
| `no-ai-conversational-bloat`| `WARN` | 代码中的 AI 对话废话注释 (`// Here is the fix:`、`// Step 1:`) |
| `no-excessive-scope-drift` | `WARN` | 智能体擅自修改超过 12 个文件或触碰敏感配置文件 |
| `no-sql-injection-risk` | `ERROR` | 原生 SQL 字符串拼接与查询模板注入漏洞 |
| `no-floating-promises` | `ERROR` | 遗漏 await、catch 或 void 处理的悬空 Promise |
| `no-async-array-predicates` | `ERROR` | 在 `.filter()`、`.some()`、`.every()`、`.find()` 中误用异步回调的真值陷阱 |
| `no-prototype-pollution` | `ERROR` | 借助未经校验的 `__proto__` 造成原型污染攻击 |
| `no-command-injection-risk` | `ERROR` | 未经转义的系统进程执行导致的命令注入 |
| `no-path-traversal-risk` | `ERROR` | 包含 `../` 的未清洗路径拼接导致的任意文件读写 |
| `no-silent-error-swallowing` | `ERROR` | 静默吞没 Promise 拒绝与异常 |
| `no-redos-vulnerability` | `ERROR` | 灾难性回溯正则表达式 (ReDoS) 拒绝服务漏洞 |
| `no-coverage-theater` | `ERROR` | 假测试剧场：空断言、无意义测试体与虚假成功判定 |
| `no-disabled-tls` | `ERROR` | 禁用 TLS 证书校验 (`NODE_TLS_REJECT_UNAUTHORIZED='0'`, `verify=False`) |
| `no-insecure-cors` | `ERROR` | 允许凭证传输时的通配符跨域 (`Access-Control-Allow-Origin: *`) |
| `no-inlined-secrets` | `ERROR` | 前端代码中内联硬编码的敏感 API 密钥与令牌 |
| `no-slopsquat-deps` | `ERROR` | 拼写仿冒与虚构恶意的 AI 依赖抢注包 |
| `no-insecure-crypto` | `ERROR` | 安全路径中使用破损密码学哈希 (`MD5`、`SHA1`、`DES`) |
| `no-hallucinated-speculative-apis` | `ERROR` | 幻觉捏造的运行时方法与投机标准库函数 |
| `no-test-sabotage` | `ERROR` | 破坏测试行为：注释掉断言、滥用 `.skip` 或强行通过 |
| `no-async-in-sync-callbacks` | `ERROR` | 在同步事件循环回调中声明异步闭包引发未捕获拒绝 |
| `no-hardcoded-jwt-secrets` | `ERROR` | 硬编码 HMAC 密钥、`alg: none` 漏洞与弱校验令牌 |
| `no-unscoped-idor-queries` | `ERROR` | 缺乏租户/用户边界隔离的未授权 IDOR 数据库查询 |
| `no-unsafe-xml-or-ssrf` | `ERROR` | XXE 外部实体注入与未校验内部 SSRF 伪造请求 |
| `no-speculative-orm-methods` | `ERROR` | 幻觉捏造的 ORM/ODM 查询辅助方法 (Prisma, TypeORM, Mongoose) |
| `no-async-pipeline-races` | `ERROR` | 无防护的 `Promise.race()` 与流管道中止泄漏 |
| `no-phantom-cloud-sdks` | `ERROR` | 依赖清单中虚构的云厂商 SDK 投机包 |
| `no-ide-protocol-hallucinations` | `ERROR` | 虚构的 IDE 扩展 API (`vscode.window.showAIMessage`, `cursor.ai.*`) |
| `no-speculative-ffi-wasm` | `ERROR` | 虚构的 `extern "C"` 原生符号与投机 WebAssembly 宿主导入 |
| `no-unbounded-concurrency` | `WARN` | 无节制并发 (`Promise.all` 映射、`asyncio.gather`) 导致的套接字与文件句柄耗尽 |
| `no-hallucinated-package-manifest` | `ERROR` | [防火墙 49] AI 幻觉生成的虚构复合包及通配符版本依赖 (`*`, `==*`, `latest`) |
| `no-unawaited-async-in-loops` | `ERROR` | [防火墙 50] 同步循环内的 `.forEach(async)`、未保护 `.map(async)`、游离 `tokio::spawn` 与 `asyncio.create_task` |
| `no-insecure-prng-tokens` | `ERROR` | [防火墙 51] 使用非密码学 PRNG (`Math.random()`, `random.choice()`) 派生安全令牌与密钥 |
| `no-unsafe-deserialization` | `ERROR` | [防火墙 52] 多态反序列化攻击链 (`pickle`, `yaml`, `node-serialize`, `XMLDecoder`, `Marshal`) |
| `no-deep-path-traversal` | `ERROR` | [防火墙 53] 多重编码穿越 (`%252e%252e`)、Windows UNC 绕过 (`\\?\`) 与 Zip Slip 归档解压 |

---

## 性能基准测试

| 评估工具 | 冷启动耗时 | 扫描耗时 (10k 行 Diff) | 内存峰值 |
| :--- | :--- | :--- | :--- |
| **SlopGuard (Rust v3.0.0)** | **< 2.5ms** | **< 0.35ms / 1k LOC** | **< 5.8 MB** |
| Biome | ~45ms | ~30ms | ~28 MB |
| ESLint | ~620ms | ~480ms | ~140 MB |
| SonarQube Scanner | ~4,200ms | ~3,100ms | ~600 MB |

---

## 社区与支持

- **Discord 社区**: [discord.gg/z7C9dprye](https://discord.gg/z7C9dprye)
- **Telegram 频道**: [@SlopGuard](https://t.me/SlopGuard)

---

## 开源许可

MIT © [avelone123123](https://github.com/avelone123123)
