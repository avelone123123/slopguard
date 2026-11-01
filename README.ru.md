<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  [English](README.md) | **Русский** | [简体中文](README.zh.md) | [**Сайт и Документация**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Присоединиться-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="Релиз: v2.9.0" src="https://img.shields.io/badge/release-v2.9.0-10b981?style=flat-square">
  <img alt="Тесты: 177 пройдены" src="https://img.shields.io/badge/tests-177%20passed%20(100%25)-10b981?style=flat-square">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="Latency: <3ms" src="https://img.shields.io/badge/latency-%3C0.35ms-2f6f4f?style=flat-square">
  <img alt="MCP Protocol: 2.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="Фаерволы: 53" src="https://img.shields.io/badge/firewalls-53%20active-blue?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>Защитите кодовую базу от накопления скрытого ИИ-мусора и галлюцинаций.</h3>

  <p>
    <b>SlopGuard</b> — сверхбыстрый фаервол на <b>чистом Rust</b> и <b>MCP-сервер</b> (Model Context Protocol),<br>
    который перехватывает недописанные заглушки, дубликаты утилит, обходы типов и разговоры ИИ в комментариях<br>
    <b>до того</b>, как они попадут в ваш git-репозиторий.
  </p>

  <code>cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard</code>

</div>

---

## Проблема: Накопление скрытого ИИ-мусора (Code Slop)

При активном вайбкодинге в Cursor, Claude Code, Windsurf или Antigravity нейросетевые агенты неизбежно засоряют кодовую базу:

1. **Недописанные заглушки**: `// TODO: implement later`, `throw new Error("not implemented")` или пустые `catch (e) {}`, тихо проглатывающие критические ошибки.
2. **Размножение дубликатов**: Агент забывает, что у вас уже есть функция `formatDate()` в `src/utils/date.ts`, и создает копию под другим именем в новом файле.
3. **Деградация типизации**: Чтобы компилятор не ругался, модель заменяет строгие типы на `any`, `Record<string, any>`, `@ts-ignore` или костыли `as unknown as any`.
4. **Фантомные зависимости**: Агент импортирует библиотеки (`axios`, `lodash`), которых нет в вашем `package.json`.
5. **Дрейф скоупа (Agent Drift)**: Вы просите поменять цвет кнопки, а агент попутно трогает конфиги, `.env` и схемы данных.
6. **Разговорный мусор в комментариях**: Засорение кода репликами вида `// Вот исправленный код:`, `// Шаг 1: Добавляем слушатель`.

> **Классические линтеры (ESLint, Ruff, Biome) не ловят такие проблемы.** Они проверяют синтаксис, но не понимают семантической деградации кода ИИ.

---

## Возможности

<p align="center">
  <img src="assets/interception-flow.svg" alt="Архитектура перехвата галлюцинаций SlopGuard" width="100%">
</p>

- **100% Native Rust**: Единый автономный бинарник без лишних рантаймов. Мгновенный запуск (<5мс).
- **53 Активных AST-Фаервола**: Комплексная защита от фантомных манифестов, асинхронных ловушек, криптографической энтропии, десериализации и Zip Slip обходов.
- **Интерактивный TUI-Терминал с Осциллографом**: Визуализация пульса MCP в реальном времени на 60 FPS.
- **Анализ Git Diff**: Сканирует только измененные строки в рабочей копии или в индексе git.
- **Встроенный MCP-сервер**: Подключается к Cursor, Claude Code и Windsurf, позволяя агенту проверять свой код перед завершением шага.
- **Автоматическая очистка**: Команда `slopguard fix` удаляет разговорные комментарии ИИ в один клик.
- **Pre-commit хук за 1 секунду**: Установка через `slopguard hook install`.
- **Индекс чистоты кода (AI Hygiene Score 0–100)**: Наглядный отчет в терминале и экспорт SVG-бейжей.

<p align="center">
  <img src="assets/terminal-evidence.svg" alt="Терминальная консоль SlopGuard с осциллографом" width="100%">
</p>

---

## Быстрый старт

### Установка

```bash
# Установка актуального релиза напрямую из GitHub:
cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard

# Либо локальная установка:
# cargo install --path .
```

Либо сборка из исходников:

```bash
git clone https://github.com/avelone123123/slopguard.git
cd slopguard
cargo build --release
```

### Основные команды

```bash
# Запуск интерактивной терминальной программы монитора (или двойной клик по slopguard.exe)
slopguard monitor

# Проверка текущих незакоммиченных изменений
slopguard check

# Запуск визуального дашборда мониторинга MCP в реальном времени
slopguard dashboard

# Проверка только подготовленных файлов (для pre-commit)
slopguard check --staged

# Режим наблюдения в реальном времени при вайбкодинге
slopguard watch

# Создание файла конфигурации .slopguardrc.json
slopguard init

# Вывод в формате JSON для тулинга и скриптов
slopguard check --json

# Вывод в формате аннотаций GitHub Actions
slopguard check --github

# Генерация пайплайна GitHub Actions CI за 1 секунду
slopguard ci

# Полное сканирование всех файлов проекта
slopguard all

# Автоматическое удаление разговорных комментариев ИИ
slopguard fix

# Установка git-хука в .git/hooks/pre-commit
slopguard hook install

# Генерация SVG-бейжа чистоты кодовой базы
slopguard badge ./slopguard-badge.svg
```

---

## Настройка MCP (Cursor, Claude Code, Windsurf)

В SlopGuard встроен официальный STDIO-сервер **Model Context Protocol (MCP)** прямо в бинарник.

### Настройка в Cursor

Добавьте в ваш `mcp.json` (`~/.cursor/mcp.json` или `.cursor/mcp.json` проекта):

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

### Доступные MCP-инструменты

- `slopguard_scan_diff`: Позволяет агенту проверить текущий git diff на заглушки и обходы типов.
- `slopguard_check_snippet`: Проверяет фрагмент кода в памяти перед записью на диск.
- `slopguard_sanitize_file`: Очищает целевой файл от разговорных артефактов ИИ.

---

## Терминальная программа-монитор (`slopguard monitor`)

В SlopGuard встроена полноэкранная **интерактивная терминальная программа** (TUI), работающая прямо в консоли без браузеров и внешних зависимостей. Для запуска достаточно просто дважды кликнуть по `slopguard.exe` в проводнике Windows или выполнить:

```bash
slopguard monitor
# или без аргументов:
slopguard
```

- **5 многофункциональных диагностических экранов**:
  - `[1] ОБЗОР`: живой движущийся осциллограф телеметрии (волны задержки в реальном времени), лента активности ИИ, шкала чистоты и счетчики.
  - `[2] ПРОБЛЕМЫ`: инспектор найденных деградаций со сниппетами и рекомендациями ИИ-файрвола. Навигация клавишами `↑`/`↓`.
  - `[3] ПОТОК MCP`: детальный лог всех вызовов (`slopguard_scan_diff`, `slopguard_check_snippet`, `slopguard_sanitize_file`).
  - `[4] ФАЙРВОЛЫ`: полная матрица из 53 активных AST-файрволов и статус защиты в реальном времени.
  - `[5] РЕПОЗИТОРИЙ`: ветка, последний коммит и измененные файлы рабочей зоны Git.
- **Живой анимированный осциллограф**: движущаяся волна телеметрии в реальном времени, визуализирующая сетевой пульс и всплески активности агентов.
- **Управление одной клавишей**:
  - `[1..5]` или `[Tab]` Мгновенное переключение экранов
  - `[↑/↓]` или `[j/k]` Навигация по спискам проблем и событий
  - `[S]` Полный аудит рабочего дерева
  - `[F]` Очистка кода от разговорных комментариев ИИ
  - `[W]` Включение режима живого слежения за файлами
  - `[E]` Экспорт отчета аудита в файл (`slopguard-audit-report.json`)
  - `[R]` Обновление состояния
  - `[Q]` Мгновенный выход

---

## Визуальный дашборд наблюдателя (`slopguard ui`)

Наблюдайте в реальном времени, что делает ИИ-ассистент (Cursor, Claude Code, Windsurf, Antigravity) с вашим кодом:

```bash
slopguard dashboard
# или
slopguard ui
```

Запускает быстрый встроенный веб-интерфейс на `http://127.0.0.1:4545`:
- **Лента вызовов MCP**: отображение каждого запроса инструмента с микросекундной точностью.
- **Индикатор чистоты кода**: динамический скоринг и рейтинг по мере генерации кода.
- **Счетчик заблокированного слопа**: число перехваченных заглушек, фейковых токенов и сбросов типов.
- **График задержки (Latency)**: доказательство скорости Rust (<2ms) без замедления процесса разработки.

---

---

## Конфигурация (`.slopguardrc.json`)

Настройте параметры аудита, создав файл `.slopguardrc.json` в корне вашего проекта:

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

## Правила и детекторы

| Идентификатор правила | Важность | Что перехватывает |
| :--- | :--- | :--- |
| `no-hardcoded-secrets` | `ERROR` | Утечки ключей OpenAI/Anthropic, учетных данных AWS, приватных RSA ключей |
| `no-unsafe-dynamic-eval` | `ERROR` | Опасные вызовы `eval()`, `new Function()`, `pickle.loads`, `shell=True` |
| `no-ai-todo-placeholders` | `ERROR` | Недописанные заглушки TODO, `throw new Error("not implemented")` |
| `no-silent-catch-pass` | `ERROR` | Пустые блоки `catch(e) {}` и Python `except: pass` |
| `no-dummy-secrets-mocks` | `ERROR` | Мок-токены (`sk-proj-dummy...`, `dummy_api_key`) |
| `no-mock-hallucinations` | `ERROR` | Публичные мок-API (`jsonplaceholder`, фейковые JWT, тестовые email) |
| `no-hallucinated-imports` | `ERROR` | Галлюцинированные экспорты библиотек (`useLocalStorage`, `createClient`) |
| `no-phantom-api-routes` | `ERROR` | Вызовы клиентских API без соответствующих эндпоинтов на бэкенде |
| `no-circular-dependencies` | `ERROR` | Циклические зависимости модулей (`A -> B -> A`), ведущие к TDZ-сбоям |
| `no-ai-sycophancy-watermarks` | `WARN` | Водяные знаки подхалимажа ИИ (`// As an AI...`, `// Hope this helps`) |
| `no-ghost-env-vars` | `ERROR` | Переменные окружения, отсутствующие в `.env.example` / шаблоне `.env` |
| `no-deep-nesting-sprawl` | `WARN` | Глубокая вложенность конструкций ветвления (>= 5 уровней) |
| `no-type-safety-degradation`| `ERROR` | Подавление проверки типов: `@ts-ignore`, `@ts-nocheck`, `: any`, `as unknown as any` |
| `no-ghost-dependencies` | `ERROR` | Сторонние библиотеки, импортированные без объявления в `package.json` |
| `no-duplicate-shadow-utilities`| `WARN` | Дубликаты утилит, изобретающие заново существующие функции проекта |
| `no-ai-conversational-bloat`| `WARN` | Разговорный мусор ИИ в комментариях (`// Here is the fix:`, `// Step 1:`) |
| `no-excessive-scope-drift` | `WARN` | Агент изменяет более 12 файлов или затрагивает критические `.env` / конфиги |
| `no-sql-injection-risk` | `ERROR` | Конкатенация сырых SQL-строк и уязвимости интерполяции шаблонов запросов |
| `no-floating-promises` | `ERROR` | Необработанные промисы без `await`, `.catch()` или маркера `void` |
| `no-async-array-predicates` | `ERROR` | Асинхронные предикаты в `.filter()`, `.some()`, `.every()`, `.find()` |
| `no-prototype-pollution` | `ERROR` | Загрязнение прототипа через невалидированные мутации `__proto__` |
| `no-command-injection-risk` | `ERROR` | Инъекции шелл-команд через неэкранированное выполнение процессов |
| `no-path-traversal-risk` | `ERROR` | Доступ к файловой системе через неочищенные пути `../` |
| `no-silent-error-swallowing` | `ERROR` | Тихое подавление отклонений промисов и проглатывание исключений |
| `no-redos-vulnerability` | `ERROR` | Уязвимости регулярных выражений к катастрофическому возврату (ReDoS) |
| `no-coverage-theater` | `ERROR` | Пустые утверждения, бессодержательные тесты и фиктивные проверки |
| `no-disabled-tls` | `ERROR` | Отключение проверки TLS (`NODE_TLS_REJECT_UNAUTHORIZED='0'`, `verify=False`) |
| `no-insecure-cors` | `ERROR` | Wildcard-источники (`Access-Control-Allow-Origin: *`) с передачей учетных данных |
| `no-inlined-secrets` | `ERROR` | Захардкоженные клиентские API-ключи и приватные токены во фронтенд-коде |
| `no-slopsquat-deps` | `ERROR` | Вредоносные зависимости с тайпсквоттингом и слопсквоттингом |
| `no-insecure-crypto` | `ERROR` | Устаревшие криптографические алгоритмы (`MD5`, `SHA1`, `DES`) в защищенных путях |
| `no-hallucinated-speculative-apis` | `ERROR` | Вымышленные методы рантайма и спекулятивные функции стандартных библиотек |
| `no-test-sabotage` | `ERROR` | Закомментированные ассерты, злоупотребление `.skip` или принудительный pass |
| `no-async-in-sync-callbacks` | `ERROR` | Асинхронные замыкания в синхронных циклах событий с потерей ошибок |
| `no-hardcoded-jwt-secrets` | `ERROR` | Захардкоженные HMAC-секреты, токены с `alg: none` и слабые ключи |
| `no-unscoped-idor-queries` | `ERROR` | Запросы к базе данных без проверки границ арендатора/пользователя (IDOR) |
| `no-unsafe-xml-or-ssrf` | `ERROR` | Инъекции внешних сущностей XXE и невалидированные внутренние запросы SSRF |
| `no-speculative-orm-methods` | `ERROR` | Вымышленные методы ORM/ODM (Prisma, TypeORM, Mongoose) |
| `no-async-pipeline-races` | `ERROR` | Незащищенные вызовы `Promise.race()` и утечки прерывания пайплайнов потоков |
| `no-phantom-cloud-sdks` | `ERROR` | Спекулятивные SDK облачных провайдеров в манифестах зависимостей |
| `no-ide-protocol-hallucinations` | `ERROR` | Вымышленные API расширений IDE (`vscode.window.showAIMessage`, `cursor.ai.*`) |
| `no-speculative-ffi-wasm` | `ERROR` | Вымышленные символы `extern "C"` и спекулятивные импорты WebAssembly хоста |
| `no-unbounded-concurrency` | `WARN` | Неограниченная конкурентность (`Promise.all` по `.map`, `asyncio.gather`) |
| `no-hallucinated-package-manifest` | `ERROR` | [Фаервол 49] Вымышленные составные пакеты и wildcard-версии (`*`, `==*`, `latest`) |
| `no-unawaited-async-in-loops` | `ERROR` | [Фаервол 50] Синхронные `.forEach(async)`, `.map(async)`, незакрепленные `tokio::spawn` и `asyncio.create_task` |
| `no-insecure-prng-tokens` | `ERROR` | [Фаервол 51] Некриптографические PRNG (`Math.random()`, `random.choice()`) для токенов и секретов |
| `no-unsafe-deserialization` | `ERROR` | [Фаервол 52] Гаджеты полиморфной десериализации (`pickle`, `yaml`, `node-serialize`, `XMLDecoder`, `Marshal`) |
| `no-deep-path-traversal` | `ERROR` | [Фаервол 53] Мультикодированные обходы (`%252e%252e`), Windows UNC (`\\?\`) и извлечение архивов Zip Slip |

---

## Бенчмарки производительности

| Инструмент | Холодный старт | Время проверки (10k строк diff) | Потребление памяти |
| :--- | :--- | :--- | :--- |
| **SlopGuard (Rust v2.9.0)** | **< 2.5ms** | **< 0.35ms / 1k LOC** | **< 5.8 MB** |
| Biome | ~45ms | ~30ms | ~28 MB |
| ESLint | ~620ms | ~480ms | ~140 MB |
| SonarQube Scanner | ~4,200ms | ~3,100ms | ~600 MB |

---

## Сообщество и Поддержка

- **Discord Сервер**: Обсуждения, примеры перехватов AI-галлюцинаций и поддержка: [discord.gg/z7C9dprye](https://discord.gg/z7C9dprye)
- **Telegram Канал**: Официальные релизы, бюллетени безопасности и бенчмарки: [@SlopGuard](https://t.me/SlopGuard)

---

## Лицензия

MIT © [avelone123123](https://github.com/avelone123123)
