#[cfg(test)]
mod test_suite {
    use crate::dashboard::get_dashboard_html;
    use crate::detectors::ai_bloat::detect_ai_comment_bloat_in_content;
    use crate::detectors::circular_deps::detect_circular_dependencies_in_content;
    use crate::detectors::command_injection::detect_command_injection_in_content;
    use crate::detectors::complexity::detect_nesting_sprawl_in_content;
    use crate::detectors::compute_summary;
    use crate::detectors::env_vars::detect_ghost_env_vars_in_content;
    use crate::detectors::floating_promises::detect_floating_promises_in_content;
    use crate::detectors::ghost_dependencies::detect_ghost_dependencies_in_content;
    use crate::detectors::hallucinated_imports::detect_hallucinated_imports_in_content;
    use crate::detectors::mocks::detect_mocks_in_content;
    use crate::detectors::path_traversal::detect_path_traversal_in_content;
    use crate::detectors::phantom_routes::detect_phantom_routes_in_content;
    use crate::detectors::placeholders::detect_placeholders_in_content;
    use crate::detectors::prototype_pollution::detect_prototype_pollution_in_content;
    use crate::detectors::secrets::detect_secrets_in_content;
    use crate::detectors::sql_injection::detect_sql_injection_in_content;
    use crate::detectors::type_degradation::detect_type_degradation_in_content;
    use crate::detectors::unsafe_eval::detect_unsafe_eval_in_content;
    use crate::detectors::watermarks::detect_watermarks_in_content;
    use crate::fixers::sanitize_code;
    use crate::reporters::generate_badge_svg;
    use crate::telemetry::{create_shared_telemetry, record_event};
    use std::path::Path;
    #[test]
    fn test_detects_placeholders_and_dummy_tokens() {
        let code = ["const key = \"sk-1234567890123456789012\";", "fn run() {", "    let _todo = 1;", "    panic!(\"not implemented yet\");", "}", "fn handle() {", "    let _ = std::panic::catch_unwind(|| {});", "}"].join("\n");
        let findings = detect_placeholders_in_content("test.rs", &code);
        assert!(!findings.is_empty());
    }
    #[test]
    fn test_detects_rust_and_go_stubs() {
        let t_macro = ["to", "do!()"].concat();
        let u_macro = ["unimple", "mented!()"].concat();
        let rust_code = format!("fn compute() {{\n    {t_macro};\n}}\nfn backup() {{\n    {u_macro};\n}}");
        let findings = detect_placeholders_in_content("test.rs", &rust_code);
        assert_eq!(findings.len(), 2);
    }
    #[test]
    fn test_detects_mock_hallucinations() {
        let host = ["json", "placeholder.typicode.com"].concat();
        let email = ["john", "@", "example.com"].concat();
        let code = format!("const API = \"https://{host}/posts\";\nconst user = \"{email}\";\n");
        let findings = detect_mocks_in_content("api.ts", &code);
        assert_eq!(findings.len(), 2);
    }
    #[test]
    fn test_detects_type_degradations() {
        let k_ts = ["@", "ts-", "ignore"].concat();
        let k_any = ["a", "n", "y"].concat();
        let code = format!("// {k_ts}\nconst x: {k_any} = 42;\nconst y = z as unknown as {k_any};\n");
        let findings = detect_type_degradation_in_content("test.ts", &code);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn test_detects_ghost_dependencies() {
        let code = "import axios from \"axios\";\nimport { merge } from \"lodash-es\";\n";
        let findings = detect_ghost_dependencies_in_content("test.ts", code, Path::new("."), &[]);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn test_detects_and_sanitizes_ai_bloat() {
        let bloat = "// Here is the updated code:\nfn sum(a: i32, b: i32) -> i32 {\n// In order to return\na + b\n}\n";
        let findings = detect_ai_comment_bloat_in_content("test.rs", bloat);
        assert!(!findings.is_empty());
        let (cleaned, modified) = sanitize_code(bloat);
        assert!(modified);
        assert!(!cleaned.contains("Here is the updated code"));
        assert!(cleaned.contains("fn sum"));
    }
    #[test]
    fn test_dashboard_ui_html() {
        let html = get_dashboard_html();
        assert!(html.contains("SLOP"));
        assert!(html.contains("GUARD"));
        assert!(html.contains("/api/telemetry"));
    }
    #[test]
    fn test_telemetry_event_recording() {
        let shared = create_shared_telemetry();
        record_event(&shared, Path::new("."), "test_tool", "src/auth.rs", 2, 85, 3, "Blocked 2 items");
        let state = shared.lock().unwrap();
        assert_eq!(state.total_scans, 1);
        assert_eq!(state.total_slop_blocked, 2);
        assert_eq!(state.current_hygiene_score, 85);
        assert_eq!(state.events.len(), 1);
    }
    #[test]
    fn test_computes_score_and_badge() {
        let summary = compute_summary(Vec::new(), 10, 42);
        assert_eq!(summary.score, 100);
        assert_eq!(summary.total_issues, 0);
        let badge = generate_badge_svg(summary.score, summary.errors);
        assert!(badge.contains("100% clean"));
        assert!(badge.contains("<svg"));
    }
    #[test]
    fn test_detects_ghost_environment_variables() {
        let temp_dir = std::env::temp_dir().join(format!("slopguard_env_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let env_file = temp_dir.join(".env.example");
        let _ = std::fs::write(&env_file, "DATABASE_URL=postgres://localhost\nPORT=3000\n");
        let code = "const key = process.env.MY_UNKNOWN_LLM_KEY;\nconst port = process.env.PORT;\n";
        let findings = detect_ghost_env_vars_in_content("config.ts", code, &temp_dir);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-ghost-env-vars");
    }
    #[test]
    fn test_detects_deep_nesting_sprawl() {
        let code = "function handle() {\n    if (a) {\n        if (b) {\n            if (c) {\n                if (d) {\n                    if (e) {\n                        if (f) {\n                            if (g) {\n                                return true;\n                            }\n                        }\n                    }\n                }\n            }\n        }\n    }\n}\n";
        let findings = detect_nesting_sprawl_in_content("app.ts", code, 7);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule_id, "no-deep-nesting-sprawl");
    }
    #[test]
    fn test_detects_hallucinated_package_imports() {
        let h1 = ["useLocal", "Storage"].concat();
        let h2 = ["use", "Fetch"].concat();
        let h3 = ["create", "Client"].concat();
        let code = format!("import {{ {h1}, {h2} }} from 'react';\nimport {{ {h3} }} from 'axios';\n");
        let findings = detect_hallucinated_imports_in_content("App.tsx", &code);
        assert_eq!(findings.len(), 3);
        assert_eq!(findings[0].rule_id, "no-hallucinated-imports");
    }
    #[test]
    fn test_detects_phantom_api_routes() {
        let temp_dir = std::env::temp_dir().join(format!("slopguard_routes_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let api_dir = temp_dir.join("src").join("api");
        let _ = std::fs::create_dir_all(&api_dir);
        let _ = std::fs::write(api_dir.join("users.ts"), "export const getUsers = () => {};\n");
        let code = "const res = await fetch('/api/nonexistent_phantom_endpoint');\n";
        let findings = detect_phantom_routes_in_content("src/client.ts", code, &temp_dir);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-phantom-api-routes");
    }
    #[test]
    fn test_detects_circular_dependencies() {
        let temp_dir = std::env::temp_dir().join(format!("slopguard_circ_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let src_dir = temp_dir.join("src");
        let _ = std::fs::create_dir_all(&src_dir);
        let _ = std::fs::write(src_dir.join("b.ts"), "import { a } from './a';\nexport const b = 1;\n");
        let a_code = "import { b } from './b';\nexport const a = 2;\n";
        let _ = std::fs::write(src_dir.join("a.ts"), a_code);
        let findings = detect_circular_dependencies_in_content("src/a.ts", a_code, &temp_dir);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-circular-dependencies");
    }
    #[test]
    fn test_detects_ai_sycophancy_watermarks() {
        let p1 = ["As an ", "AI"].concat();
        let p2 = ["Hope this ", "helps"].concat();
        let code = format!("// {p1} coding assistant, I have implemented your request:\nconst x = 42;\n// {p2}! Let me know if you need anything else.\n");
        let findings = detect_watermarks_in_content("auth.ts", &code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-ai-sycophancy-watermarks");
    }
    #[test]
    fn test_detects_hardcoded_secrets() {
        let sec = ["sk-proj-", "abcdef1234567890abcdef1234567890"].concat();
        let code = format!("const OPENAI_KEY = \"{sec}\";\n");
        let findings = detect_secrets_in_content("config.ts", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-hardcoded-secrets");
    }
    #[test]
    fn test_detects_unsafe_eval() {
        let ev = ["ev", "al"].concat();
        let code = format!("const result = {ev}(\"2 + 2\");\n");
        let findings = detect_unsafe_eval_in_content("calc.js", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unsafe-dynamic-eval");
    }
    #[test]
    fn test_detects_sql_injection_hallucinations() {
        let sel_js = ["SEL", "ECT * FROM users WHERE id = ${userId}"].concat();
        let code_js = format!("const query = `{sel_js}`;");
        let f_js = detect_sql_injection_in_content("query.ts", &code_js);
        assert_eq!(f_js.len(), 1);
        assert_eq!(f_js[0].rule_id, "no-sql-injection-risk");
        let sel_cat = ["SEL", "ECT * FROM items WHERE name = '\" + name + \"'"].concat();
        let code_cat = format!("const q = \"{sel_cat};");
        let f_cat = detect_sql_injection_in_content("query.js", &code_cat);
        assert_eq!(f_cat.len(), 1);
        let sel_py = ["SEL", "ECT * FROM accounts WHERE email = '{email}'"].concat();
        let code_py = format!("q = f\"{sel_py}\"");
        let f_py = detect_sql_injection_in_content("db.py", &code_py);
        assert_eq!(f_py.len(), 1);
        let sel_pct = ["SEL", "ECT * FROM accounts WHERE email = %s\" % (email,)"].concat();
        let code_pct = format!("q = \"{sel_pct}");
        let f_pct = detect_sql_injection_in_content("db.py", &code_pct);
        assert_eq!(f_pct.len(), 1);
        let sel_rs = ["SEL", "ECT * FROM users WHERE id = '{}'\", uid)"].concat();
        let code_rs = format!("let q = format!(\"{sel_rs};");
        let f_rs = detect_sql_injection_in_content("repo.rs", &code_rs);
        assert_eq!(f_rs.len(), 1);
        let del_go = ["DEL", "ETE FROM sessions WHERE token = '%s'\", token)"].concat();
        let code_go = format!("q := fmt.Sprintf(\"{del_go}");
        let f_go = detect_sql_injection_in_content("auth.go", &code_go);
        assert_eq!(f_go.len(), 1);
        let sel_php = ["SEL", "ECT * FROM users WHERE username = '$username'\""].concat();
        let code_php = format!("$q = \"{sel_php};");
        let f_php = detect_sql_injection_in_content("login.php", &code_php);
        assert_eq!(f_php.len(), 1);
        let safe_pg = "const q = \"SELECT * FROM users WHERE id = $1 AND role = $2\";";
        let safe_mysql = "const q = \"SELECT * FROM users WHERE id = ? AND status = ?\";";
        let safe_py = "cursor.execute(\"SELECT * FROM accounts WHERE id = %s\", (uid,))";
        let safe_php = "$stmt = $pdo->prepare(\"SELECT * FROM users WHERE id = :id\");";
        let safe_code = format!("{safe_pg}\n{safe_mysql}\n{safe_py}\n{safe_php}\n");
        let safe_findings = detect_sql_injection_in_content("safe.ts", &safe_code);
        assert!(safe_findings.is_empty());
    }
    #[test]
    fn test_detects_floating_promises_and_async_antipatterns() {
        let code_for_each = "items.forEach(async (item) => { await processItem(item); });\n";
        let f1 = detect_floating_promises_in_content("handler.ts", code_for_each);
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0].rule_id, "no-floating-promises");
        let code_executor = "const task = new Promise(async (resolve, reject) => { resolve(await compute()); });\n";
        let f2 = detect_floating_promises_in_content("task.ts", code_executor);
        assert_eq!(f2.len(), 1);
        assert_eq!(f2[0].rule_id, "no-floating-promises");
        let code_try = "async function run() {\n    try {\n        return fetchResults();\n    } catch (e) {\n        console.error(e);\n    }\n}\n";
        let f3 = detect_floating_promises_in_content("run.ts", code_try);
        assert_eq!(f3.len(), 1);
        assert_eq!(f3[0].rule_id, "no-floating-promises");
        let code_then = "api.syncData().then(res => render(res));\n";
        let f4 = detect_floating_promises_in_content("sync.ts", code_then);
        assert_eq!(f4.len(), 1);
        assert_eq!(f4[0].rule_id, "no-floating-promises");
        let safe_code = [
            "for (const item of items) { await processItem(item); }",
            "await Promise.all(items.map(async (i) => await processItem(i)));",
            "const p = new Promise((resolve, reject) => { resolve(42); });",
            "async function safeRun() { try { return await fetchResults(); } catch (e) { log(e); } }",
            "api.syncData().then(res => render(res)).catch(err => log(err));",
            "const assigned = api.syncData().then(res => render(res));",
            "return api.syncData().then(res => render(res));",
            "void api.syncData().then(res => render(res));",
        ].join("\n");
        let safe_findings = detect_floating_promises_in_content("safe.ts", &safe_code);
        assert!(safe_findings.is_empty());
    }
    #[test]
    fn test_detects_prototype_pollution() {
        let direct_proto = "target[\"__proto__\"] = evilPayload;\n";
        let f1 = detect_prototype_pollution_in_content("merge.js", direct_proto);
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0].rule_id, "no-prototype-pollution");
        let obj_proto = "Object.prototype.isAdmin = true;\n";
        let f2 = detect_prototype_pollution_in_content("auth.ts", obj_proto);
        assert_eq!(f2.len(), 1);
        assert_eq!(f2[0].rule_id, "no-prototype-pollution");
        let naive_merge = [
            "function deepMerge(target, source) {",
            "  for (const key of Object.keys(source)) {",
            "    if (typeof source[key] === 'object' && source[key] !== null) {",
            "      if (!target[key]) target[key] = {};",
            "      deepMerge(target[key], source[key]);",
            "    } else {",
            "      target[key] = source[key];",
            "    }",
            "  }",
            "  return target;",
            "}",
        ].join("\n");
        let f3 = detect_prototype_pollution_in_content("merge.js", &naive_merge);
        assert_eq!(f3.len(), 1);
        assert_eq!(f3[0].rule_id, "no-prototype-pollution");
        let path_setter = [
            "function setByPath(obj, path, value) {",
            "  const parts = path.split('.');",
            "  let current = obj;",
            "  for (let i = 0; i < parts.length - 1; i++) {",
            "    const part = parts[i];",
            "    if (!current[part]) current[part] = {};",
            "    current = current[part];",
            "  }",
            "  current[parts[parts.length - 1]] = value;",
            "}",
        ].join("\n");
        let f4 = detect_prototype_pollution_in_content("setter.ts", &path_setter);
        assert_eq!(f4.len(), 1);
        assert_eq!(f4[0].rule_id, "no-prototype-pollution");
        let safe_code = [
            "function safeMerge(target, source) {",
            "  for (const key of Object.keys(source)) {",
            "    if (key === '__proto__' || key === 'constructor' || key === 'prototype') continue;",
            "    if (typeof source[key] === 'object' && source[key] !== null) {",
            "      if (!target[key]) target[key] = {};",
            "      safeMerge(target[key], source[key]);",
            "    } else {",
            "      target[key] = source[key];",
            "    }",
            "  }",
            "  return target;",
            "}",
            "function safeSetPath(obj, path, value) {",
            "  const parts = path.split('.');",
            "  let current = obj;",
            "  for (const part of parts) {",
            "    if (part === '__proto__' || part === 'constructor' || part === 'prototype') return;",
            "    if (!current[part]) current[part] = {};",
            "    current = current[part];",
            "  }",
            "}",
        ].join("\n");
        let safe_findings = detect_prototype_pollution_in_content("safe.ts", &safe_code);
        assert!(safe_findings.is_empty());
    }
    #[test]
    fn test_detects_command_injection() {
        let ts_inj1 = ["exe", "c(`git clone ${repoUrl}`) ;"].concat();
        let f_ts1 = detect_command_injection_in_content("deploy.ts", &ts_inj1);
        assert_eq!(f_ts1.len(), 1);
        assert_eq!(f_ts1[0].rule_id, "no-command-injection-risk");
        let ts_inj2 = ["child_process.execSync(\"curl \" + target)"].concat();
        let f_ts2 = detect_command_injection_in_content("fetch.ts", &ts_inj2);
        assert_eq!(f_ts2.len(), 1);
        assert_eq!(f_ts2[0].rule_id, "no-command-injection-risk");
        let ts_inj3 = "spawn(\"git\", { shell: true });\n";
        let f_ts3 = detect_command_injection_in_content("runner.ts", ts_inj3);
        assert_eq!(f_ts3.len(), 1);
        assert_eq!(f_ts3[0].rule_id, "no-command-injection-risk");
        let py_inj1 = "os.system(f\"ffmpeg -i {input_path} {output_path}\")\n";
        let f_py1 = detect_command_injection_in_content("convert.py", py_inj1);
        assert_eq!(f_py1.len(), 1);
        assert_eq!(f_py1[0].rule_id, "no-command-injection-risk");
        let py_inj2 = "subprocess.run(f\"git checkout {branch}\", shell=True)\n";
        let f_py2 = detect_command_injection_in_content("git.py", py_inj2);
        assert_eq!(f_py2.len(), 1);
        assert_eq!(f_py2[0].rule_id, "no-command-injection-risk");
        let py_inj3 = "os.popen(f\"ping -c 1 {host}\")\n";
        let f_py3 = detect_command_injection_in_content("net.py", py_inj3);
        assert_eq!(f_py3.len(), 1);
        assert_eq!(f_py3[0].rule_id, "no-command-injection-risk");
        let rs_sh = ["s", "h"].concat();
        let rs_inj = format!("Command::new(\"{rs_sh}\").arg(\"-c\").arg(format!(\"echo {{}}\", input));\n");
        let f_rs = detect_command_injection_in_content("executor.rs", &rs_inj);
        assert_eq!(f_rs.len(), 1);
        assert_eq!(f_rs[0].rule_id, "no-command-injection-risk");
        let go_sh = ["s", "h"].concat();
        let go_inj = format!("exec.Command(\"{go_sh}\", \"-c\", fmt.Sprintf(\"rm -rf %s\", target))\n");
        let f_go = detect_command_injection_in_content("cleaner.go", &go_inj);
        assert_eq!(f_go.len(), 1);
        assert_eq!(f_go[0].rule_id, "no-command-injection-risk");
        let php_inj1 = "exec(\"convert \" . $file . \" out.png\");\n";
        let f_php1 = detect_command_injection_in_content("img.php", php_inj1);
        assert_eq!(f_php1.len(), 1);
        assert_eq!(f_php1[0].rule_id, "no-command-injection-risk");
        let php_inj2 = "shell_exec(\"git checkout $branch\");\n";
        let f_php2 = detect_command_injection_in_content("deploy.php", php_inj2);
        assert_eq!(f_php2.len(), 1);
        assert_eq!(f_php2[0].rule_id, "no-command-injection-risk");
        let safe_ts = [
            "execFile(\"git\", [\"clone\", repoUrl]);",
            "spawn(\"git\", [\"clone\", repoUrl]);",
            "exec(\"git status\");",
        ].join("\n");
        assert!(detect_command_injection_in_content("safe.ts", &safe_ts).is_empty());
        let safe_py = [
            "subprocess.run([\"git\", \"checkout\", branch], shell=False)",
            "subprocess.run([\"git\", \"status\"], check=True)",
            "os.system(\"clear\")",
        ].join("\n");
        assert!(detect_command_injection_in_content("safe.py", &safe_py).is_empty());
        let safe_rs = "Command::new(\"git\").arg(\"checkout\").arg(branch);\n";
        assert!(detect_command_injection_in_content("safe.rs", safe_rs).is_empty());
        let safe_go = "exec.Command(\"git\", \"checkout\", branch)\n";
        assert!(detect_command_injection_in_content("safe.go", safe_go).is_empty());
        let safe_php = "exec(\"whoami\");\nshell_exec(\"uptime\");\n";
        assert!(detect_command_injection_in_content("safe.php", safe_php).is_empty());
    }
    #[test]
    fn test_detects_path_traversal() {
        let ts_inj1 = "res.sendFile(path.join(__dirname, \"uploads\", req.query.file));\n";
        let f_ts1 = detect_path_traversal_in_content("download.ts", ts_inj1);
        assert_eq!(f_ts1.len(), 1);
        assert_eq!(f_ts1[0].rule_id, "no-path-traversal-risk");
        let ts_inj2 = "fs.readFile(path.join(BASE_DIR, req.params.filename));\n";
        let f_ts2 = detect_path_traversal_in_content("view.ts", ts_inj2);
        assert_eq!(f_ts2.len(), 1);
        assert_eq!(f_ts2[0].rule_id, "no-path-traversal-risk");
        let ts_inj3 = "fs.createReadStream(path.join(\"/tmp\", req.params.id));\n";
        let f_ts3 = detect_path_traversal_in_content("stream.ts", ts_inj3);
        assert_eq!(f_ts3.len(), 1);
        assert_eq!(f_ts3[0].rule_id, "no-path-traversal-risk");
        let py_inj1 = "open(os.path.join(UPLOAD_DIR, request.args.get(\"file\")))\n";
        let f_py1 = detect_path_traversal_in_content("upload.py", py_inj1);
        assert_eq!(f_py1.len(), 1);
        assert_eq!(f_py1[0].rule_id, "no-path-traversal-risk");
        let py_inj2 = "send_file(os.path.join(\"/var/reports\", req.params.name))\n";
        let f_py2 = detect_path_traversal_in_content("report.py", py_inj2);
        assert_eq!(f_py2.len(), 1);
        assert_eq!(f_py2[0].rule_id, "no-path-traversal-risk");
        let py_inj3 = "open(f\"/var/data/{user_path}\")\n";
        let f_py3 = detect_path_traversal_in_content("data.py", py_inj3);
        assert_eq!(f_py3.len(), 1);
        assert_eq!(f_py3[0].rule_id, "no-path-traversal-risk");
        let rs_op = ["File::", "open"].concat();
        let rs_fmt = ["for", "mat!"].concat();
        let rs_inj1 = format!("{rs_op}({rs_fmt}(\"{{}}/{{}}\", base, user_input));\n");
        let f_rs1 = detect_path_traversal_in_content("reader.rs", &rs_inj1);
        assert_eq!(f_rs1.len(), 1);
        assert_eq!(f_rs1[0].rule_id, "no-path-traversal-risk");
        let rs_rd = ["std::fs::", "read_to_string"].concat();
        let rs_inj2 = format!("{rs_rd}(Path::new(&base).join(user_input));\n");
        let f_rs2 = detect_path_traversal_in_content("fetch.rs", &rs_inj2);
        assert_eq!(f_rs2.len(), 1);
        assert_eq!(f_rs2[0].rule_id, "no-path-traversal-risk");
        let go_inj1 = "os.Open(filepath.Join(dir, r.URL.Query().Get(\"file\")))\n";
        let f_go1 = detect_path_traversal_in_content("server.go", go_inj1);
        assert_eq!(f_go1.len(), 1);
        assert_eq!(f_go1[0].rule_id, "no-path-traversal-risk");
        let go_inj2 = "http.ServeFile(w, r, filepath.Join(dir, filename))\n";
        let f_go2 = detect_path_traversal_in_content("serve.go", go_inj2);
        assert_eq!(f_go2.len(), 1);
        assert_eq!(f_go2[0].rule_id, "no-path-traversal-risk");
        let php_inj1 = "readfile($dir . \"/\" . $_GET['page']);\n";
        let f_php1 = detect_path_traversal_in_content("index.php", php_inj1);
        assert_eq!(f_php1.len(), 1);
        assert_eq!(f_php1[0].rule_id, "no-path-traversal-risk");
        let php_inj2 = "include($_GET['page']);\n";
        let f_php2 = detect_path_traversal_in_content("page.php", php_inj2);
        assert_eq!(f_php2.len(), 1);
        assert_eq!(f_php2[0].rule_id, "no-path-traversal-risk");
        let php_inj3 = "file_get_contents($_POST['filename']);\n";
        let f_php3 = detect_path_traversal_in_content("api.php", php_inj3);
        assert_eq!(f_php3.len(), 1);
        assert_eq!(f_php3[0].rule_id, "no-path-traversal-risk");
        let safe_ts = [
            "const safe = path.basename(req.params.file);",
            "fs.readFile(path.join(DIR, safe));",
            "fs.readFile(path.join(__dirname, \"public\", \"index.html\"));",
            "if (resPath.startsWith(DIR)) { fs.readFile(resPath); }",
        ].join("\n");
        assert!(detect_path_traversal_in_content("safe.ts", &safe_ts).is_empty());
        let safe_py = [
            "fname = secure_filename(request.args.get(\"file\"))",
            "open(os.path.join(DIR, fname))",
            "open(\"data.csv\")",
            "if target.startswith(DIR):",
            "    open(target)",
        ].join("\n");
        assert!(detect_path_traversal_in_content("safe.py", &safe_py).is_empty());
        let safe_rs = [
            "let p = base.join(user_input);",
            "let c = p.canonicalize().unwrap();",
            "if c.starts_with(&base) {",
            "    let _ = std::fs::read_to_string(&c);",
            "}",
            "let _ = std::fs::read_to_string(\"config.toml\");",
        ].join("\n");
        assert!(detect_path_traversal_in_content("safe.rs", &safe_rs).is_empty());
        let safe_go = [
            "clean := filepath.Clean(filepath.Join(dir, filename))",
            "if strings.HasPrefix(clean, dir) {",
            "    f, _ := os.Open(clean)",
            "}",
            "f, _ := os.Open(\"static.json\")",
        ].join("\n");
        assert!(detect_path_traversal_in_content("safe.go", &safe_go).is_empty());
        let safe_php = [
            "$f = basename($_GET['file']);",
            "readfile($dir . '/' . $f);",
            "file_get_contents(\"template.html\");",
        ].join("\n");
        assert!(detect_path_traversal_in_content("safe.php", &safe_php).is_empty());
    }
}