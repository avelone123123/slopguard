<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  [English](README.md) | [Русский](README.ru.md) | **简体中文** | [**Website &amp; Docs**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Join%20Chat-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  [![M8ven Score](https://m8ven.ai/badge/mcp/avelone123123-slopguard-ddb87e)](https://m8ven.ai/mcp/avelone123123-slopguard-ddb87e?s=readme)
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <a href="https://github.com/avelone123123/slopguard/releases/tag/v3.5.592"><img alt="版本: v3.5.592" src="https://img.shields.io/badge/release-v3.5.592-10b981?style=flat-square"></a>
  <img alt="测试: 259 通过" src="https://img.shields.io/badge/tests-259%20passed%20(100%25)-10b981?style=flat-square">
  <img alt="开源许可证: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="极低延迟: <0.35ms" src="https://img.shields.io/badge/latency-%3C0.35ms-2f6f4f?style=flat-square">
  <img alt="MCP 协议: 2.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="防御防火墙: 658" src="https://img.shields.io/badge/firewalls-658%20active-blue?style=flat-square">
  <img alt="跨平台" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>防止 AI 编程智能体用未经检验的代码垃圾腐蚀您的项目。</h3>

  <p>
    <b>SlopGuard</b> 是一款基于<b>纯 Rust</b>构建的超高速 Git 预提交防火墙和 <b>Model Context Protocol (MCP)</b> 引擎，<br>
    在 AI 幻觉、未完成占位符、影子工具函数和类型退化进入 Git 历史<b>之前</b>将其拦截。
  </p>

  <code>cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard</code>
  <br>
  <small style="color:#94a3b8">或从本地工作区克隆安装: <code>cargo install --path .</code></small>

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
- **66 个活跃 AST 防火墙**：全面拦截幻觉包导入、未等待异步循环、生成器泄露、伪随机数生成密钥、无界通道扩散、多态反序列化利用、TOCTOU 文件竞争、幻觉加密算法与动态 SSRF 模板插值。
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
cargo install --path .
```

### 预编译发布二进制文件 (Precompiled Release Binaries)

| 目标文件 | 平台 | 格式 | SHA-256 校验哈希 |
| :--- | :--- | :--- | :--- |
| `slopguard-v3.5.592-windows-x64.exe` | Windows x86_64 | Native PE Standalone | `A6F0345EF99300C19653F6232BA134A2B71118D5B85265FA1B9A69E9F5101B06` |

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

- **5 大多功能诊断视图**：
  - `[1] 总览 (OVERVIEW)`：实时动态遥测波形示波器、MCP 活动流、代码健康仪表与微秒级延迟计数器。
  - `[2] 问题 (ISSUES)`：交互式问题检查器，支持 `↑`/`↓` 快速导航、代码片段高亮与 AI 修复建议。
  - `[3] MCP 流 (MCP STREAM)`：深度审计捕获的 MCP 工具调用 (`slopguard_scan_diff`, `slopguard_check_snippet`, `slopguard_sanitize_file`)。
  - `[4] 防火墙 (FIREWALLS)`：完整的 66 个活跃 AST 防火墙矩阵与实时拦截状态。
  - `[5] 仓库 (REPOSITORY)`：Git 诊断状态、当前分支、最新提交及工作区文件变动分析。
- **实时动态波形示波器**：多级 ASCII/ANSI 频谱波形，以 60 FPS 实时呈现 AI 智能体脉冲与响应延迟。
- **控制台单键控制**：
  - `[1..5]` 或 `[Tab]` 瞬间切换诊断视图
  - `[↑/↓]` 或 `[j/k]` 浏览问题与事件列表
  - `[S]` 立即执行全工作区代码审计
  - `[F]` 一键自动清洗 AI 冗余对话注释
  - `[W]` 开启/关闭实时文件变动监视
  - `[E]` 导出完整审计诊断报告 (`slopguard-audit-report.json`)
  - `[R]` 强制刷新遥测记录
  - `[Q]` 安全退出程序

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
- **毫秒级延迟图表**：验证 Rust 极致响应速度（<0.35ms），零开发卡顿。

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

## 战略威胁矩阵 (THREAT-01 – THREAT-16)

SlopGuard 将全部 66 个活跃 AST 防火墙映射到 16 个战略级 AI 代码退化与供应链威胁向量：

| 战略威胁 ID | 威胁描述 | 核心活跃防火墙 |
| :--- | :--- | :--- |
| `THREAT-01` | 幽灵依赖与清单仿冒包过滤 | `SLOP-009`, `SLOP-011`, `SLOP-046`, `SLOP-049` |
| `THREAT-02` | 微任务事件循环饥饿与未等待异步循环陷阱 | `SLOP-014`, `SLOP-019`, `SLOP-020`, `SLOP-050` |
| `THREAT-03` | 不安全 PRNG 熵与可预测密钥派生防御 | `SLOP-004`, `SLOP-048`, `SLOP-051` |
| `THREAT-04` | 非规范深层路径穿越与 Zip-Slip 归档越界拦截 | `SLOP-030`, `SLOP-053`, `SLOP-054` |
| `THREAT-05` | 跨语言多态反序列化 RCE 利用链防御 | `SLOP-034`, `SLOP-052`, `SLOP-056` |
| `THREAT-06` | 投机性原生 FFI ABI 符号与 WASM 宿主绑定防护 | `SLOP-018` |
| `THREAT-07` | 复合型投机命名空间仿冒与虚构清单投毒 | `SLOP-057` |
| `THREAT-08` | 异步流与生成器迭代中未捕获 break/return 句柄泄露 | `SLOP-058` |
| `THREAT-09` | 直接非 CSPRNG 截断伪随机密钥派生与弱熵拦截 | `SLOP-059` |
| `THREAT-10` | 无界通道泛滥与无背压队列内存溢出 (OOM) 防护 | `SLOP-060` |
| `THREAT-11` | 动态子进程与 Shell 字符串插值命令注入防御 | `SLOP-061` |
| `THREAT-12` | 循环内未管理事件监听器无界积累与内存泄露 | `SLOP-062` |
| `THREAT-13` | 虚构跨语言 AI SDK 封装包与供应链抢注防御 | `SLOP-063` |
| `THREAT-14` | 投机性 TOCTOU 文件竞争与并发失效风险 | `SLOP-064` |
| `THREAT-15` | 幻觉生成的虚构非标密码学算法名称拦截 | `SLOP-065` |
| `THREAT-16` | 投机性动态主机与协议模板插值 SSRF 漏洞 | `SLOP-066` |

---

## 规则与检测矩阵

| 威胁 ID | 规则名称 | 严重级别 | 分类 | 拦截危害说明 |
| :--- | :--- | :--- | :--- | :--- |
| `SLOP-001` | `no-hardcoded-secrets` | `ERROR` | 密码与认证 | 泄露的 OpenAI/Anthropic 密钥、AWS 凭证、RSA 私钥 |
| `SLOP-002` | `no-inlined-secrets` | `ERROR` | 密码与认证 | 前端代码中内联硬编码的敏感 API 密钥与私有令牌 |
| `SLOP-003` | `no-ai-todo-placeholders` | `ERROR` | 代码质量 | 未完成的 TODO 占位桩、未实现异常抛出与空桩返回 |
| `SLOP-004` | `no-insecure-crypto` | `ERROR` | 密码与认证 | 安全路径中使用破损密码学哈希 (`MD5`、`SHA1`、`DES`) |
| `SLOP-005` | `no-circular-dependencies` | `ERROR` | 异步与并发 | 引发运行时死锁与 TDZ 崩溃的模块循环引用 (`A -> B -> A`) |
| `SLOP-006` | `no-mock-hallucinations` | `ERROR` | 软件供应链 | 公共假数据 API (`jsonplaceholder`、假 JWT、测试邮箱) |
| `SLOP-007` | `no-hallucinated-imports` | `ERROR` | 软件供应链 | 幻觉生成的虚构库导出与幽灵子路径 (`useLocalStorage`、`createClient`) |
| `SLOP-008` | `no-floating-promises` | `ERROR` | 异步与并发 | 遗漏 await、catch 或 void 处理的悬空 Promise |
| `SLOP-009` | `no-ghost-dependencies` | `ERROR` | 软件供应链 | `package.json` 中未声明就直接导入的第三方依赖 |
| `SLOP-010` | `no-ai-conversational-bloat` | `WARN` | 代码质量 | 代码中的 AI 对话废话注释 (`// Here is the fix:`、`// Step 1:`) |
| `SLOP-011` | `no-slopsquat-deps` | `ERROR` | 软件供应链 | 拼写仿冒与虚构恶意的 AI 依赖抢注包 |
| `SLOP-012` | `no-ai-sycophancy-watermarks` | `WARN` | 代码质量 | AI 谄媚与模型道歉水印 (`// As an AI...`、`// Hope this helps`) |
| `SLOP-013` | `no-hallucinated-speculative-apis` | `ERROR` | 软件供应链 | 幻觉捏造的运行时方法与投机标准库函数 |
| `SLOP-014` | `no-async-in-sync-callbacks` | `ERROR` | 异步与并发 | 在同步事件循环回调中声明异步闭包引发未捕获拒绝 |
| `SLOP-015` | `no-speculative-orm-methods` | `ERROR` | 软件供应链 | 幻觉捏造的 ORM/ODM 查询辅助方法 (Prisma, TypeORM, Mongoose) |
| `SLOP-016` | `no-phantom-cloud-sdks` | `ERROR` | 软件供应链 | 依赖清单中虚构的云厂商 SDK 投机包 |
| `SLOP-017` | `no-ide-protocol-hallucinations` | `ERROR` | 软件供应链 | 虚构的 IDE 扩展 API (`vscode.window.showAIMessage`, `cursor.ai.*`) |
| `SLOP-018` | `no-speculative-ffi-wasm` | `ERROR` | 软件供应链 | 虚构的 `extern "C"` 原生符号与投机 WebAssembly 宿主导入 |
| `SLOP-019` | `no-unbounded-concurrency` | `WARN` | 异步与并发 | 无节制并发 (`Promise.all` 映射、`asyncio.gather`) 导致的套接字与句柄耗尽 |
| `SLOP-020` | `no-async-pipeline-races` | `WARN` | 异步与并发 | 无防护的 `Promise.race()` 与流管道中止泄漏 |
| `SLOP-021` | `no-async-array-predicates` | `ERROR` | 异步与并发 | 在 `.filter()`、`.some()`、`.every()`、`.find()` 中误用异步回调的真值陷阱 |
| `SLOP-022` | `no-redos-vulnerability` | `ERROR` | 异步与并发 | 灾难性回溯正则表达式 (ReDoS) 拒绝服务漏洞 |
| `SLOP-023` | `no-deep-nesting-sprawl` | `WARN` | 异步与并发 | 超过 5 层的深层嵌套金字塔流程控制 |
| `SLOP-024` | `no-hardcoded-jwt-secrets` | `ERROR` | 密码与认证 | 硬编码 HMAC 密钥、`alg: none` 漏洞与弱校验令牌 |
| `SLOP-025` | `no-disabled-tls` | `ERROR` | 密码与认证 | 禁用 TLS 证书校验 (`NODE_TLS_REJECT_UNAUTHORIZED='0'`, `verify=False`) |
| `SLOP-026` | `no-insecure-cors` | `ERROR` | 密码与认证 | 允许凭证传输时的通配符跨域 (`Access-Control-Allow-Origin: *`) |
| `SLOP-027` | `no-unscoped-idor-queries` | `ERROR` | 密码与认证 | 缺乏租户/用户边界隔离的未授权 IDOR 数据库查询 |
| `SLOP-028` | `no-dummy-secrets-mocks` | `ERROR` | 密码与认证 | 模拟凭证与虚拟令牌 (`sk-proj-dummy...`、`dummy_api_key`) |
| `SLOP-029` | `no-ghost-env-vars` | `WARN` | 密码与认证 | `.env.example` 或模板中未声明的环境变量引用 |
| `SLOP-030` | `no-path-traversal-risk` | `ERROR` | 路径与注入 | 包含 `../` 的未清洗路径拼接导致的任意文件读写 |
| `SLOP-031` | `no-sql-injection-risk` | `ERROR` | 路径与注入 | 原生 SQL 字符串拼接与查询模板注入漏洞 |
| `SLOP-032` | `no-command-injection-risk` | `ERROR` | 路径与注入 | 未经转义的系统进程执行导致的命令注入 |
| `SLOP-033` | `no-unsafe-xml-or-ssrf` | `ERROR` | 路径与注入 | XXE 外部实体注入与未校验内部 SSRF 伪造请求 |
| `SLOP-034` | `no-unsafe-dynamic-eval` | `ERROR` | 路径与注入 | 危险动态求值 `eval()`、`new Function()`、`pickle.loads`、`shell=True` |
| `SLOP-035` | `no-prototype-pollution` | `ERROR` | 路径与注入 | 借助未经校验的 `__proto__` 造成原型污染攻击 |
| `SLOP-036` | `no-phantom-api-routes` | `ERROR` | 路径与注入 | 缺少后端对应端点的幽灵客户端路由调用 |
| `SLOP-037` | `no-silent-catch-pass` | `ERROR` | 路径与注入 | 空 `catch(e) {}` 块以及 Python 静默 `except: pass` |
| `SLOP-038` | `no-silent-error-swallowing` | `ERROR` | 路径与注入 | 静默吞没 Promise 拒绝与异常 |
| `SLOP-039` | `no-test-sabotage` | `ERROR` | 代码质量 | 破坏测试行为：注释掉断言、滥用 `.skip` 或强行通过 |
| `SLOP-040` | `no-coverage-theater` | `ERROR` | 代码质量 | 假测试剧场：空断言、无意义测试体与虚假成功判定 |
| `SLOP-041` | `no-duplicate-shadow-utilities` | `WARN` | 代码质量 | 重新发明项目中已有的重复辅助工具函数 |
| `SLOP-042` | `no-type-safety-degradation` | `ERROR` | 代码质量 | 类型系统绕过：`@ts-ignore`、`@ts-nocheck`、`: any`、`as unknown as any` |
| `SLOP-043` | `no-excessive-scope-drift` | `WARN` | 代码质量 | 智能体擅自修改超过 12 个文件或触碰敏感配置文件 |
| `SLOP-044` | `no-unsafe-buffer-overflows` | `ERROR` | 代码质量 | 无界缓冲区索引与裸原始内存检查缺失 |
| `SLOP-045` | `no-dangling-native-pointers` | `ERROR` | 代码质量 | 悬空原生指针与释放后使用 (use-after-free) 风险 |
| `SLOP-046` | `no-multi-registry-slopsquat` | `ERROR` | 软件供应链 | 跨多注册表投机依赖劫持与仿冒包检测 |
| `SLOP-047` | `no-async-stream-leaks` | `WARN` | 异步与并发 | 未消费完全的异步生成器与文件描述符泄漏 |
| `SLOP-048` | `no-static-entropy-seeds` | `ERROR` | 密码与认证 | 伪随机数生成器中的静态常数硬编码种子 |
| `SLOP-049` | `no-hallucinated-package-manifest` | `ERROR` | 软件供应链 | [防火墙 49] AI 幻觉生成的虚构复合包及通配符版本依赖 (`*`, `==*`, `latest`) |
| `SLOP-050` | `no-unawaited-async-in-loops` | `ERROR` | 异步与并发 | [防火墙 50] 同步循环内的 `.forEach(async)`、未保护 `.map(async)`、游离 `tokio::spawn` 与 `asyncio.create_task` |
| `SLOP-051` | `no-insecure-prng-tokens` | `ERROR` | 密码与认证 | [防火墙 51] 使用非密码学 PRNG (`Math.random()`, `random.choice()`) 派生安全令牌与密钥 |
| `SLOP-052` | `no-unsafe-deserialization` | `ERROR` | 路径与注入 | [防火墙 52] 多态反序列化攻击链 (`pickle`, `yaml`, `node-serialize`, `XMLDecoder`, `Marshal`) |
| `SLOP-053` | `no-deep-path-traversal` | `ERROR` | 路径与注入 | [防火墙 53] 多重编码穿越 (`%252e%252e`)、Windows UNC 绕过 (`\\?\`) 与 Zip Slip 归档解压 |
| `SLOP-054` | `no-zip-slip-traversal` | `ERROR` | 路径与注入 | [防火墙 54] ZipSlip 压缩包解压路径穿越与边界隔离绕过 |
| `SLOP-055` | `no-ide-protocol-hallucinations` | `ERROR` | 软件供应链 | [防火墙 55] 虚构的 IDE 与编辑器扩展 API 协议 (`vscode.window.showAIMessage` 等) |
| `SLOP-056` | `no-polymorphic-deserialization` | `ERROR` | 路径与注入 | [防火墙 56] 跨语言多态反序列化利用链与任意代码执行漏洞 |
| `SLOP-057` | `no-slopsquat-compound-namespace` | `ERROR` | 软件供应链 | [防火墙 57] 复合型投机命名空间仿冒与虚构依赖劫持 |
| `SLOP-058` | `no-async-generator-unhandled-break` | `ERROR` | 异步与并发 | [防火墙 58] 异步流或生成器迭代中未处理的提前 break/return 引发句柄泄露 |
| `SLOP-059` | `no-insecure-direct-crypto-keygen` | `ERROR` | 密码与认证 | [防火墙 59] 直接通过弱 PRNG 或截断伪随机字节派生对称密钥 (<256位熵) |
| `SLOP-060` | `no-unbounded-channel-sprawl` | `ERROR` | 异步与并发 | [防火墙 60] 无背压控制的无界消息队列与通道泛滥导致内存耗尽 (OOM) |
| `SLOP-061` | `no-speculative-subprocess-injection` | `ERROR` | 命令与注入 | [防火墙 61] 检查动态 shell 命令插值与子进程注入风险 (CWE-78) |
| `SLOP-062` | `no-unbounded-event-listener-loop` | `WARN` | 异步与并发 | [防火墙 62] 循环内未管理事件监听器注册引发无界内存泄露 (CWE-400) |
| `SLOP-063` | `no-phantom-ai-wrappers` | `ERROR` | 软件供应链 | [防火墙 63] 幻觉生成的虚构跨语言 AI SDK 封装包仿冒 (CWE-1357) |
| `SLOP-064` | `no-speculative-file-race-toctou` | `ERROR` | 路径与注入 | [防火墙 64] 投机性 TOCTOU 文件竞争与并发条件检查-写入缺陷 (CWE-367) |
| `SLOP-065` | `no-hallucinated-crypto-algorithm` | `ERROR` | 密码与认证 | [防火墙 65] 幻觉生成的虚构加密与哈希算法名称 (`aes-256-turbo`, `sha256-fast`) (CWE-327) |
| `SLOP-066` | `no-speculative-ssrf-interpolation` | `ERROR` | 路径与注入 | [防火墙 66] 未校验的目标协议与主机动态模板插值 SSRF 漏洞 (CWE-918) |

---

## 性能基准测试

| 评估工具 | 冷启动耗时 | 扫描耗时 (10k 行 Diff) | 内存峰值 |
| :--- | :--- | :--- | :--- |
| **SlopGuard (Rust v3.5.592)** | **< 2.5ms** | **< 0.35ms / 10k LOC** | **< 5.8 MB** |
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
