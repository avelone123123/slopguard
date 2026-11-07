<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  [English](README.md) | **Русский** | [简体中文](README.zh.md) | [**Сайт и Документация**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Присоединиться-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  [![M8ven Score](https://m8ven.ai/badge/mcp/avelone123123-slopguard-ddb87e)](https://m8ven.ai/mcp/avelone123123-slopguard-ddb87e?s=readme)
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <a href="https://github.com/avelone123123/slopguard/releases/tag/v3.5.218"><img alt="Релиз: v3.5.218" src="https://img.shields.io/badge/release-v3.5.218-10b981?style=flat-square"></a>
  <img alt="Тесты: 477 пройдены" src="https://img.shields.io/badge/tests-477%20passed%20(100%25)-10b981?style=flat-square">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="Задержка: <0.35ms" src="https://img.shields.io/badge/latency-%3C0.35ms-2f6f4f?style=flat-square">
  <img alt="MCP Protocol: 2.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="Фаерволы: 284" src="https://img.shields.io/badge/firewalls-284%20active-blue?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>Защитите кодовую базу от накопления скрытого ИИ-мусора и галлюцинаций.</h3>

  <p>
    <b>SlopGuard</b> — сверхбыстрый фаервол на <b>чистом Rust</b> и <b>MCP-сервер</b> (Model Context Protocol),<br>
    который перехватывает недописанные заглушки, дубликаты утилит, обходы типов и разговоры ИИ в комментариях<br>
    <b>до того</b>, как они попадут в ваш git-репозиторий.
  </p>

  <code>cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard</code>
  <br>
  <small style="color:#94a3b8">или из локального репозитория: <code>cargo install --path .</code></small>

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
- **66 Активных AST-Фаерволов**: Комплексная защита от фантомных манифестов, асинхронных ловушек, утечек генераторов, нестойкой генерации ключей, бесконтрольных каналов, десериализации, уязвимостей Zip Slip, TOCTOU гонок файлов, галлюцинированных криптоалгоритмов и динамических SSRF интерполяций.
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
# Установка актуального релиза напрямую из GitHub через Cargo Git:
cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard

# Либо установка из локального клона репозитория:
cargo install --path .
```

### Готовые бинарные сборки (Precompiled Release Binaries)

| Целевой файл | Платформа | Формат | Контрольная сумма SHA-256 |
| :--- | :--- | :--- | :--- |
| `slopguard-v3.5.218-windows-x64.exe` | Windows x86_64 | Native PE Standalone | `A6F0345EF99300C19653F6232BA134A2B71118D5B85265FA1B9A69E9F5101B06` |

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
  - `[4] ФАЙРВОЛЫ`: полная матрица из 66 активных AST-файрволов и статус защиты в реальном времени.
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
- **График задержки (Latency)**: доказательство скорости Rust (<0.42ms) без замедления процесса разработки.

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

## Матрица угроз (THREAT-01 – THREAT-16)

SlopGuard распределяет 66 активных AST-фаерволов по 16 стратегическим векторам угроз ИИ-кодогенерации и цепочки поставок:

| ID вектора угроз | Описание вектора | Основные активные фаерволы |
| :--- | :--- | :--- |
| `THREAT-01` | Фантомные пакеты и фильтрация слопсквоттинга в манифестах | `SLOP-009`, `SLOP-011`, `SLOP-046`, `SLOP-049` |
| `THREAT-02` | Микротасковое голодание и неожидаемые асинхронные циклы | `SLOP-014`, `SLOP-019`, `SLOP-020`, `SLOP-050` |
| `THREAT-03` | Небезопасная энтропия PRNG и предсказуемая генерация ключей | `SLOP-004`, `SLOP-048`, `SLOP-051` |
| `THREAT-04` | Неканонический глубокий обход путей и защита от Zip-Slip | `SLOP-030`, `SLOP-053`, `SLOP-054` |
| `THREAT-05` | Перехват гаджетов полиморфной десериализации (RCE) | `SLOP-034`, `SLOP-052`, `SLOP-056` |
| `THREAT-06` | Спекулятивные вызовы нативного FFI ABI и хостового WASM | `SLOP-018` |
| `THREAT-07` | Составные пространства имен слопсквоттинга в манифестах | `SLOP-057` |
| `THREAT-08` | Утечки ресурсов при выходе из асинхронных генераторов | `SLOP-058` |
| `THREAT-09` | Прямая генерация ключей через PRNG и усеченная энтропия | `SLOP-059` |
| `THREAT-10` | Неограниченное разрастание каналов и очереди без backpressure | `SLOP-060` |
| `THREAT-11` | Динамическая инъекция команд через shell-интерполяцию | `SLOP-061` |
| `THREAT-12` | Утечки памяти через бесконтрольные слушатели событий в циклах | `SLOP-062` |
| `THREAT-13` | Галлюцинированные SDK-обертки ИИ и слопсквоттинг зависимостей | `SLOP-063` |
| `THREAT-14` | Спекулятивные гонки файлов TOCTOU и инвалидация конкурентности | `SLOP-064` |
| `THREAT-15` | Галлюцинированные и нестандартные криптографические алгоритмы | `SLOP-065` |
| `THREAT-16` | Спекулятивная SSRF-интерполяция динамических хостов и URL | `SLOP-066` |

---

## Правила и детекторы

| ID угрозы | Правило | Важность | Категория | Что перехватывает |
| :--- | :--- | :--- | :--- | :--- |
| `SLOP-001` | `no-hardcoded-secrets` | `ERROR` | Криптография и аутентификация | Утечки ключей OpenAI/Anthropic, учетных данных AWS, приватных RSA ключей |
| `SLOP-002` | `no-inlined-secrets` | `ERROR` | Криптография и аутентификация | Захардкоженные клиентские API-ключи и приватные токены во фронтенд-коде |
| `SLOP-003` | `no-ai-todo-placeholders` | `ERROR` | Качество кода | Недописанные заглушки TODO, `throw new Error("not implemented")` |
| `SLOP-004` | `no-insecure-crypto` | `ERROR` | Криптография и аутентификация | Устаревшие криптографические алгоритмы (`MD5`, `SHA1`, `DES`) в защищенных путях |
| `SLOP-005` | `no-circular-dependencies` | `ERROR` | Асинхронность и конкурентность | Циклические зависимости модулей (`A -> B -> A`), ведущие к TDZ-сбоям |
| `SLOP-006` | `no-mock-hallucinations` | `ERROR` | Цепочка поставок | Публичные мок-API (`jsonplaceholder`, фейковые JWT, тестовые email) |
| `SLOP-007` | `no-hallucinated-imports` | `ERROR` | Цепочка поставок | Галлюцинированные экспорты библиотек (`useLocalStorage`, `createClient`) |
| `SLOP-008` | `no-floating-promises` | `ERROR` | Асинхронность и конкурентность | Необработанные промисы без `await`, `.catch()` или маркера `void` |
| `SLOP-009` | `no-ghost-dependencies` | `ERROR` | Цепочка поставок | Сторонние библиотеки, импортированные без объявления в `package.json` |
| `SLOP-010` | `no-ai-conversational-bloat` | `WARN` | Качество кода | Разговорный мусор ИИ в комментариях (`// Here is the fix:`, `// Step 1:`) |
| `SLOP-011` | `no-slopsquat-deps` | `ERROR` | Цепочка поставок | Вредоносные зависимости с тайпсквоттингом и слопсквоттингом |
| `SLOP-012` | `no-ai-sycophancy-watermarks` | `WARN` | Качество кода | Водяные знаки подхалимажа ИИ (`// As an AI...`, `// Hope this helps`) |
| `SLOP-013` | `no-hallucinated-speculative-apis` | `ERROR` | Цепочка поставок | Вымышленные методы рантайма и спекулятивные функции стандартных библиотек |
| `SLOP-014` | `no-async-in-sync-callbacks` | `ERROR` | Асинхронность и конкурентность | Асинхронные замыкания в синхронных циклах событий с потерей ошибок |
| `SLOP-015` | `no-speculative-orm-methods` | `ERROR` | Цепочка поставок | Вымышленные методы ORM/ODM (Prisma, TypeORM, Mongoose) |
| `SLOP-016` | `no-phantom-cloud-sdks` | `ERROR` | Цепочка поставок | Спекулятивные SDK облачных провайдеров в манифестах зависимостей |
| `SLOP-017` | `no-ide-protocol-hallucinations` | `ERROR` | Цепочка поставок | Вымышленные API расширений IDE (`vscode.window.showAIMessage`, `cursor.ai.*`) |
| `SLOP-018` | `no-speculative-ffi-wasm` | `ERROR` | Цепочка поставок | Вымышленные символы `extern "C"` и спекулятивные импорты WebAssembly хоста |
| `SLOP-019` | `no-unbounded-concurrency` | `WARN` | Асинхронность и конкурентность | Неограниченная конкурентность (`Promise.all` по `.map`, `asyncio.gather`) |
| `SLOP-020` | `no-async-pipeline-races` | `WARN` | Асинхронность и конкурентность | Незащищенные вызовы `Promise.race()` и утечки прерывания пайплайнов потоков |
| `SLOP-021` | `no-async-array-predicates` | `ERROR` | Асинхронность и конкурентность | Асинхронные предикаты в `.filter()`, `.some()`, `.every()`, `.find()` |
| `SLOP-022` | `no-redos-vulnerability` | `ERROR` | Асинхронность и конкурентность | Уязвимости регулярных выражений к катастрофическому возврату (ReDoS) |
| `SLOP-023` | `no-deep-nesting-sprawl` | `WARN` | Асинхронность и конкурентность | Глубокая вложенность конструкций ветвления (>= 5 уровней) |
| `SLOP-024` | `no-hardcoded-jwt-secrets` | `ERROR` | Криптография и аутентификация | Захардкоженные HMAC-секреты, токены с `alg: none` и слабые ключи |
| `SLOP-025` | `no-disabled-tls` | `ERROR` | Криптография и аутентификация | Отключение проверки TLS (`NODE_TLS_REJECT_UNAUTHORIZED='0'`, `verify=False`) |
| `SLOP-026` | `no-insecure-cors` | `ERROR` | Криптография и аутентификация | Wildcard-источники (`Access-Control-Allow-Origin: *`) с передачей учетных данных |
| `SLOP-027` | `no-unscoped-idor-queries` | `ERROR` | Криптография и аутентификация | Запросы к базе данных без проверки границ арендатора/пользователя (IDOR) |
| `SLOP-028` | `no-dummy-secrets-mocks` | `ERROR` | Криптография и аутентификация | Мок-токены (`sk-proj-dummy...`, `dummy_api_key`) |
| `SLOP-029` | `no-ghost-env-vars` | `WARN` | Криптография и аутентификация | Переменные окружения, отсутствующие в `.env.example` / шаблоне `.env` |
| `SLOP-030` | `no-path-traversal-risk` | `ERROR` | Пути и инъекции | Доступ к файловой системе через неочищенные пути `../` |
| `SLOP-031` | `no-sql-injection-risk` | `ERROR` | Пути и инъекции | Конкатенация сырых SQL-строк и уязвимости интерполяции шаблонов запросов |
| `SLOP-032` | `no-command-injection-risk` | `ERROR` | Пути и инъекции | Инъекции шелл-команд через неэкранированное выполнение процессов |
| `SLOP-033` | `no-unsafe-xml-or-ssrf` | `ERROR` | Пути и инъекции | Инъекции внешних сущностей XXE и невалидированные внутренние запросы SSRF |
| `SLOP-034` | `no-unsafe-dynamic-eval` | `ERROR` | Пути и инъекции | Опасные вызовы `eval()`, `new Function()`, `pickle.loads`, `shell=True` |
| `SLOP-035` | `no-prototype-pollution` | `ERROR` | Пути и инъекции | Загрязнение прототипа через невалидированные мутации `__proto__` |
| `SLOP-036` | `no-phantom-api-routes` | `ERROR` | Пути и инъекции | Вызовы клиентских API без соответствующих эндпоинтов на бэкенде |
| `SLOP-037` | `no-silent-catch-pass` | `ERROR` | Пути и инъекции | Пустые блоки `catch(e) {}` и Python `except: pass` |
| `SLOP-038` | `no-silent-error-swallowing` | `ERROR` | Пути и инъекции | Тихое подавление отклонений промисов и проглатывание исключений |
| `SLOP-039` | `no-test-sabotage` | `ERROR` | Качество кода | Закомментированные ассерты, злоупотребление `.skip` или принудительный pass |
| `SLOP-040` | `no-coverage-theater` | `ERROR` | Качество кода | Пустые утверждения, бессодержательные тесты и фиктивные проверки |
| `SLOP-041` | `no-duplicate-shadow-utilities` | `WARN` | Качество кода | Дубликаты утилит, изобретающие заново существующие функции проекта |
| `SLOP-042` | `no-type-safety-degradation` | `ERROR` | Качество кода | Подавление проверки типов: `@ts-ignore`, `@ts-nocheck`, `: any`, `as unknown as any` |
| `SLOP-043` | `no-excessive-scope-drift` | `WARN` | Качество кода | Агент изменяет более 12 файлов или затрагивает критические `.env` / конфиги |
| `SLOP-044` | `no-unsafe-buffer-overflows` | `ERROR` | Качество кода | Неограниченная индексация буферов и сырой непроверенный доступ к памяти |
| `SLOP-045` | `no-dangling-native-pointers` | `ERROR` | Качество кода | Висячие нативные указатели и риски use-after-free |
| `SLOP-046` | `no-multi-registry-slopsquat` | `ERROR` | Цепочка поставок | Мульти-реестровый слопсквоттинг со спекулятивными пакетами |
| `SLOP-047` | `no-async-stream-leaks` | `WARN` | Асинхронность и конкурентность | Непотребленные асинхронные генераторы потоков, вызывающие утечку дескрипторов |
| `SLOP-048` | `no-static-entropy-seeds` | `ERROR` | Криптография и аутентификация | Статические константные сиды для псевдослучайных генераторов |
| `SLOP-049` | `no-hallucinated-package-manifest` | `ERROR` | Цепочка поставок | Спекулятивные пакеты и wildcard-версии (`*`, `==*`, `latest`) в манифестах |
| `SLOP-050` | `no-unawaited-async-in-loops` | `ERROR` | Асинхронность и конкурентность | Неожидаемые асинхронные колбэки в итераторах, вызывающие микротасковое голодание |
| `SLOP-051` | `no-insecure-prng-tokens` | `ERROR` | Криптография и аутентификация | Небезопасный PRNG (`Math.random()`, `random.choice()`) в контексте токенов и секретов |
| `SLOP-052` | `no-unsafe-deserialization` | `ERROR` | Пути и инъекции | Гаджеты полиморфной десериализации (`pickle`, `yaml`, `node-serialize`, `XMLDecoder`) |
| `SLOP-053` | `no-deep-path-traversal` | `ERROR` | Пути и инъекции | Мультикодированные обходы (`%252e%252e`), Windows UNC (`\\?\`) и Zip Slip |
| `SLOP-054` | `no-zip-slip-traversal` | `ERROR` | Пути и инъекции | ZipSlip-обход путей при распаковке архивов и нарушение изоляции границ |
| `SLOP-055` | `no-ide-protocol-hallucinations` | `ERROR` | Цепочка поставок | Галлюцинированные протоколы и API расширений IDE (`vscode.window.showAIMessage` и др.) |
| `SLOP-056` | `no-polymorphic-deserialization` | `ERROR` | Пути и инъекции | Гаджеты полиморфной десериализации, приводящие к удаленному выполнению кода (RCE) |
| `SLOP-057` | `no-slopsquat-compound-namespace` | `ERROR` | Цепочка поставок | Составные слопсквоттинг-пакеты, нацеленные на спекулятивные зависимости |
| `SLOP-058` | `no-async-generator-unhandled-break` | `ERROR` | Асинхронность и конкурентность | Необработанный break/return в асинхронном генераторе, вызывающий утечку сокетов |
| `SLOP-059` | `no-insecure-direct-crypto-keygen` | `ERROR` | Криптография и аутентификация | Прямая генерация ключей через PRNG или усеченная энтропия (<256 бит) |
| `SLOP-060` | `no-unbounded-channel-sprawl` | `ERROR` | Асинхронность и конкурентность | Создание неограниченных очередей и каналов без механизмов противодавления (backpressure) |
| `SLOP-061` | `no-speculative-subprocess-injection` | `ERROR` | Команды и инъекции | Уязвимости динамической интерполяции шелл-команд в подпроцессах (CWE-78) |
| `SLOP-062` | `no-unbounded-event-listener-loop` | `WARN` | Асинхронность и конкурентность | Неконтролируемая регистрация слушателей событий в циклах с утечкой памяти (CWE-400) |
| `SLOP-063` | `no-phantom-ai-wrappers` | `ERROR` | Цепочка поставок | Галлюцинированные SDK-обертки ИИ и слопсквоттинг зависимостей (CWE-1357) |
| `SLOP-064` | `no-speculative-file-race-toctou` | `ERROR` | Пути и инъекции | Гонки файлов check-then-write и TOCTOU-уязвимости (CWE-367) |
| `SLOP-065` | `no-hallucinated-crypto-algorithm` | `ERROR` | Криптография и аутентификация | Вымышленные криптографические алгоритмы (`aes-256-turbo`, `sha256-fast`) (CWE-327) |
| `SLOP-066` | `no-speculative-ssrf-interpolation` | `ERROR` | Пути и инъекции | Невалидированная интерполяция динамических хостов и SSRF-шаблоны (CWE-918) |

---

## Бенчмарки производительности

| Инструмент | Холодный старт | Время проверки (10k строк diff) | Потребление памяти |
| :--- | :--- | :--- | :--- |
| **SlopGuard (Rust v3.5.218)** | **< 2.5ms** | **< 0.35ms / 10k LOC** | **< 5.8 MB** |
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
