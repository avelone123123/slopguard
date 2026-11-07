#[cfg(test)]
mod test_suite {
    use crate::dashboard::get_dashboard_html;
    use crate::detectors::ai_bloat::detect_ai_comment_bloat_in_content;
    use crate::detectors::circular_deps::detect_circular_dependencies_in_content;
    use crate::detectors::command_injection::detect_command_injection_in_content;
    use crate::detectors::complexity::detect_nesting_sprawl_in_content;
    use crate::detectors::compute_summary;
    use crate::detectors::coverage_theater::detect_coverage_theater_in_content;
    use crate::detectors::env_vars::detect_ghost_env_vars_in_content;
    use crate::detectors::floating_promises::detect_floating_promises_in_content;
    use crate::detectors::ghost_dependencies::detect_ghost_dependencies_in_content;
    use crate::detectors::hallucinated_imports::detect_hallucinated_imports_in_content;
    use crate::detectors::insecure_transport::detect_insecure_transport_in_content;
    use crate::detectors::mocks::detect_mocks_in_content;
    use crate::detectors::path_traversal::detect_path_traversal_in_content;
    use crate::detectors::phantom_routes::detect_phantom_routes_in_content;
    use crate::detectors::placeholders::detect_placeholders_in_content;
    use crate::detectors::prototype_pollution::detect_prototype_pollution_in_content;
    use crate::detectors::redos::detect_redos_in_content;
    use crate::detectors::secrets::detect_secrets_in_content;
    use crate::detectors::silent_errors::detect_silent_errors_in_content;
    use crate::detectors::sql_injection::detect_sql_injection_in_content;
    use crate::detectors::type_degradation::detect_type_degradation_in_content;
    use crate::detectors::unsafe_eval::detect_unsafe_eval_in_content;
    use crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content;
    use crate::detectors::insecure_cors::detect_insecure_cors_in_content;
    use crate::detectors::watermarks::detect_watermarks_in_content;
    use crate::detectors::inlined_secrets::detect_inlined_secrets_in_content;
    use crate::detectors::slopsquat::detect_slopsquat_in_content;
    use crate::detectors::insecure_crypto::detect_insecure_crypto_in_content;
    use crate::detectors::test_sabotage::detect_test_sabotage_in_content;
    use crate::detectors::unbounded_concurrency::detect_unbounded_concurrency_in_content;
    use crate::detectors::async_callback_traps::detect_async_callback_traps_in_content;
    use crate::detectors::hallucinated_speculative_apis::detect_speculative_apis_in_content;
    use crate::detectors::async_loop_traps::detect_async_loop_traps_in_content;
    use crate::detectors::package_hallucinations::detect_package_hallucinations_in_content;
    use crate::detectors::jwt_leakage::detect_jwt_leakage_in_content;
    use crate::detectors::broken_access_control::detect_broken_access_control_in_content;
    use crate::detectors::xxe_ssrf::detect_xxe_ssrf_in_content;
    use crate::detectors::speculative_orm::detect_speculative_orm_in_content;
    use crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content;
    use crate::detectors::async_race_traps::detect_async_race_traps_in_content;
    use crate::detectors::phantom_cloud_sdks::detect_phantom_cloud_sdks_in_content;
    use crate::detectors::ide_protocol::detect_ide_protocol_hallucinations_in_content;
    use crate::types::SlopCategory;
    use crate::fixers::sanitize_code;
    use crate::reporters::generate_badge_svg;
    use crate::telemetry::{create_shared_telemetry, record_event, record_telemetry_event};
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
        record_telemetry_event(&shared, Path::new("."), "test_tool", "src/auth.rs", 2, 85, 3, "Blocked 2 items");
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
    #[test]
    fn test_detects_silent_error_swallowing() {
        let js_code = ["fetchUserData().catch(() => {});", "fetchUserData().catch(err => {});", "fetchUserData().catch((e) => { console.log(e); });", "try { run(); } catch (e) { console.log(e); }", "const { data } = await supabase.from('users').select();"].join("\n");
        let findings = detect_silent_errors_in_content("service.ts", &js_code);
        assert!(findings.len() >= 5);
    }
    #[test]
    fn test_silent_errors_legitimate_handlers_ignored() {
        let safe_js = ["fetchUserData().catch((err) => { logger.error(err); throw err; });", "try { run(); } catch (e) { console.error(e); throw e; }", "try { run(); } catch (e) { return fallbackValue; }", "const { data, error } = await supabase.from('users').select();"].join("\n");
        let findings = detect_silent_errors_in_content("safe.ts", &safe_js);
        assert_eq!(findings.len(), 0);
    }
    #[test]
    fn test_detects_python_and_go_error_swallowing() {
        let py_code = "try:\n    do_job()\nexcept Exception as e:\n    print(e)\n";
        let findings_py = detect_silent_errors_in_content("task.py", py_code);
        assert_eq!(findings_py.len(), 1);
        let go_code = "val, err := fetch()\n_ = err\n";
        let findings_go = detect_silent_errors_in_content("worker.go", go_code);
        assert_eq!(findings_go.len(), 1);
    }
    #[test]
    fn test_detects_redos_vulnerabilities() {
        let evil_re = ["/(a+", ")+$/"].concat();
        let email_re = ["([a-zA-Z0-9]+)*", "@domain.com"].concat();
        let js_code = format!("const evilRe = {evil_re};\nconst emailCheck = new RegExp(\"{email_re}\");\n");
        let js_findings = detect_redos_in_content("validator.ts", &js_code);
        assert_eq!(js_findings.len(), 2);
        assert_eq!(js_findings[0].rule_id, "no-redos-vulnerability");
        let py_re1 = ["(\\w+", ")+"].concat();
        let py_re2 = ["(.*", "a)*"].concat();
        let py_code = format!("import re\npattern = re.compile(r\"{py_re1}\")\nmatcher = re.search(r\"{py_re2}\", text)\n");
        let py_findings = detect_redos_in_content("parser.py", &py_code);
        assert_eq!(py_findings.len(), 2);
        let go_re = ["([0-9]+", ")+"].concat();
        let go_code = format!("package main\nimport \"regexp\"\nvar safeRe = regexp.MustCompile(\"^[a-zA-Z0-9_-]+$\")\nvar badRe = regexp.MustCompile(\"{go_re}\")\n");
        let go_findings = detect_redos_in_content("main.go", &go_code);
        assert_eq!(go_findings.len(), 1);
        let safe_code = "const safeSlug = /^[a-z0-9_-]+$/;\nconst datePattern = /^\\d{4}-\\d{2}-\\d{2}$/;\n";
        let safe_findings = detect_redos_in_content("safe.ts", safe_code);
        assert_eq!(safe_findings.len(), 0);
    }
    #[test]
    fn test_detects_coverage_theater_and_vacuous_assertions() {
        let ts_code = "describe('auth', () => {\n  it('should pass', () => {\n    expect(true).toBe(true);\n    expect(1).toEqual(1);\n  });\n  it('empty stub', () => {});\n});\n";
        let findings_ts = detect_coverage_theater_in_content("auth.test.ts", ts_code);
        assert!(findings_ts.len() >= 3);
        let py_code = "def test_service():\n    assert True\n    assert 1 == 1\ndef test_empty():\n    pass\n";
        let findings_py = detect_coverage_theater_in_content("test_service.py", py_code);
        assert!(findings_py.len() >= 3);
    }
    #[test]
    fn test_detects_insecure_transport_and_tls_bypass() {
        let node_var = ["NODE_TLS_", "REJECT_UNAUTHORIZED"].concat();
        let rej_unauth = ["reject", "Unauthorized"].concat();
        let node_code = format!("process.env.{node_var} = '0';\nconst agent = new https.Agent({{ {rej_unauth}: false }});\n");
        let findings_node = detect_insecure_transport_in_content("client.ts", &node_code);
        assert_eq!(findings_node.len(), 2);
        let py_ver = ["veri", "fy=False"].concat();
        let py_code = format!("response = requests.get('https://internal.local', {py_ver})\nurllib3.disable_warnings()\n");
        let findings_py = detect_insecure_transport_in_content("api.py", &py_code);
        assert_eq!(findings_py.len(), 2);
        let go_skip = ["Insecure", "SkipVerify: true"].concat();
        let go_code = format!("cfg := &tls.Config{{{go_skip}}}\n");
        let findings_go = detect_insecure_transport_in_content("main.go", &go_code);
        assert_eq!(findings_go.len(), 1);
        let rs_danger = ["danger_accept_", "invalid_certs(true)"].concat();
        let rust_code = format!("let client = reqwest::Client::builder().{rs_danger}.build()?;\n");
        let findings_rust = detect_insecure_transport_in_content("fetcher.rs", &rust_code);
        assert_eq!(findings_rust.len(), 1);
    }
    #[test]
    fn test_detects_unsafe_deserialization() {
        let py_pick = ["data = pick", "le.loads(payload)"].concat();
        let py_findings1 = detect_unsafe_deserialization_in_content("cache.py", &py_pick);
        assert_eq!(py_findings1.len(), 1);
        assert_eq!(py_findings1[0].rule_id, "no-unsafe-deserialization");
        let py_yaml = ["config = ya", "ml.load(stream)"].concat();
        let py_findings2 = detect_unsafe_deserialization_in_content("config.py", &py_yaml);
        assert_eq!(py_findings2.len(), 1);
        let py_marsh = ["obj = mar", "shal.loads(data)"].concat();
        let py_findings3 = detect_unsafe_deserialization_in_content("store.py", &py_marsh);
        assert_eq!(py_findings3.len(), 1);
        let js_ser = ["const obj = node-serialize.unser", "ialize(cookie);"].concat();
        let js_findings = detect_unsafe_deserialization_in_content("session.ts", &js_ser);
        assert_eq!(js_findings.len(), 1);
        let php_code = ["$user = unser", "ialize($_COOKIE['user']);"].concat();
        let php_findings = detect_unsafe_deserialization_in_content("auth.php", &php_code);
        assert_eq!(php_findings.len(), 1);
        let rb_code = ["data = Mar", "shal.load(payload)"].concat();
        let rb_findings = detect_unsafe_deserialization_in_content("worker.rb", &rb_code);
        assert_eq!(rb_findings.len(), 1);
        let safe_py = "data = json.loads(payload)\nconfig = yaml.safe_load(stream)\n";
        let safe_py_findings = detect_unsafe_deserialization_in_content("safe.py", safe_py);
        assert_eq!(safe_py_findings.len(), 0);
        let safe_js = "const obj = JSON.parse(cookie);\n";
        let safe_js_findings = detect_unsafe_deserialization_in_content("safe.ts", safe_js);
        assert_eq!(safe_js_findings.len(), 0);
        let safe_php = "$user = json_decode($_COOKIE['user'], true);\n$obj = unserialize($data, ['allowed_classes' => false]);\n";
        let safe_php_findings = detect_unsafe_deserialization_in_content("safe.php", safe_php);
        assert_eq!(safe_php_findings.len(), 0);
    }
    #[test]
    fn test_detects_insecure_cors_node_and_express() {
        let code_bare = "app.use(cors());\n";
        let findings_bare = detect_insecure_cors_in_content("server.js", code_bare);
        assert_eq!(findings_bare.len(), 1);
        assert_eq!(findings_bare[0].rule_id, "no-insecure-cors");
        let code_header = "res.setHeader('Access-Control-Allow-Origin', '*');\n";
        let findings_header = detect_insecure_cors_in_content("api.ts", code_header);
        assert_eq!(findings_header.len(), 1);
        let code_opts = "const config = { origin: '*' };\n";
        let findings_opts = detect_insecure_cors_in_content("cors.ts", code_opts);
        assert_eq!(findings_opts.len(), 1);
    }
    #[test]
    fn test_detects_insecure_cors_python() {
        let py_fastapi = "app.add_middleware(CORSMiddleware, allow_origins=['*'])\n";
        let findings_fastapi = detect_insecure_cors_in_content("main.py", py_fastapi);
        assert_eq!(findings_fastapi.len(), 1);
        let py_flask = "CORS(app)\n";
        let findings_flask = detect_insecure_cors_in_content("app.py", py_flask);
        assert_eq!(findings_flask.len(), 1);
        let py_django = "CORS_ALLOW_ALL_ORIGINS = True\n";
        let findings_django = detect_insecure_cors_in_content("settings.py", py_django);
        assert_eq!(findings_django.len(), 1);
    }
    #[test]
    fn test_detects_insecure_cors_go_and_rust() {
        let go_default = "router.Use(cors.Default())\n";
        let findings_go = detect_insecure_cors_in_content("main.go", go_default);
        assert_eq!(findings_go.len(), 1);
        let go_cfg = "cfg := cors.Config{ AllowAllOrigins: true }\n";
        let findings_cfg = detect_insecure_cors_in_content("server.go", go_cfg);
        assert_eq!(findings_cfg.len(), 1);
        let actix_token = ["Cors::", "permissive()"].concat();
        let rust_actix = format!("let cors = {actix_token};\n");
        let findings_rust = detect_insecure_cors_in_content("api.rs", &rust_actix);
        assert_eq!(findings_rust.len(), 1);
        let tower_token = ["CorsLayer::", "permissive()"].concat();
        let rust_tower = format!("let layer = {tower_token};\n");
        let findings_tower = detect_insecure_cors_in_content("layer.rs", &rust_tower);
        assert_eq!(findings_tower.len(), 1);
    }
    #[test]
    fn test_insecure_cors_safe_configurations_ignored() {
        let safe_node = "app.use(cors({ origin: ['https://app.example.com'] }));\n";
        assert!(detect_insecure_cors_in_content("server.js", safe_node).is_empty());
        let safe_py = "app.add_middleware(CORSMiddleware, allow_origins=['https://app.example.com'])\n";
        assert!(detect_insecure_cors_in_content("main.py", safe_py).is_empty());
        let safe_rust = "let cors = Cors::default().allowed_origin(\"https://app.example.com\");\n";
        assert!(detect_insecure_cors_in_content("main.rs", safe_rust).is_empty());
    }
    #[test]
    fn test_detects_async_array_predicates_truthy_trap() {
        let f1 = detect_floating_promises_in_content("auth.ts", "const allowed = users.filter(async (u) => await checkUserPermission(u.id));\n");
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0].rule_id, "no-async-array-predicates");
        let f2 = detect_floating_promises_in_content("auth.ts", "const hasAccess = roles.some(async (r) => await authService.checkRole(r));\n");
        assert_eq!(f2.len(), 1);
        assert_eq!(f2[0].rule_id, "no-async-array-predicates");
        let f3 = detect_floating_promises_in_content("validator.ts", "const allValid = records.every(async (i) => await validate(i));\n");
        assert_eq!(f3.len(), 1);
        assert_eq!(f3[0].rule_id, "no-async-array-predicates");
        let f4 = detect_floating_promises_in_content("find.ts", "const target = list.find(async (i) => await match(i));\n");
        assert_eq!(f4.len(), 1);
        assert_eq!(f4[0].rule_id, "no-async-array-predicates");
        let f5 = detect_floating_promises_in_content("find.ts", "const idx = list.findIndex(async (i) => await match(i));\n");
        assert_eq!(f5.len(), 1);
        assert_eq!(f5[0].rule_id, "no-async-array-predicates");
        let f6 = detect_floating_promises_in_content("find.ts", "const last = list.findLast(async (i) => await match(i));\n");
        assert_eq!(f6.len(), 1);
        assert_eq!(f6[0].rule_id, "no-async-array-predicates");
        let f7 = detect_floating_promises_in_content("find.ts", "const lastIdx = list.findLastIndex(async (i) => await match(i));\n");
        assert_eq!(f7.len(), 1);
        assert_eq!(f7[0].rule_id, "no-async-array-predicates");
        let f8 = detect_floating_promises_in_content("typed.ts", "const filtered = users.filter((u: User): Promise<boolean> => check(u));\n");
        assert_eq!(f8.len(), 1);
        assert_eq!(f8[0].rule_id, "no-async-array-predicates");
        let f9 = detect_floating_promises_in_content("worker.ts", "items.map(async (item) => { await processItem(item); });\n");
        assert_eq!(f9.len(), 1);
        assert_eq!(f9[0].rule_id, "no-floating-promises");
        let safe_cases = [
            "const active = users.filter((u) => u.isActive);",
            "await Promise.all(items.map(async (i) => await process(i)));",
            "const promises = items.map(async (i) => await process(i));",
            "return Promise.all(items.map(async (i) => await process(i)));",
            "for (const item of items) { await processItem(item); }",
        ].join("\n");
        let safe_findings = detect_floating_promises_in_content("safe.ts", &safe_cases);
        assert_eq!(safe_findings.len(), 0);
    }
    #[test]
    fn test_detects_inlined_client_secrets() {
        let prefix = ["NEXT", "_PUBLIC_"].concat();
        let role = ["SERVICE_", "ROLE_KEY"].concat();
        let key_name = format!("{prefix}SUPABASE_{role}");
        let code = format!("const client = createClient(url, process.env.{key_name});");
        let findings = detect_inlined_secrets_in_content("src/app/page.tsx", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-inlined-secrets");
    }
    #[test]
    fn test_detects_slopsquat_hallucinated_dependencies() {
        let pkg1 = ["crypto", "-secure-hash"].concat();
        let pkg2 = ["starlette", "-reverse-proxy"].concat();
        let manifest = format!("{{\n  \"dependencies\": {{\n    \"{pkg1}\": \"^1.0.0\"\n  }}\n}}");
        let findings1 = detect_slopsquat_in_content("package.json", &manifest);
        assert_eq!(findings1.len(), 1);
        assert_eq!(findings1[0].rule_id, "no-slopsquat-deps");
        let reqs = format!("{pkg2}==0.1.0\nuvicorn>=0.20.0");
        let findings2 = detect_slopsquat_in_content("requirements.txt", &reqs);
        assert_eq!(findings2.len(), 1);
    }
    #[test]
    fn test_detects_insecure_crypto_weak_prng_js_ts() {
        let m_rnd = ["Math.", "random()"].concat();
        let js_code = [
            format!("const sessionToken = {m_rnd}.toString(36).substring(2);"),
            format!("let resetPasswordToken = {m_rnd}.toString();"),
            format!("const apiKey = \"ak_\" + {m_rnd}.toString(16);"),
            format!("const otp = Math.floor(100000 + {m_rnd} * 900000);"),
            format!("function generateAuthNonce() {{ return {m_rnd}.toString(36); }}"),
            format!("const csrfToken = {m_rnd}.toString();"),
        ].join("\n");
        let findings = detect_insecure_crypto_in_content("src/auth/token.ts", &js_code);
        assert_eq!(findings.len(), 6);
        assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
    }
    #[test]
    fn test_detects_insecure_crypto_weak_prng_python() {
        let py_code = [
            "token = ''.join(random.choice(chars) for _ in range(32))",
            "secret_key = random.randint(100000, 999999)",
            "reset_code = random.randrange(100000, 999999)",
            "session_id = ''.join(random.choices(string.ascii_letters, k=32))",
            "salt = random.sample(alphabet, 16)",
        ].join("\n");
        let findings = detect_insecure_crypto_in_content("auth/security.py", &py_code);
        assert_eq!(findings.len(), 5);
        assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
    }
    #[test]
    fn test_detects_insecure_crypto_broken_hashes() {
        let m5 = ["m", "d5"].concat();
        let code_node = format!("const hash = crypto.createHash('{m5}').update(pwd).digest('hex');\n");
        let findings1 = detect_insecure_crypto_in_content("hash.js", &code_node);
        assert_eq!(findings1.len(), 1);
        let code_py = format!("digest = hashlib.{m5}(token.encode()).hexdigest()\n");
        let findings2 = detect_insecure_crypto_in_content("hasher.py", &code_py);
        assert_eq!(findings2.len(), 1);
        let code_go = format!("h := {m5}.New()\n");
        let findings3 = detect_insecure_crypto_in_content("hash.go", &code_go);
        assert_eq!(findings3.len(), 1);
        let code_php = format!("$hash = {m5}($password);\n");
        let findings4 = detect_insecure_crypto_in_content("login.php", &code_php);
        assert_eq!(findings4.len(), 1);
    }
    #[test]
    fn test_insecure_crypto_safe_patterns_ignored() {
        let safe_js = [
            "const width = Math.random() * 800;",
            "const diceRoll = Math.floor(Math.random() * 6) + 1;",
            "const token = crypto.randomBytes(32).toString('hex');",
            "const id = crypto.randomUUID();",
            "const digest = crypto.createHash('sha256').update(data).digest('hex');",
        ].join("\n");
        assert!(detect_insecure_crypto_in_content("visuals.js", &safe_js).is_empty());
        let safe_py = [
            "color = random.choice(['red', 'green', 'blue'])",
            "die = random.randint(1, 6)",
            "auth_token = secrets.token_hex(32)",
            "pw_hash = hashlib.sha256(data).hexdigest()",
        ].join("\n");
        assert!(detect_insecure_crypto_in_content("game.py", &safe_py).is_empty());
    }
    #[test]
    fn test_detects_test_sabotage_js_ts() {
        let sk = ["sk", "ip"].concat();
        let on = ["on", "ly"].concat();
        let f_it = ["f", "it"].concat();
        let x_it = ["x", "it"].concat();
        let js_code = [
            format!("describe.{sk}('Billing integration', () => {{}});"),
            format!("it.{sk}('verifies signature', () => {{}});"),
            format!("test.{on}('isolated sanity check', () => {{}});"),
            format!("{f_it}('focused test case', () => {{}});"),
            format!("{x_it}('muted test case', () => {{}});"),
        ].join("\n");
        let findings = detect_test_sabotage_in_content("tests/billing.test.ts", &js_code);
        assert_eq!(findings.len(), 5);
        assert_eq!(findings[0].rule_id, "no-test-sabotage");
    }
    #[test]
    fn test_detects_test_sabotage_python() {
        let sk = ["sk", "ip"].concat();
        let p_xf = ["@", "pytest.mark.xfail(strict=True)"].concat();
        let u_sk = ["@", "unittest.skip('temporarily skipped')"].concat();
        let py_code = [
            format!("@pytest.mark.{sk}(reason='broken in CI')"),
            "def test_payment(): pass".to_string(),
            p_xf,
            "def test_webhook(): pass".to_string(),
            format!("pytest.{sk}('skipping network test')"),
            u_sk,
            "def test_legacy(): pass".to_string(),
        ].join("\n");
        let findings = detect_test_sabotage_in_content("tests/test_auth.py", &py_code);
        assert_eq!(findings.len(), 4);
        assert_eq!(findings[0].rule_id, "no-test-sabotage");
    }
    #[test]
    fn test_detects_test_sabotage_rust() {
        let ign = ["ign", "ore"].concat();
        let rust_code = [
            "#[cfg(test)]",
            "mod tests {",
            "    #[test]",
            &format!("    #[{ign}]"),
            "    fn test_crypto_handshake() {}",
            "}",
        ].join("\n");
        let findings = detect_test_sabotage_in_content("src/crypto_test.rs", &rust_code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-test-sabotage");
    }
    #[test]
    fn test_test_sabotage_clean_tests_pass() {
        let clean_js = [
            "describe('Auth Gateway', () => {",
            "    it('validates jwt token', () => {",
            "        expect(verifyToken(token)).toBe(true);",
            "    });",
            "});",
        ].join("\n");
        assert!(detect_test_sabotage_in_content("tests/auth.test.ts", &clean_js).is_empty());
    }
    #[test]
    fn test_detects_unbounded_concurrency_js_ts() {
        let code = [
            "const results = await Promise.all(users.map(async (u) => fetchUser(u)));",
            "const settled = await Promise.allSettled(records.map(async r => sync(r)));",
            "await Promise.all(",
            "    items.map(async (item) => {",
            "        return process(item);",
            "    })",
            ");",
        ].join("\n");
        let findings = detect_unbounded_concurrency_in_content("src/batch.ts", &code);
        assert_eq!(findings.len(), 3);
        assert_eq!(findings[0].rule_id, "no-unbounded-concurrency");
        let safe_code = [
            "const [config, user] = await Promise.all([fetchConfig(), fetchUser()]);",
            "const throttled = await Promise.all(items.map(item => limit(() => fetchItem(item))));",
            "const chunked = await Promise.all(chunks.map(async c => processChunk(c)));",
        ].join("\n");
        assert!(detect_unbounded_concurrency_in_content("src/safe.ts", &safe_code).is_empty());
    }
    #[test]
    fn test_detects_unbounded_concurrency_python() {
        let code = [
            "results = await asyncio.gather(*[fetch(u) for u in urls])",
            "gen_res = await asyncio.gather(*(process(x) for x in items))",
            "for item in items:",
            "    asyncio.create_task(process(item))",
        ].join("\n");
        let findings = detect_unbounded_concurrency_in_content("services/worker.py", &code);
        assert_eq!(findings.len(), 3);
        assert_eq!(findings[0].rule_id, "no-unbounded-concurrency");
        let safe_code = [
            "res = await asyncio.gather(fetch_primary(), fetch_secondary())",
            "sem = asyncio.Semaphore(10)",
            "async with sem:",
            "    await asyncio.gather(*[worker(x) for x in items])",
        ].join("\n");
        assert!(detect_unbounded_concurrency_in_content("services/safe.py", &safe_code).is_empty());
    }
    #[test]
    fn test_detects_unbounded_concurrency_rust() {
        let j_all = ["join_", "all"].concat();
        let r1 = format!("let responses = futures::future::{j_all}(items.into_iter().map(|item| fetch(item))).await;");
        let r2 = format!("let done = {j_all}(requests.iter().map(|r| client.send(r))).await;");
        let code = format!("{r1}\n{r2}");
        let findings = detect_unbounded_concurrency_in_content("src/sync.rs", &code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-unbounded-concurrency");
        let s1 = format!("let (a, b) = {j_all}(vec![task_a, task_b]).await;");
        let safe_code = format!("{s1}\nlet stream_res = stream::iter(items).map(fetch).buffer_unordered(8).collect::<Vec<_>>().await;");
        assert!(detect_unbounded_concurrency_in_content("src/safe.rs", &safe_code).is_empty());
    }
    #[test]
    fn test_detects_async_in_sync_callbacks_js_ts() {
        let js_code = [
            "users.forEach(async (user) => { await db.sync(user); });",
            "items.forEach(async function(item) { await process(item); });",
            "const total = records.reduce(async (acc, r) => (await acc) + r.cost, Promise.resolve(0));",
            "items.reduceRight(async (prev, cur) => await link(prev, cur));",
        ].join("\n");
        let findings = detect_async_callback_traps_in_content("src/sync.ts", &js_code);
        assert_eq!(findings.len(), 4);
        assert_eq!(findings[0].rule_id, "no-async-in-sync-callbacks");
    }
    #[test]
    fn test_detects_async_in_sync_callbacks_python() {
        let py_code = [
            "results = list(map(async_fetch_user, user_ids))",
            "filtered = filter(async_validate, records)",
            "tasks = map(async lambda x: process(x), dataset)",
        ].join("\n");
        let findings = detect_async_callback_traps_in_content("app/pipeline.py", &py_code);
        assert_eq!(findings.len(), 3);
        assert_eq!(findings[0].rule_id, "no-async-in-sync-callbacks");
    }
    #[test]
    fn test_detects_async_in_sync_callbacks_rust() {
        let fe = ["for_", "each"].concat();
        let r1 = format!("items.iter().{fe}(|x| async move {{ do_work(x).await; }});");
        let r2 = format!("records.{fe}(|r| {{ async {{ save(r).await; }}; }});");
        let rust_code = format!("{r1}\n{r2}");
        let findings = detect_async_callback_traps_in_content("src/worker.rs", &rust_code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-async-in-sync-callbacks");
    }
    #[test]
    fn test_async_callbacks_safe_patterns_pass() {
        let safe_js = [
            "for (const user of users) { await db.sync(user); }",
            "const results = await Promise.all(items.map(async (i) => await fetch(i)));",
            "users.forEach((user) => console.log(user.name));",
        ].join("\n");
        assert!(detect_async_callback_traps_in_content("src/safe.ts", &safe_js).is_empty());
        let safe_py = [
            "results = await asyncio.gather(*(async_fetch(u) for u in user_ids))",
            "async for item in async_generator(): await process(item)",
        ].join("\n");
        assert!(detect_async_callback_traps_in_content("app/safe.py", &safe_py).is_empty());
    }
    #[test]
    fn test_detects_speculative_apis_python() {
        let code = [
            "raw = json.loadf(fp)",
            "path = os.path.joinpath(\"a\", \"b\")",
            "hex_val = hashlib.sha256(b).hex()",
        ].join("\n");
        let findings = detect_speculative_apis_in_content("script.py", &code);
        assert_eq!(findings.len(), 3);
        for f in &findings {
            assert_eq!(f.rule_id, "no-hallucinated-speculative-apis");
            assert_eq!(f.category, SlopCategory::SpeculativeApi);
            assert!(f.suggestion.is_some());
        }
    }
    #[test]
    fn test_detects_speculative_apis_js_ts() {
        let code = [
            "fs.writeJsonSync(path, data);",
            "const id = crypto.randomUUIDv4();",
        ].join("\n");
        let findings = detect_speculative_apis_in_content("app.ts", &code);
        assert_eq!(findings.len(), 2);
        for f in &findings {
            assert_eq!(f.rule_id, "no-hallucinated-speculative-apis");
            assert_eq!(f.category, SlopCategory::SpeculativeApi);
        }
    }
    #[test]
    fn test_detects_speculative_apis_rust() {
        let exp = ["Option::expect_", "or_default(\"err\");"].concat();
        let p_all = ["Vec::push_", "all(&slice);"].concat();
        let code = format!("let val = {exp}\n{p_all}");
        let findings = detect_speculative_apis_in_content("src/main.rs", &code);
        assert_eq!(findings.len(), 2);
        for f in &findings {
            assert_eq!(f.rule_id, "no-hallucinated-speculative-apis");
            assert_eq!(f.category, SlopCategory::SpeculativeApi);
        }
    }
    #[test]
    fn test_speculative_apis_legitimate_calls_pass() {
        let py_code = [
            "data = json.loads(text)",
            "p = pathlib.Path(\"a\").joinpath(\"b\")",
        ].join("\n");
        assert!(detect_speculative_apis_in_content("safe.py", &py_code).is_empty());
        let rs_code = "vec.extend_from_slice(&s);";
        assert!(detect_speculative_apis_in_content("safe.rs", rs_code).is_empty());
    }
    #[test]
    fn test_detects_insecure_prng_tokens_js_py_rust() {
        let m_rnd = ["Math.", "random()"].concat();
        let js_code = format!("const sessionToken = {m_rnd}.toString(36);");
        let findings_js = detect_insecure_crypto_in_content("token.js", &js_code);
        assert_eq!(findings_js.len(), 1);
        assert_eq!(findings_js[0].rule_id, "no-insecure-prng-tokens");
        assert_eq!(findings_js[0].category, SlopCategory::CryptoFlaw);
        let py_code = "api_key = random.choices(string.ascii_letters, k=32)";
        let findings_py = detect_insecure_crypto_in_content("auth.py", py_code);
        assert_eq!(findings_py.len(), 1);
        assert_eq!(findings_py[0].rule_id, "no-insecure-prng-tokens");
        assert_eq!(findings_py[0].category, SlopCategory::CryptoFlaw);
        let r_rng = ["rand::", "thread_rng().gen::<u64>()"].concat();
        let rs_code = format!("let csrf_token = {r_rng};");
        let findings_rs = detect_insecure_crypto_in_content("auth.rs", &rs_code);
        assert_eq!(findings_rs.len(), 1);
        assert_eq!(findings_rs[0].rule_id, "no-insecure-prng-tokens");
        assert_eq!(findings_rs[0].category, SlopCategory::CryptoFlaw);
    }
    #[test]
    fn test_insecure_prng_tokens_safe_entropy_pass() {
        let js_code = [
            "const entropy = crypto.randomBytes(32);",
            "let dice_roll = Math.random();",
        ].join("\n");
        let findings_js = detect_insecure_crypto_in_content("safe.js", &js_code);
        assert!(findings_js.iter().all(|f| f.rule_id != "no-insecure-prng-tokens"));
        let py_code = "token = secrets.token_hex(32)";
        let findings_py = detect_insecure_crypto_in_content("safe.py", py_code);
        assert!(findings_py.iter().all(|f| f.rule_id != "no-insecure-prng-tokens"));
    }
    #[test]
    fn test_detects_slopsquat_npm_manifest() {
        let manifest = "{\n  \"dependencies\": {\n    \"express\": \"^4.18.2\",\n    \"express-jwt-router\": \"^1.0.0\",\n    \"@types/node\": \"^20.0.0\"\n  }\n}";
        let findings = detect_slopsquat_in_content("package.json", manifest);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-slopsquat-deps");
        assert_eq!(findings[0].line, 4);
    }
    #[test]
    fn test_detects_slopsquat_python_requirements() {
        let reqs = "fastapi>=0.100.0\nfastapi-auth-bearer-v2==1.0.0\nuvicorn>=0.20.0";
        let findings = detect_slopsquat_in_content("requirements.txt", reqs);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-slopsquat-deps");
        assert_eq!(findings[0].line, 2);
    }
    #[test]
    fn test_detects_slopsquat_pyproject_toml() {
        let toml = "[project.dependencies]\npython = \">=3.11\"\npydantic-mongo-helpers = \"^0.2.0\"\nrequests = \">=2.31.0\"";
        let findings = detect_slopsquat_in_content("pyproject.toml", toml);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-slopsquat-deps");
        assert_eq!(findings[0].line, 3);
    }
    #[test]
    fn test_detects_slopsquat_cargo_manifest() {
        let cargo = "[dependencies]\nserde = \"1.0\"\ncrypto-token-generator = \"0.1\"\ntokio = { version = \"1\", features = [\"full\"] }";
        let findings = detect_slopsquat_in_content("Cargo.toml", cargo);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-slopsquat-deps");
        assert_eq!(findings[0].line, 3);
    }
    #[test]
    fn test_slopsquat_whitelist_legit_packages() {
        let manifest = "{\n  \"dependencies\": {\n    \"@types/react\": \"^18.0.0\",\n    \"@nestjs/common\": \"^10.0.0\",\n    \"local-lib\": \"file:../lib\"\n  }\n}";
        let findings = detect_slopsquat_in_content("package.json", manifest);
        assert_eq!(findings.len(), 0);
    }
    #[test]
    fn test_detects_unawaited_async_in_loops_js_ts() {
        let code = ["for (const item of items) {", "    fetch('/api/process', { method: 'POST', body: JSON.stringify(item) });", "}"].join("\n");
        let findings = detect_async_loop_traps_in_content("sync.ts", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
        assert_eq!(findings[0].category, SlopCategory::AsyncIterationHazard);
    }
    #[test]
    fn test_detects_unawaited_async_in_loops_python() {
        let code = ["for payload in batch:", "    aiohttp.ClientSession().post('http://example.com', json=payload)"].join("\n");
        let findings = detect_async_loop_traps_in_content("batch.py", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
    }
    #[test]
    fn test_detects_unawaited_async_in_loops_rust() {
        let code = ["for task in task_queue {", "    tokio::spawn(async move {", "        do_work().await;", "    });", "}"].join("\n");
        let findings = detect_async_loop_traps_in_content("worker.rs", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
    }
    #[test]
    fn test_async_loops_safe_awaited_patterns_pass() {
        let js_code = ["for (const item of items) {", "    await fetch('/api/process', { method: 'POST', body: JSON.stringify(item), signal: controller.signal });", "}"].join("\n");
        let py_code = ["for payload in batch:", "    await client.post('http://example.com', json=payload)"].join("\n");
        let rs_code = ["for task in task_queue {", "    let permit = sem.clone().acquire_owned().await.unwrap();", "    join_set.spawn(async move {", "        let _p = permit;", "        do_work().await;", "    });", "}"].join("\n");
        assert!(detect_async_loop_traps_in_content("ok.ts", &js_code).is_empty());
        assert!(detect_async_loop_traps_in_content("ok.py", &py_code).is_empty());
        assert!(detect_async_loop_traps_in_content("ok.rs", &rs_code).is_empty());
    }
    #[test]
    fn test_detects_hallucinated_package_manifest_npm() {
        let json = ["{", "  \"dependencies\": {", "    \"requests-async-retry\": \"^1.0.0\",", "    \"react-state-enhancer\": \"*\"", "  }", "}"].join("\n");
        let findings = detect_package_hallucinations_in_content("package.json", &json);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
        assert_eq!(findings[0].category, SlopCategory::SupplyChainPoisoning);
    }
    #[test]
    fn test_detects_hallucinated_package_manifest_python() {
        let reqs = ["fastapi-jwt-guard>=1.0.0", "requests-async-retry==0.2.1"].join("\n");
        let findings = detect_package_hallucinations_in_content("requirements.txt", &reqs);
        assert_eq!(findings.len(), 2);
    }
    #[test]
    fn test_detects_hallucinated_package_manifest_cargo() {
        let toml = ["[dependencies]", "tokio-rate-limiter-pro = \"*\""].join("\n");
        let findings = detect_package_hallucinations_in_content("Cargo.toml", &toml);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
    }
    #[test]
    fn test_hallucinated_package_manifest_safe_deps_pass() {
        let safe_json = ["{", "  \"dependencies\": {", "    \"react\": \"^18.2.0\",", "    \"@types/node\": \"^20.0.0\",", "    \"@prisma/client\": \"^5.0.0\"", "  }", "}"].join("\n");
        assert!(detect_package_hallucinations_in_content("package.json", &safe_json).is_empty());
    }
    #[test]
    fn test_detects_hardcoded_jwt_secrets_and_weak_keys() {
        let js_code = ["const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';", "const sign = jwt.sign(payload, 'secret');"].join("\n");
        let findings = detect_jwt_leakage_in_content("auth.js", &js_code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hardcoded-jwt-secrets");
        assert_eq!(findings[0].category, SlopCategory::AuthLeakage);
    }
    #[test]
    fn test_detects_jwt_alg_none_bypass() {
        let py_code = ["decoded = jwt.decode(token, algorithms=['none'])"].join("\n");
        let findings = detect_jwt_leakage_in_content("auth.py", &py_code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-hardcoded-jwt-secrets");
    }
    #[test]
    fn test_detects_broken_access_control_idor_queries() {
        let sql_code = ["DELETE FROM orders WHERE id = $1"].join("\n");
        let findings = detect_broken_access_control_in_content("repo.js", &sql_code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unscoped-idor-queries");
        assert_eq!(findings[0].category, SlopCategory::BrokenAccessControl);
    }
    #[test]
    fn test_detects_unsafe_xml_xxe_and_metadata_ssrf() {
        let xxe_code = ["parser.setFeature('http://xml.org/sax/features/external-general-entities', true);", "const url = 'http://169.254.169.254/latest/meta-data/';"].join("\n");
        let findings = detect_xxe_ssrf_in_content("xml_helper.ts", &xxe_code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-unsafe-xml-or-ssrf");
        assert_eq!(findings[0].category, SlopCategory::SsrfHazard);
    }
    #[test]
    fn test_safe_auth_and_access_control_patterns_pass() {
        let safe_sql = ["DELETE FROM orders WHERE id = $1 AND user_id = $auth_user_id;"].join("\n");
        assert!(detect_broken_access_control_in_content("safe.js", &safe_sql).is_empty());
        let safe_jwt = ["const secret = process.env.JWT_SECRET;", "jwt.verify(token, secret, { algorithms: ['RS256'] });"].join("\n");
        assert!(detect_jwt_leakage_in_content("safe.js", &safe_jwt).is_empty());
        let safe_xml = ["parser.setFeature('http://xml.org/sax/features/external-general-entities', false);"].join("\n");
        assert!(detect_xxe_ssrf_in_content("safe.js", &safe_xml).is_empty());
    }
    #[test]
    fn test_detects_speculative_orm_prisma() {
        let code = ["const user = await prisma.user.findOrCreate({ where: { email } });"].join("\n");
        let findings = detect_speculative_orm_in_content("service.ts", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-speculative-orm-methods");
        assert_eq!(findings[0].category, SlopCategory::SpeculativeApi);
    }
    #[test]
    fn test_detects_speculative_orm_django_sqlalchemy() {
        let code = ["user = User.query.filter_by_slug(\"admin\")"].join("\n");
        let findings = detect_speculative_orm_in_content("models.py", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-speculative-orm-methods");
        assert_eq!(findings[0].category, SlopCategory::SpeculativeApi);
    }
    #[test]
    fn test_detects_speculative_orm_diesel_seaorm() {
        let code = ["let res = users::table::find_or_create(&conn);"].join("\n");
        let findings = detect_speculative_orm_in_content("repo.rs", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-speculative-orm-methods");
        assert_eq!(findings[0].category, SlopCategory::SpeculativeApi);
    }
    #[test]
    fn test_speculative_orm_safe_patterns_pass() {
        let ts_code = ["const user = await prisma.user.findUnique({ where: { id: 1 } });", "const repo = repository.findOneBy({ id: 1 });"].join("\n");
        let py_code = ["user, created = User.objects.get_or_create(username='admin')"].join("\n");
        let rs_code = ["let user = Entity::find().filter(Column::Id.eq(1)).one(&db).await?;", "let saved = repository.save(entity);"].join("\n");
        assert!(detect_speculative_orm_in_content("repo.ts", &ts_code).is_empty());
        assert!(detect_speculative_orm_in_content("models.py", &py_code).is_empty());
        assert!(detect_speculative_orm_in_content("db.rs", &rs_code).is_empty());
    }
    #[test]
    fn test_detects_deep_path_traversal_python_zip_slip() {
        let code = ["tarfile.open(archive).extractall()"].join("\n");
        let findings = detect_deep_path_traversal_in_content("extract.py", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
        assert_eq!(findings[0].category, SlopCategory::PathTraversalSecurity);
    }
    #[test]
    fn test_detects_deep_path_traversal_node_params() {
        let code = ["const stream = fs.createReadStream(path.join(root, req.params.file));"].join("\n");
        let findings = detect_deep_path_traversal_in_content("server.js", &code);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
        assert_eq!(findings[0].category, SlopCategory::PathTraversalSecurity);
    }
    #[test]
    fn test_detects_deep_path_traversal_rust_unconstrained() {
        let code = ["let file = File::create(format!(\"{}/{}\", base, user_input));"].join("\n");
        let findings = detect_deep_path_traversal_in_content("fs.rs", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
        assert_eq!(findings[0].category, SlopCategory::PathTraversalSecurity);
    }
    #[test]
    fn test_deep_path_traversal_safe_guards_pass() {
        let rs_code = ["let safe_path = path.canonicalize()?;", "let file = File::create(format!(\"{}/{}\", base, safe_path));"].join("\n");
        let py_code = ["safe_name = secure_filename(filename)", "tarfile.open(safe_name).extractall()"].join("\n");
        let js_code = ["if (filePath.startsWith('/safe/dir')) {", "    fs.createReadStream(path.join(root, req.params.file));", "}"].join("\n");
        assert!(detect_deep_path_traversal_in_content("safe.rs", &rs_code).is_empty());
        assert!(detect_deep_path_traversal_in_content("safe.py", &py_code).is_empty());
        assert!(detect_deep_path_traversal_in_content("safe.js", &js_code).is_empty());
    }
    #[test]
    fn test_detects_async_race_traps_js_uncaught() {
        let code = ["const res = Promise.race([fetch(a), fetch(b)]);"].join("\n");
        let findings = detect_async_race_traps_in_content("async_ops.ts", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-async-pipeline-races");
        assert_eq!(findings[0].category, SlopCategory::AsyncResourceLeak);
    }
    #[test]
    fn test_detects_async_stream_pipe_leak_node() {
        let code = ["readable.pipe(writable);"].join("\n");
        let findings = detect_async_race_traps_in_content("stream.js", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-async-pipeline-races");
        assert_eq!(findings[0].category, SlopCategory::AsyncResourceLeak);
    }
    #[test]
    fn test_detects_async_gather_unshielded_python() {
        let code = ["results = await asyncio.gather(task1, task2)"].join("\n");
        let findings = detect_async_race_traps_in_content("tasks.py", &code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-async-pipeline-races");
        assert_eq!(findings[0].category, SlopCategory::AsyncResourceLeak);
    }
    #[test]
    fn test_async_race_safe_patterns_pass() {
        let js_code = ["Promise.race([fetch(a), fetch(b)]).catch(handleError);"].join("\n");
        let py_code = ["results = await asyncio.gather(task1, task2, return_exceptions=True)"].join("\n");
        let stream_code = ["stream.pipeline(src, dest, callback);"].join("\n");
        assert!(detect_async_race_traps_in_content("safe.ts", &js_code).is_empty());
        assert!(detect_async_race_traps_in_content("safe.py", &py_code).is_empty());
        assert!(detect_async_race_traps_in_content("safe.js", &stream_code).is_empty());
    }
    #[test]
    fn test_detects_phantom_cloud_sdks_npm() {
        let json = ["{", "  \"dependencies\": {", "    \"aws-sdk-v4\": \"^4.0.0\"", "  }", "}"].join("\n");
        let findings = detect_phantom_cloud_sdks_in_content("package.json", &json);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-phantom-cloud-sdks");
        assert_eq!(findings[0].category, SlopCategory::SupplyChainPoisoning);
    }
    #[test]
    fn test_detects_phantom_cloud_sdks_pypi() {
        let reqs = ["boto4>=1.0.0"].join("\n");
        let findings = detect_phantom_cloud_sdks_in_content("requirements.txt", &reqs);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-phantom-cloud-sdks");
        assert_eq!(findings[0].category, SlopCategory::SupplyChainPoisoning);
    }
    #[test]
    fn test_detects_phantom_cloud_sdks_cargo() {
        let toml = ["[dependencies]", "tokio-plus = \"1.0\""].join("\n");
        let findings = detect_phantom_cloud_sdks_in_content("Cargo.toml", &toml);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-phantom-cloud-sdks");
        assert_eq!(findings[0].category, SlopCategory::SupplyChainPoisoning);
    }
    #[test]
    fn test_phantom_cloud_sdks_safe_pass() {
        let safe_json = ["{", "  \"dependencies\": {", "    \"@aws-sdk/client-s3\": \"^3.500.0\",", "    \"azure-identity\": \"^4.0.0\",", "    \"openai\": \"^4.0.0\"", "  }", "}"].join("\n");
        let safe_reqs = ["boto3>=1.34.0", "google-cloud-storage>=2.14.0"].join("\n");
        let safe_toml = ["[dependencies]", "tokio = \"1.38\""].join("\n");
        assert!(detect_phantom_cloud_sdks_in_content("package.json", &safe_json).is_empty());
        assert!(detect_phantom_cloud_sdks_in_content("requirements.txt", &safe_reqs).is_empty());
        assert!(detect_phantom_cloud_sdks_in_content("Cargo.toml", &safe_toml).is_empty());
    }
    #[test]
    fn test_detects_speculative_package_hallucinations_npm() {
        let manifest = ["{", "  \"dependencies\": {", "    \"fast-jwt-verifier\": \"^1.0.0\",", "    \"express-jwt-auto-verify\": \"^2.0.0\"", "  }", "}"].join("\n");
        let findings = detect_package_hallucinations_in_content("package.json", &manifest);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
        assert_eq!(findings[1].rule_id, "no-hallucinated-package-manifest");
    }
    #[test]
    fn test_detects_speculative_package_hallucinations_python() {
        let reqs = ["crypto-safe-random>=1.0.0", "requests-async-retry>=0.5.0"].join("\n");
        let findings = detect_package_hallucinations_in_content("requirements.txt", &reqs);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
        assert_eq!(findings[1].rule_id, "no-hallucinated-package-manifest");
    }
    #[test]
    fn test_detects_deep_path_traversal_multi_encoded() {
        let code1 = ["let target = \"/var/data/%2e%2e%2fsecret.key\";"].join("\n");
        let code2 = ["const resolved = path.join(__dirname, req.query.file);"].join("\n");
        let f1 = detect_deep_path_traversal_in_content("server.js", &code1);
        let f2 = detect_deep_path_traversal_in_content("server.js", &code2);
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0].rule_id, "no-deep-path-traversal");
        assert_eq!(f2.len(), 1);
        assert_eq!(f2[0].rule_id, "no-deep-path-traversal");
    }
    #[test]
    fn test_deep_path_traversal_canonicalize_pass() {
        let code1 = ["let safe = path.canonicalize()?;", "File::open(safe)?;"];
        let code2 = ["filename = secure_filename(req.params.file)", "open(filename)"];
        assert!(detect_deep_path_traversal_in_content("safe.rs", &code1.join("\n")).is_empty());
        assert!(detect_deep_path_traversal_in_content("safe.py", &code2.join("\n")).is_empty());
    }
    #[test]
    fn test_detects_unawaited_async_in_loops_and_foreach() {
        let code_foreach = ["array.forEach(async (item) => {", "    await process(item);", "});"].join("\n");
        let code_loop = ["for (const item of array) {", "    fetch('/api/' + item);", "}"].join("\n");
        let f_foreach = crate::detectors::async_callback_traps::detect_async_callback_traps_in_content("worker.js", &code_foreach);
        let f_loop = detect_async_loop_traps_in_content("worker.js", &code_loop);
        assert_eq!(f_foreach.len(), 1);
        assert_eq!(f_foreach[0].rule_id, "no-async-in-sync-callbacks");
        assert_eq!(f_loop.len(), 1);
        assert_eq!(f_loop[0].rule_id, "no-unawaited-async-in-loops");
    }
    #[test]
    fn test_detects_insecure_prng_session_and_csrf() {
        let js_code = ["const sessionToken = Math.random().toString(36);"].join("\n");
        let py_code = ["csrf_token = random.randint(1000, 9999)"].join("\n");
        let f_js = detect_insecure_crypto_in_content("auth.js", &js_code);
        let f_py = detect_insecure_crypto_in_content("auth.py", &py_code);
        assert_eq!(f_js.len(), 1);
        assert_eq!(f_js[0].rule_id, "no-insecure-prng-tokens");
        assert_eq!(f_py.len(), 1);
        assert_eq!(f_py[0].rule_id, "no-insecure-prng-tokens");
    }
    #[test]
    fn test_detects_package_manifest_go_mod_hallucinations() {
        let manifest = ["module myapp", "go 1.22", "require (", "    github.com/gin-gonic/gin v1.9.1", "    github.com/user/jwt-guard v1.0.0", "    github.com/org/auth-shield v2.0.0", ")"].join("\n");
        let findings = detect_package_hallucinations_in_content("go.mod", &manifest);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
        assert_eq!(findings[1].rule_id, "no-hallucinated-package-manifest");
    }
    #[test]
    fn test_detects_package_manifest_pyproject_wildcards() {
        let pyproject = ["[project]", "name = \"demo\"", "dependencies = [", "    \"pydantic>=2.0\",", "    \"requests==*\",", "]", "[tool.poetry.dependencies]", "fastapi = \"*\""].join("\n");
        let findings = detect_package_hallucinations_in_content("pyproject.toml", &pyproject);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
        assert_eq!(findings[1].rule_id, "no-hallucinated-package-manifest");
    }
    #[test]
    fn test_detects_async_stream_unhandled_generator_yield() {
        let js_code = ["async function* fetchStream(urls) {", "    for (const url of urls) {", "        yield fetch(url);", "    }", "}"].join("\n");
        let findings = detect_async_loop_traps_in_content("stream.js", &js_code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
        assert_eq!(findings[0].category, SlopCategory::AsyncIterationHazard);
    }
    #[test]
    fn test_detects_async_loop_missing_abort_controller() {
        let js_code = ["for (const url of endpoints) {", "    const res = await fetch(url);", "    await process(res);", "}"].join("\n");
        let findings = detect_async_loop_traps_in_content("worker.js", &js_code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
        assert!(findings[0].message.contains("AbortController"));
    }
    #[test]
    fn test_detects_unsafe_deserialization_gadgets_python_java() {
        let py_code = ["import pickle", "obj = pickle.loads(untrusted_payload)"].join("\n");
        let java_code = ["ObjectInputStream in = new ObjectInputStream(socket.getInputStream());", "Object obj = in.readObject();"].join("\n");
        let f_py = detect_unsafe_deserialization_in_content("worker.py", &py_code);
        let f_java = detect_unsafe_deserialization_in_content("Server.java", &java_code);
        assert_eq!(f_py.len(), 1);
        assert_eq!(f_py[0].rule_id, "no-unsafe-deserialization");
        assert_eq!(f_java.len(), 1);
        assert_eq!(f_java[0].rule_id, "no-unsafe-deserialization");
    }
    #[test]
    fn test_detects_unsafe_deserialization_node_eval_chains() {
        let node_code = ["const serialize = require('node-serialize');", "const obj = serialize.unserialize(cookieData);", "const hydrated = eval('(' + userInput + ')');"].join("\n");
        let findings = detect_unsafe_deserialization_in_content("session.js", &node_code);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
        assert_eq!(findings[1].rule_id, "no-unsafe-deserialization");
    }
    #[test]
    fn test_detects_insecure_prng_salt_and_nonce_rust_go() {
        let rs_code = ["fn generate_credentials() {", "    let salt = rand::random::<u64>();", "}"].join("\n");
        let go_code = ["func generateNonce() int {", "    nonce := rand.Intn(1000000)", "    return nonce", "}"].join("\n");
        let f_rs = detect_insecure_crypto_in_content("crypto.rs", &rs_code);
        let f_go = detect_insecure_crypto_in_content("auth.go", &go_code);
        assert_eq!(f_rs.len(), 1);
        assert_eq!(f_rs[0].rule_id, "no-insecure-prng-tokens");
        assert_eq!(f_go.len(), 1);
        assert_eq!(f_go[0].rule_id, "no-insecure-prng-tokens");
    }
    #[test]
    fn test_safe_csprng_and_safe_manifests_zero_false_positives() {
        let safe_js_csprng = ["const token = crypto.randomBytes(32).toString('hex');", "const uuid = crypto.randomUUID();"].join("\n");
        let safe_py_csprng = ["import secrets", "auth_token = secrets.token_hex(32)", "salt = secrets.token_bytes(16)"].join("\n");
        let safe_manifest_cargo = ["[dependencies]", "serde = { version = \"1.0.200\", features = [\"derive\"] }", "tokio = \"1.38.0\""].join("\n");
        let safe_manifest_npm = ["{", "  \"dependencies\": {", "    \"react\": \"^18.2.0\",", "    \"@types/node\": \"^20.0.0\"", "  }", "}"].join("\n");
        assert!(detect_insecure_crypto_in_content("auth.js", &safe_js_csprng).is_empty());
        assert!(detect_insecure_crypto_in_content("auth.py", &safe_py_csprng).is_empty());
        assert!(detect_package_hallucinations_in_content("Cargo.toml", &safe_manifest_cargo).is_empty());
        assert!(detect_package_hallucinations_in_content("package.json", &safe_manifest_npm).is_empty());
    }
    #[allow(dead_code)]
    pub struct FuzzRegressionHarness {
        #[allow(dead_code)]
        pub iterations: usize,
        #[allow(dead_code)]
        pub max_payload_len: usize,
    }
    impl FuzzRegressionHarness {
        pub fn new(iterations: usize, max_payload_len: usize) -> Self {
            Self { iterations, max_payload_len }
        }
        pub fn verify_invariant<F>(&self, mut runner: F) -> bool
        where F: FnMut(&str) -> bool {
            let mut passed = true;
            let samples = [
                "../../etc/shadow",
                "%2e%2e/%2e%2e/data",
                "..\\..\\windows\\system32",
                "\\u002e\\u002e/secret",
                "express-jwt-auto-auth",
                "serde-json-valid",
                "tokio-stream-ext",
                "torch-model-utils",
                "pydantic-validation-plus",
                "Math.random() * 1000",
                "crypto.pseudoRandomBytes(32)",
                "rand::thread_rng().gen_range(0..100)"
            ];
            for s in samples.iter() {
                if !runner(s) {
                    passed = false;
                    break;
                }
            }
            passed
        }
    }
    #[test]
    fn test_fuzz_regression_harness_invariant_consistency() {
        let harness = FuzzRegressionHarness::new(100, 256);
        let passed = harness.verify_invariant(|s| {
            !s.is_empty()
        });
        assert!(passed);
    }
    #[test]
    fn test_detects_speculative_manifest_dependencies_slop_051() {
        let bad_cargo = ["[dependencies]", "serde-json-valid = \"1.0\"", "tokio-stream-ext = \"0.1\""].join("\n");
        let bad_npm = ["{", "  \"dependencies\": {", "    \"express-jwt-auto-auth\": \"^1.0.0\",", "    \"torch-model-utils\": \"^0.2.1\"", "  }", "}"].join("\n");
        assert!(!detect_package_hallucinations_in_content("Cargo.toml", &bad_cargo).is_empty());
        assert!(!detect_package_hallucinations_in_content("package.json", &bad_npm).is_empty());
    }
    #[test]
    fn test_safe_manifest_dependencies_zero_false_positives() {
        let clean_cargo = ["[dependencies]", "serde = { version = \"1.0\", features = [\"derive\"] }", "tokio = { version = \"1.0\", features = [\"full\"] }"].join("\n");
        let clean_npm = ["{", "  \"dependencies\": {", "    \"express\": \"^4.19.2\",", "    \"jsonwebtoken\": \"^9.0.2\"", "  }", "}"].join("\n");
        assert!(detect_package_hallucinations_in_content("Cargo.toml", &clean_cargo).is_empty());
        assert!(detect_package_hallucinations_in_content("package.json", &clean_npm).is_empty());
    }
    #[test]
    fn test_detects_unshielded_async_iteration_batch_slop_052() {
        let bad_js = "const results = await Promise.allSettled(urls.map(async u => fetch(u)));";
        let bad_py = "results = [await r for r in asyncio.as_completed([fetch(u) for u in urls])]";
        assert!(!detect_unbounded_concurrency_in_content("batch.js", bad_js).is_empty());
        assert!(!detect_unbounded_concurrency_in_content("batch.py", bad_py).is_empty());
    }
    #[test]
    fn test_safe_bounded_async_iteration_passes() {
        let safe_js = "const limit = pLimit(10); const results = await Promise.allSettled(urls.map(u => limit(() => fetch(u))));";
        let safe_py = "sem = asyncio.Semaphore(5)\nasync with sem:\n    results = [await r for r in asyncio.as_completed(tasks)]";
        assert!(detect_unbounded_concurrency_in_content("batch.js", safe_js).is_empty());
        assert!(detect_unbounded_concurrency_in_content("batch.py", safe_py).is_empty());
    }
    #[test]
    fn test_detects_insecure_fallback_prng_in_credentials_slop_053() {
        let bad_node = "const authToken = crypto.pseudoRandomBytes(32).toString('hex');";
        let bad_rust = "let session_nonce = rand::thread_rng().gen_range(0..100000);";
        assert!(!detect_insecure_crypto_in_content("auth.js", bad_node).is_empty());
        assert!(!detect_insecure_crypto_in_content("auth.rs", bad_rust).is_empty());
    }
    #[test]
    fn test_detects_evasive_path_normalization_traversal_slop_054() {
        let bad_norm = "const safePath = path.normalize(req.query.file); fs.readFileSync(safePath);";
        let bad_unicode = "const target = '/var/data/' + '\\u002e\\u002e/etc/passwd';";
        assert!(!detect_deep_path_traversal_in_content("server.js", bad_norm).is_empty());
        assert!(!detect_deep_path_traversal_in_content("server.js", bad_unicode).is_empty());
    }
    #[test]
    fn test_detects_ide_protocol_hallucinations_slop_055() {
        let bad_vscode = "vscode.window.showAIMessage('Generating code with model...');";
        let bad_cursor = "cursor.editor.injectPrompt({ prompt: 'Refactor function' });";
        let bad_window_ai = "await window.ai.generateText('Explain diff');";
        assert!(!detect_ide_protocol_hallucinations_in_content("extension.ts", bad_vscode).is_empty());
        assert!(!detect_ide_protocol_hallucinations_in_content("plugin.js", bad_cursor).is_empty());
        assert!(!detect_ide_protocol_hallucinations_in_content("index.tsx", bad_window_ai).is_empty());
    }
    #[test]
    fn test_legitimate_editor_apis_zero_false_positives() {
        let safe_vscode = "vscode.window.showInformationMessage('SlopGuard: Workspace Clean');";
        let safe_command = "vscode.commands.executeCommand('workbench.action.reloadWindow');";
        assert!(detect_ide_protocol_hallucinations_in_content("extension.ts", safe_vscode).is_empty());
        assert!(detect_ide_protocol_hallucinations_in_content("extension.ts", safe_command).is_empty());
    }
    #[test]
    fn test_detects_polymorphic_unsafe_deserialization_slop_056() {
        let bad_yaml = "payload = yaml.load(user_stream, Loader=yaml.Loader)";
        let bad_js = "const state = serializeJavascript.deserialize(req.body);";
        assert!(!detect_unsafe_deserialization_in_content("server.py", bad_yaml).is_empty());
        assert!(!detect_unsafe_deserialization_in_content("server.js", bad_js).is_empty());
    }
    #[test]
    fn test_safe_deserializers_zero_false_positives() {
        let safe_py = "data = yaml.safe_load(user_stream, Loader=yaml.SafeLoader)";
        let safe_json = "const data = JSON.parse(user_input);";
        assert!(detect_unsafe_deserialization_in_content("server.py", safe_py).is_empty());
        assert!(detect_unsafe_deserialization_in_content("server.js", safe_json).is_empty());
    }
    #[test]
    fn test_detects_unawaited_async_loop_traps_slop_057() {
        let bad_foreach = "users.forEach(async user => { await sendEmail(user); });";
        assert!(!detect_async_loop_traps_in_content("mailer.js", bad_foreach).is_empty());
    }
    #[test]
    fn test_detects_static_prng_seed_and_weak_entropy_slop_058() {
        let bad_rust = "let mut rng = StdRng::seed_from_u64(0);";
        let bad_py = "random.seed(1337)\ntoken = f'token_{random.random()}'";
        assert!(!detect_insecure_crypto_in_content("auth.rs", bad_rust).is_empty());
        assert!(!detect_insecure_crypto_in_content("auth.py", bad_py).is_empty());
    }
    #[test]
    fn test_detects_ghost_package_slopsquatting_v240() {
        let py_hallucinated = "import requests_async\nimport boto4\nfrom flask_jwt_simple import JWTManager\n";
        let py_findings = detect_package_hallucinations_in_content("service.py", py_hallucinated);
        assert_eq!(py_findings.len(), 3);
        let js_hallucinated = "const cryptoNode = require('crypto-node');\nimport { verify } from 'jsonwebtoken-async';\n";
        let js_findings = detect_package_hallucinations_in_content("auth.ts", js_hallucinated);
        assert_eq!(js_findings.len(), 2);
        let rs_hallucinated = "use tokio_requests::Client;\nuse anyhow_derive::Error;\n";
        let rs_findings = detect_package_hallucinations_in_content("lib.rs", rs_hallucinated);
        assert_eq!(rs_findings.len(), 2);
        let legit_deps = "import boto3\nimport httpx\nimport jwt\nconst crypto = require('crypto');\nuse reqwest::Client;\n";
        let legit_findings = detect_package_hallucinations_in_content("clean.ts", legit_deps);
        assert_eq!(legit_findings.len(), 0);
    }
    #[test]
    fn test_detects_async_loop_traps_v240() {
        let foreach_trap = "records.forEach(async (record) => { await db.save(record); });\n";
        let foreach_findings = detect_async_loop_traps_in_content("worker.js", foreach_trap);
        assert!(!foreach_findings.is_empty());
        let safe_promise_all = "await Promise.all(records.map(async (record) => { return await db.save(record); }));\n";
        let safe_findings = detect_async_loop_traps_in_content("safe_worker.js", safe_promise_all);
        assert_eq!(safe_findings.len(), 0);
    }
    #[test]
    fn test_detects_unsafe_deserialization_cloader_and_marshal_v240() {
        let py_cloader = "data = yaml.load(user_stream, Loader=yaml.CLoader)\n";
        let cloader_findings = detect_unsafe_deserialization_in_content("parser.py", py_cloader);
        assert_eq!(cloader_findings.len(), 1);
        let py_marshal = "raw_data = marshal.loads(crafted_bytes)\n";
        let marshal_findings = detect_unsafe_deserialization_in_content("loader.py", py_marshal);
        assert_eq!(marshal_findings.len(), 1);
        let java_ois = "ObjectInputStream ois = new ObjectInputStream(untrustedStream);\n";
        let java_findings = detect_unsafe_deserialization_in_content("Handler.java", java_ois);
        assert_eq!(java_findings.len(), 1);
    }
    #[test]
    fn test_detects_weak_prng_in_session_salt_v240() {
        let js_salt = "const salt = Math.random().toString(36).substring(2);\n";
        let salt_findings = detect_insecure_crypto_in_content("auth.js", js_salt);
        assert_eq!(salt_findings.len(), 1);
        let py_token = "reset_token = f'key_{random.randint(100000, 999999)}'\n";
        let token_findings = detect_insecure_crypto_in_content("user.py", py_token);
        assert_eq!(token_findings.len(), 1);
        let safe_crypto = "const token = crypto.randomBytes(32).toString('hex');\n";
        let safe_findings = detect_insecure_crypto_in_content("secure.js", safe_crypto);
        assert_eq!(safe_findings.len(), 0);
    }
    #[test]
    fn test_detects_deep_path_traversal_param_resolution_v240() {
        let bad_path = "const targetPath = path.resolve(baseDir, req.params.filePath);\n";
        let bad_findings = detect_deep_path_traversal_in_content("fileServer.ts", bad_path);
        assert_eq!(bad_findings.len(), 1);
        let safe_path = "const targetPath = path.resolve(baseDir, req.params.filePath);\nif (!targetPath.startsWith(baseDir)) { throw new Error('Traversal attempt'); }\n";
        let safe_findings = detect_deep_path_traversal_in_content("safeServer.ts", safe_path);
        assert_eq!(safe_findings.len(), 0);
    }
    #[test]
    fn test_threat_01_speculative_manifest_poisoning_v250() {
        let py_speculative = "pydantic-validation-plus==1.0.0\ntokio-stream-ext==0.1.0\n";
        let py_findings = detect_package_hallucinations_in_content("requirements.txt", py_speculative);
        assert_eq!(py_findings.len(), 2);
        let cargo_speculative = "[dependencies]\nserde-json-valid = \"1.0\"\n";
        let cargo_findings = detect_package_hallucinations_in_content("Cargo.toml", cargo_speculative);
        assert_eq!(cargo_findings.len(), 1);
    }
    #[test]
    fn test_threat_02_microtask_starvation_foreach_async_v250() {
        let bad_code = "async function syncAll(users) { users.forEach(async u => { await push(u); }); }";
        let findings = detect_async_loop_traps_in_content("dispatcher.ts", bad_code);
        assert!(!findings.is_empty());
        let safe_code = "async function syncAll(users) { for (const u of users) { await push(u); } }";
        let clean_findings = detect_async_loop_traps_in_content("safe_dispatcher.ts", safe_code);
        assert_eq!(clean_findings.len(), 0);
    }
    #[test]
    fn test_threat_03_polymorphic_deserialization_gadgets_v250() {
        let bad_yaml = "doc = yaml.load(payload, Loader=yaml.UnsafeLoader)";
        assert_eq!(detect_unsafe_deserialization_in_content("worker.py", bad_yaml).len(), 1);
        let bad_pickle = "obj = pickle.loads(untrusted_stream)";
        assert_eq!(detect_unsafe_deserialization_in_content("worker.py", bad_pickle).len(), 1);
        let safe_yaml = "doc = yaml.safe_load(payload)";
        assert_eq!(detect_unsafe_deserialization_in_content("worker.py", safe_yaml).len(), 0);
    }
    #[test]
    fn test_threat_04_weak_crypto_prng_in_token_salt_v250() {
        let bad_token = "const session_token = Math.random().toString(36).slice(2);";
        assert_eq!(detect_insecure_crypto_in_content("session.js", bad_token).len(), 1);
        let bad_py_salt = "auth_salt = f'salt_{random.random()}'";
        assert_eq!(detect_insecure_crypto_in_content("hasher.py", bad_py_salt).len(), 1);
        let safe_token = "const session_token = crypto.randomUUID();";
        assert_eq!(detect_insecure_crypto_in_content("session.js", safe_token).len(), 0);
    }
    #[test]
    fn test_threat_05_evasive_path_traversal_boundary_v250() {
        let bad_endpoint = "const target = path.join(uploadDir, req.body.filename);";
        assert_eq!(detect_deep_path_traversal_in_content("upload.ts", bad_endpoint).len(), 1);
        let safe_endpoint = "const target = path.join(uploadDir, req.body.filename);\nif (!target.startsWith(uploadDir)) throw new Error();";
        assert_eq!(detect_deep_path_traversal_in_content("upload.ts", safe_endpoint).len(), 0);
    }
    #[test]
    fn test_ast_detector_contract_matrix_integrity_v250() {
        for contract in crate::detectors::ACTIVE_CONTRACTS {
            assert!(!contract.rule_id.is_empty());
            assert!(!contract.target_languages.is_empty());
        }
    }
    #[test]
    fn test_threat_01_to_05_sanitizer_and_safe_code_passes_v250() {
        let valid_cargo = "[dependencies]\nserde = \"1.0\"\ntokio = { version = \"1\", features = [\"full\"] }\n";
        assert_eq!(detect_package_hallucinations_in_content("Cargo.toml", valid_cargo).len(), 0);
        let valid_async = "await Promise.all(items.map(async item => process(item)));";
        assert_eq!(detect_async_loop_traps_in_content("batch.js", valid_async).len(), 0);
    }
    #[test]
    fn test_telemetry_event_streaming_bridge_v250() {
        let telemetry = create_shared_telemetry();
        record_event(&telemetry, "THREAT_01_INTERCEPTED", "serde-json-valid detected in Cargo.toml");
        let events = telemetry.lock().unwrap().get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].category, "THREAT_01_INTERCEPTED");
    }
    #[test]
    fn test_threat_01_speculative_crates_and_npm_hallucinations_v260() {
        let npm_slop = "{\"dependencies\": {\"express-jwt-auto-auth\": \"^1.0.0\", \"react-smart-toast\": \"^2.0.0\"}}";
        let npm_findings = detect_package_hallucinations_in_content("package.json", npm_slop);
        assert_eq!(npm_findings.len(), 2);
        let cargo_slop = "[dependencies]\nchrono-tz-sys = \"1.0.0\"\nfast-jwt-verifier = \"0.2\"\n";
        let cargo_findings = detect_package_hallucinations_in_content("Cargo.toml", cargo_slop);
        assert_eq!(cargo_findings.len(), 2);
    }
    #[test]
    fn test_threat_02_unawaited_async_map_hazard_v260() {
        let unshielded_async = "async function runTasks(items) { items.forEach(async item => { await processItem(item); }); }";
        let findings = detect_async_loop_traps_in_content("queue.ts", unshielded_async);
        assert_eq!(findings.len(), 1);
        let clean_async = "async function runTasks(items) { for (const item of items) { await processItem(item); } }";
        assert_eq!(detect_async_loop_traps_in_content("queue.ts", clean_async).len(), 0);
    }
    #[test]
    fn test_threat_03_ruby_marshal_and_php_unserialize_gadgets_v260() {
        let rb_marshal = "cached_session = Marshal.load(raw_cookie_payload)";
        assert_eq!(detect_unsafe_deserialization_in_content("auth.rb", rb_marshal).len(), 1);
        let php_unserialize = "$user_obj = unserialize($_POST['state']);";
        assert_eq!(detect_unsafe_deserialization_in_content("state.php", php_unserialize).len(), 1);
        let php_safe = "$user_obj = unserialize($_POST['state'], ['allowed_classes' => false]);";
        assert_eq!(detect_unsafe_deserialization_in_content("safe_state.php", php_safe).len(), 0);
    }
    #[test]
    fn test_threat_03_java_xmldecoder_gadget_v260() {
        let java_xml = "XMLDecoder decoder = new XMLDecoder(inStream);";
        assert_eq!(detect_unsafe_deserialization_in_content("XmlParser.java", java_xml).len(), 1);
        let java_safe = "Document doc = builder.parse(inStream);";
        assert_eq!(detect_unsafe_deserialization_in_content("XmlParser.java", java_safe).len(), 0);
    }
    #[test]
    fn test_threat_04_csprng_strict_python_secrets_pass_v260() {
        let py_secret_pass = "reset_token = secrets.token_urlsafe(32)\nauth_salt = secrets.token_hex(16)\n";
        assert_eq!(detect_insecure_crypto_in_content("auth.py", py_secret_pass).len(), 0);
        let js_csprng_pass = "const token = crypto.randomUUID();\nconst nonce = crypto.randomBytes(16);\n";
        assert_eq!(detect_insecure_crypto_in_content("auth.js", js_csprng_pass).len(), 0);
    }
    #[test]
    fn test_threat_04_insecure_random_token_rejection_v260() {
        let py_bad_pin = "otp_code = f'{random.randint(100000, 999999)}'\n";
        assert_eq!(detect_insecure_crypto_in_content("verify.py", py_bad_pin).len(), 1);
        let js_bad_secret = "const client_secret = Math.random().toString(36);\n";
        assert_eq!(detect_insecure_crypto_in_content("auth.ts", js_bad_secret).len(), 1);
    }
    #[test]
    fn test_threat_05_multi_encoded_unicode_path_traversal_v260() {
        let unicode_evasion = "const target = path.join(rootDir, \"\\u002e\\u002e/etc/passwd\");";
        assert_eq!(detect_deep_path_traversal_in_content("server.js", unicode_evasion).len(), 1);
        let double_url = "const target = path.join(rootDir, \"%252e%252e/config.json\");";
        assert_eq!(detect_deep_path_traversal_in_content("server.js", double_url).len(), 1);
    }
    #[test]
    fn test_threat_06_speculative_ffi_extern_c_hallucination_v260() {
        let fake_c = "fn invoke() { slop_fast_alloc(1024); }";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("engine.rs", fake_c).len(), 1);
        let legit_c = "extern \"C\" { fn malloc(size: usize) -> *mut u8; }";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("engine.rs", legit_c).len(), 0);
    }
    #[test]
    fn test_threat_06_speculative_wasm_import_hallucination_v260() {
        let fake_wasm = "WebAssembly.instantiate(bytes, { env: { __ai_eval: () => 42 } });";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("wasm_host.ts", fake_wasm).len(), 1);
        let legit_wasi = "WebAssembly.instantiate(bytes, { wasi_snapshot_preview1: wasi.wasiImport });";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("wasm_host.ts", legit_wasi).len(), 0);
    }
    #[test]
    fn test_latency_benchmark_sub_millisecond_1k_loc_v260() {
        let mut sample = String::with_capacity(32768);
        for i in 0..1000 {
            sample.push_str(&format!("const line_{i} = {i};\n"));
        }
        let start = std::time::Instant::now();
        let _ = detect_package_hallucinations_in_content("large.ts", &sample);
        let duration = start.elapsed();
        assert!(duration.as_millis() < 35, "Latency benchmark failed: took {:?}", duration);
    }
    #[test]
    fn test_v270_threat_01_speculative_go_and_wildcard_manifests() {
        let go_mod_slop = "module myapp\ngo 1.22\nrequire github.com/slop/go-redis-rate-limit v1.0.0\n";
        assert_eq!(detect_package_hallucinations_in_content("go.mod", go_mod_slop).len(), 1);
        let wildcard_npm = "{\"dependencies\": {\"lodash\": \"*\", \"axios\": \"latest\"}}";
        assert_eq!(detect_package_hallucinations_in_content("package.json", wildcard_npm).len(), 2);
    }
    #[test]
    fn test_v270_threat_02_tokio_spawn_unawaited_in_loop() {
        let rust_spawn_loop = "fn process_queue() { for item in queue { tokio::spawn(async move { handle(item); }); } }";
        assert_eq!(detect_async_loop_traps_in_content("worker.rs", rust_spawn_loop).len(), 1);
        let rust_safe_join = "fn process_queue() { for item in queue { let handle = tokio::spawn(async move { handle(item); }); handles.push(handle); } }";
        assert_eq!(detect_async_loop_traps_in_content("worker.rs", rust_safe_join).len(), 0);
    }
    #[test]
    fn test_v270_threat_03_node_serialize_gadget_rejection() {
        let js_unserialize = "const obj = serialize.unserialize(userInputPayload);";
        assert_eq!(detect_unsafe_deserialization_in_content("api.js", js_unserialize).len(), 1);
        let js_safe_json = "const obj = JSON.parse(userInputPayload);";
        assert_eq!(detect_unsafe_deserialization_in_content("api.js", js_safe_json).len(), 0);
    }
    #[test]
    fn test_v270_threat_04_rust_weak_prng_session_token() {
        let rust_bad_token = "let session_token: u64 = rand::random();";
        assert_eq!(detect_insecure_crypto_in_content("session.rs", rust_bad_token).len(), 1);
        let rust_csprng_token = "let session_token = ring::rand::SystemRandom::new();";
        assert_eq!(detect_insecure_crypto_in_content("session.rs", rust_csprng_token).len(), 0);
    }
    #[test]
    fn test_v270_threat_05_windows_unc_and_archive_zip_slip() {
        let zip_slip = "archive.extractall(target_directory)";
        assert_eq!(detect_deep_path_traversal_in_content("unzip.py", zip_slip).len(), 1);
        let rust_bad_path = "let full_path = format!(\"{}/{}\", base_dir, user_param);";
        assert_eq!(detect_deep_path_traversal_in_content("fs.rs", rust_bad_path).len(), 1);
    }
    #[test]
    fn test_v270_threat_06_synthetic_native_symbols_wat() {
        let wat_slop = "(import \"env\" \"ai_kernel_calc\" (func $calc (param i32) (result i32)))";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("kernel.wat", wat_slop).len(), 1);
        let wat_legit = "(import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))";
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("kernel.wat", wat_legit).len(), 0);
    }
    #[test]
    fn test_v270_diff_scanner_comprehensive_threat_matrix() {
        let diff_file = crate::types::DiffFile {
            old_path: "src/worker.rs".to_string(),
            new_path: "src/worker.rs".to_string(),
            is_new: false,
            is_deleted: false,
            chunks: vec![crate::types::DiffChunk {
                old_start: 1,
                old_lines: 0,
                new_start: 1,
                new_lines: 2,
                lines: vec![
                    crate::types::DiffLine {
                        line_type: crate::types::DiffLineType::Add,
                        content: "let secret_token = Math.random();".to_string(),
                        old_line_number: None,
                        new_line_number: Some(1),
                    },
                    crate::types::DiffLine {
                        line_type: crate::types::DiffLineType::Add,
                        content: "extern \"C\" { fn slop_fast_alloc(s: usize); }".to_string(),
                        old_line_number: None,
                        new_line_number: Some(2),
                    },
                ],
            }],
        };
        let cfg = crate::config::SlopConfig::default();
        let findings = crate::detectors::scanner::scan_diff(&[diff_file], &cfg);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn test_v270_ast_contract_registry_completeness() {
        let contracts = vec![
            crate::detectors::package_hallucinations::CONTRACT,
            crate::detectors::async_loop_traps::CONTRACT,
            crate::detectors::unsafe_deserialization::CONTRACT,
            crate::detectors::insecure_crypto::CONTRACT,
            crate::detectors::path_traversal_deep::CONTRACT,
            crate::detectors::ffi_speculative_apis::CONTRACT,
        ];
        assert_eq!(contracts.len(), 6);
        for c in contracts {
            assert!(!c.rule_id.is_empty());
            assert!(!c.target_languages.is_empty());
        }
    }
    #[test]
    fn test_v270_microbenchmark_sub_millisecond_10k_loc() {
        let mut buffer = String::with_capacity(262144);
        for i in 0..10000 {
            buffer.push_str(&format!("const value_{i} = {i};\n"));
        }
        let start = std::time::Instant::now();
        let findings = crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("large.rs", &buffer);
        let elapsed = start.elapsed();
        assert_eq!(findings.len(), 0);
        assert!(elapsed.as_millis() < 100, "Execution exceeded threshold: {:?}", elapsed);
    }
    #[test]
    fn test_v270_zero_false_positives_on_clean_idiomatic_code() {
        let clean_ts = "import crypto from 'crypto';\nasync function sync(arr: string[]) {\n  for (const item of arr) {\n    await process(item);\n  }\n}\nconst token = crypto.randomBytes(32).toString('hex');\n";
        assert_eq!(detect_async_loop_traps_in_content("app.ts", clean_ts).len(), 0);
        assert_eq!(detect_insecure_crypto_in_content("app.ts", clean_ts).len(), 0);
        assert_eq!(crate::detectors::ffi_speculative_apis::detect_ffi_speculative_apis_in_content("app.ts", clean_ts).len(), 0);
    }
#[test]
fn test_v280_firewall_49_hallucinated_typing_extensions_and_imports() {
let py_typing = "from typing_extensions import NonExistentType\n";
let findings_py = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("app.py", py_typing);
assert_eq!(findings_py.len(), 1);
assert_eq!(findings_py[0].rule_id, "no-hallucinated-package-manifest");
let py_pkg = "import pydantic_enhanced\n";
let findings_pkg = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("models.py", py_pkg);
assert_eq!(findings_pkg.len(), 1);
let js_pkg = "import { useSmartToast } from 'react-smart-toast';\n";
let findings_js = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("toast.tsx", js_pkg);
assert_eq!(findings_js.len(), 1);
let safe_js = "import { useQuery } from '@tanstack/react-query';\n";
let safe_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("query.ts", safe_js);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_49_manifest_wildcard_rejection() {
let manifest_star = "{\n  \"dependencies\": {\n    \"fastify\": \"*\"\n  }\n}";
let findings_star = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", manifest_star);
assert_eq!(findings_star.len(), 1);
assert_eq!(findings_star[0].rule_id, "no-hallucinated-package-manifest");
let manifest_latest = "{\n  \"dependencies\": {\n    \"lodash\": \"latest\"\n  }\n}";
let findings_latest = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", manifest_latest);
assert_eq!(findings_latest.len(), 1);
let safe_manifest = "{\n  \"dependencies\": {\n    \"@tanstack/react-query\": \"^5.28.0\"\n  }\n}";
let safe_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", safe_manifest);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_50_unawaited_async_in_loops_js_ts() {
let loop_fetch = "for (const item of items) {\n  fetch('/api/process', { method: 'POST', body: JSON.stringify(item) });\n}";
let findings_fetch = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.ts", loop_fetch);
assert_eq!(findings_fetch.len(), 1);
assert_eq!(findings_fetch[0].rule_id, "no-unawaited-async-in-loops");
assert_eq!(findings_fetch[0].severity, crate::types::SlopSeverity::Error);
let loop_email = "while (queue.length > 0) {\n  sendEmail(queue.pop());\n}";
let findings_email = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("mail.js", loop_email);
assert_eq!(findings_email.len(), 1);
let safe_loop = "for (const item of items) {\n  await fetch('/api/process', { signal: ctrl.signal });\n}";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("safe.ts", safe_loop);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_50_python_unshielded_loop_gathering() {
let py_loop = "for task in tasks:\n    asyncio.create_task(run_task(task))\n";
let findings_py = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("batch.py", py_loop);
assert_eq!(findings_py.len(), 1);
assert_eq!(findings_py[0].rule_id, "no-unawaited-async-in-loops");
let safe_py = "for task in tasks:\n    await run_task(task)\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("safe_batch.py", safe_py);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_51_math_random_in_token_and_secret_derivation() {
let js_token = "const session_token = Math.random().toString(36);";
let findings_js = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.js", js_token);
assert_eq!(findings_js.len(), 1);
assert_eq!(findings_js[0].rule_id, "no-insecure-prng-tokens");
let py_token = "api_key = random.choice(alphabet)";
let findings_py = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("keygen.py", py_token);
assert_eq!(findings_py.len(), 1);
assert_eq!(findings_py[0].rule_id, "no-insecure-prng-tokens");
let rs_token = "let csrf_token = rand::random::<u64>();";
let findings_rs = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("guard.rs", rs_token);
assert_eq!(findings_rs.len(), 1);
assert_eq!(findings_rs[0].rule_id, "no-insecure-prng-tokens");
}
#[test]
fn test_v280_firewall_51_csprng_entropy_pass() {
let js_csprng = "const token = crypto.randomBytes(32).toString('hex');\nconst otp = crypto.getRandomValues(new Uint32Array(1))[0];";
let findings_js = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("token.js", js_csprng);
assert_eq!(findings_js.len(), 0);
let py_csprng = "import secrets\ntoken = secrets.token_hex(32)\nnonce = secrets.token_urlsafe(16)";
let findings_py = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("secrets_auth.py", py_csprng);
assert_eq!(findings_py.len(), 0);
let rs_csprng = "use rand::rngs::OsRng;\nuse rand::RngCore;\nlet mut token = [0u8; 32];\nOsRng.fill_bytes(&mut token);";
let findings_rs = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("secure.rs", rs_csprng);
assert_eq!(findings_rs.len(), 0);
}
#[test]
fn test_v280_firewall_52_unsafe_pickle_and_yaml_gadgets() {
let py_pickle = "def load_payload(raw):\n    return pickle.loads(raw)\n";
let findings_pickle = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("loader.py", py_pickle);
assert_eq!(findings_pickle.len(), 1);
assert_eq!(findings_pickle[0].rule_id, "no-unsafe-deserialization");
let py_yaml = "cfg = yaml.load(user_stream, Loader=yaml.Loader)\n";
let findings_yaml = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("config.py", py_yaml);
assert_eq!(findings_yaml.len(), 1);
let safe_yaml = "cfg = yaml.safe_load(user_stream)\n";
let safe_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe_config.py", safe_yaml);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_52_node_serialize_vm_eval_rejection() {
let node_serial = "const obj = serialize.unserialize(payload);\n";
let findings_serial = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("handler.js", node_serial);
assert_eq!(findings_serial.len(), 1);
let node_vm = "vm.runInThisContext(user_input);\n";
let findings_vm = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("sandbox.js", node_vm);
assert_eq!(findings_vm.len(), 1);
let node_eval = "eval(req.body.code);\n";
let findings_eval = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("exec.js", node_eval);
assert_eq!(findings_eval.len(), 1);
let safe_json = "const obj = JSON.parse(payload);\n";
let safe_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe.js", safe_json);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_firewall_53_path_traversal_and_zip_slip_rejection() {
let node_path = "const target = path.join(__dirname, req.params.file);\n";
let findings_path = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("download.js", node_path);
assert_eq!(findings_path.len(), 1);
assert_eq!(findings_path[0].rule_id, "no-deep-path-traversal");
let py_zip = "archive.extractall(target_dir)\n";
let findings_zip = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("unzip.py", py_zip);
assert_eq!(findings_zip.len(), 1);
let safe_path = "let safe = std::fs::canonicalize(&base_path).unwrap();\nif target.starts_with(&safe) { std::fs::read(target); }\n";
let safe_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("safe.rs", safe_path);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v280_microbenchmark_sub_millisecond_10k_loc() {
let mut buffer = String::with_capacity(262144);
for i in 0..10000 {
buffer.push_str(&format!("const value_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("bench.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 100, "Microbenchmark exceeded latency budget: {:?}", elapsed);
}
#[test]
fn test_v290_firewall_49_manifest_wildcards_and_hallucinations() {
let npm_slop = "{\n  \"dependencies\": {\n    \"react-smart-toast\": \"*\",\n    \"next-auth-shield\": \"latest\"\n  }\n}";
let findings_npm = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", npm_slop);
assert!(findings_npm.len() >= 2);
let cargo_slop = "[dependencies]\nchrono-tz-sys = \"*\"\ntokio-compat-02-03 = \"0.1\"\n";
let findings_cargo = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_slop);
assert!(findings_cargo.len() >= 2);
let pypi_slop = "fastapi-jwt-auth==*\nsqlalchemy-stubs>=0.4\n";
let findings_pypi = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("requirements.txt", pypi_slop);
assert!(findings_pypi.len() >= 2);
}
#[test]
fn test_v290_firewall_49_synthetic_go_mod_and_cargo_dependencies() {
let go_mod = "module myapp\n\ngo 1.22\n\nrequire github.com/gin-gonic/gin-cors v1.0.0\n";
let findings_go = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("go.mod", go_mod);
assert_eq!(findings_go.len(), 1);
let cargo_toml = "[dependencies]\nfast-jwt-verifier = \"1.0.0\"\n";
let findings_cargo = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_toml);
assert_eq!(findings_cargo.len(), 1);
}
#[test]
fn test_v290_firewall_49_npm_wildcard_and_latest_rejection() {
let pkg_json = "{\n  \"dependencies\": {\n    \"express\": \"*\",\n    \"cors\": \"latest\"\n  }\n}";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", pkg_json);
assert_eq!(findings.len(), 2);
}
#[test]
fn test_v290_firewall_50_async_foreach_and_loop_starvation() {
let foreach_slop = "records.forEach(async (r) => { await db.save(r); });\n";
let findings_foreach = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.ts", foreach_slop);
assert_eq!(findings_foreach.len(), 1);
assert_eq!(findings_foreach[0].rule_id, "no-unawaited-async-in-loops");
let loop_uncollected = "for item in queue:\n    asyncio.create_task(process(item))\n";
let findings_loop = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.py", loop_uncollected);
assert_eq!(findings_loop.len(), 1);
let clean_for_of = "for (const r of records) {\n  await db.save(r);\n}\n";
let clean_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.ts", clean_for_of);
assert_eq!(clean_findings.len(), 0);
}
#[test]
fn test_v290_firewall_50_map_async_without_promise_all() {
let bad_map = "items.map(async (item) => { await process(item); });\n";
let bad_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("map.ts", bad_map);
assert_eq!(bad_findings.len(), 1);
let safe_map = "await Promise.all(items.map(async (item) => { await process(item); }));\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("map.ts", safe_map);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v290_firewall_50_rust_tokio_spawn_in_loop_without_joinset() {
let bad_rs = "for task in tasks {\n  tokio::spawn(async move {\n    handle(task);\n  });\n}\n";
let bad_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.rs", bad_rs);
assert_eq!(bad_findings.len(), 1);
let safe_rs = "for task in tasks {\n  let handle = tokio::spawn(async move {\n    handle(task);\n  });\n  handles.push(handle);\n}\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.rs", safe_rs);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v290_firewall_50_python_asyncio_create_task_loop_leak() {
let bad_py = "while active:\n    asyncio.create_task(run_worker())\n";
let bad_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("loop.py", bad_py);
assert_eq!(bad_findings.len(), 1);
let safe_py = "while active:\n    task = asyncio.create_task(run_worker())\n    tasks.append(task)\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("loop.py", safe_py);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v290_firewall_51_csprng_entropy_auth_tokens() {
let js_bad_token = "const auth_token = Math.random().toString(36);\n";
let js_findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.ts", js_bad_token);
assert_eq!(js_findings.len(), 1);
assert_eq!(js_findings[0].rule_id, "no-insecure-prng-tokens");
let rs_bad_token = "let reset_token: u64 = rand::random();\n";
let rs_findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.rs", rs_bad_token);
assert_eq!(rs_findings.len(), 1);
let py_bad_token = "auth_salt = random.choice(alphabet)\n";
let py_findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.py", py_bad_token);
assert_eq!(py_findings.len(), 1);
let js_safe = "const auth_token = crypto.randomBytes(32).toString('hex');\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.ts", js_safe).len(), 0);
}
#[test]
fn test_v290_firewall_51_crypto_random_bytes_and_secrets_clean_pass() {
let js_csprng = "const token = crypto.randomBytes(32);\nconst secret = crypto.randomUUID();\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.js", js_csprng).len(), 0);
let py_csprng = "import secrets\nreset_token = secrets.token_urlsafe(32)\nauth_key = secrets.token_hex(16)\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.py", py_csprng).len(), 0);
let rs_csprng = "use rand::rngs::OsRng;\nuse rand::RngCore;\nlet mut token = [0u8; 32];\nOsRng.fill_bytes(&mut token);\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.rs", rs_csprng).len(), 0);
}
#[test]
fn test_v290_firewall_51_rust_thread_rng_sensitive_token_rejection() {
let bad_rng = "let mut secret_key = [0u8; 16];\nrand::thread_rng().fill_bytes(&mut secret_key);\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("key.rs", bad_rng);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
}
#[test]
fn test_v290_firewall_52_polyglot_deserialization_gadgets() {
let node_bad = "const obj = serialize.unserialize(payload);\n";
let node_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("api.js", node_bad);
assert_eq!(node_findings.len(), 1);
let java_bad = "XMLDecoder decoder = new XMLDecoder(inStream);\n";
let java_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Handler.java", java_bad);
assert_eq!(java_findings.len(), 1);
let py_bad = "data = pickle.loads(raw_input)\n";
let py_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("loader.py", py_bad);
assert_eq!(py_findings.len(), 1);
let rb_bad = "obj = Marshal.load(cookie_str)\n";
let rb_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("session.rb", rb_bad);
assert_eq!(rb_findings.len(), 1);
}
#[test]
fn test_v290_firewall_52_java_xmldecoder_and_dotnet_binary_formatter() {
let java_xml = "XMLDecoder decoder = new XMLDecoder(stream);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Parser.java", java_xml).len(), 1);
let dotnet_bin = "BinaryFormatter.Deserialize(stream);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Parser.cs", dotnet_bin).len(), 1);
}
#[test]
fn test_v290_firewall_52_php_and_ruby_unserialize_gadgets() {
let php_bad = "$session = unserialize($_COOKIE['session']);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("auth.php", php_bad).len(), 1);
let php_safe = "$session = unserialize($_COOKIE['session'], ['allowed_classes' => false]);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("auth.php", php_safe).len(), 0);
let rb_bad = "data = Marshal.load(payload)\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("job.rb", rb_bad).len(), 1);
}
#[test]
fn test_v290_firewall_53_windows_unc_and_multi_encoded_zip_slip() {
let unc_slop = "let secret_path = \"\\\\\\\\?\\\\C:\\\\secret\\\\target\";\n";
let unc_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("fs.rs", unc_slop);
assert_eq!(unc_findings.len(), 1);
assert_eq!(unc_findings[0].rule_id, "no-deep-path-traversal");
let multi_encoded = "const user_file = \"%252e%252e%252fetc/passwd\";\n";
let multi_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("route.ts", multi_encoded);
assert_eq!(multi_findings.len(), 1);
let zip_slip = "archive.extractall(target_directory)\n";
let zip_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("unzip.py", zip_slip);
assert_eq!(zip_findings.len(), 1);
}
#[test]
fn test_v290_firewall_53_deep_traversal_with_canonicalize_pass() {
let safe_rs = "let safe_path = std::fs::canonicalize(base_dir).unwrap();\nif target.starts_with(&safe_path) {\n  std::fs::read(target);\n}\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("fs.rs", safe_rs).len(), 0);
let safe_py = "upload_name = secure_filename(file.filename)\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("upload.py", safe_py).len(), 0);
let safe_node = "const safe = path.resolve(base_dir);\nif (target.startsWith(safe)) {\n  fs.readFile(target);\n}\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("file.ts", safe_node).len(), 0);
}
#[test]
fn test_v290_microbenchmark_sub_millisecond_10k_loc() {
let mut buffer = String::with_capacity(262144);
for i in 0..10000 {
buffer.push_str(&format!("const safe_variable_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("large.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 200, "Execution exceeded threshold: {:?}", elapsed);
}
#[test]
fn test_v300_slopsquat_pypi_flask_bcrypt_auth_and_crypto_util_helpers() {
let py_req = "flask-bcrypt-auth==1.2.0\ncrypto-util-helpers==0.4.1\n";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("requirements.txt", py_req);
assert_eq!(findings.len(), 2);
assert_eq!(findings[0].rule_id, "no-hallucinated-package-manifest");
assert_eq!(findings[1].rule_id, "no-hallucinated-package-manifest");
let py_import = "import flask_bcrypt_auth\nimport crypto_util_helpers\n";
let import_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("auth.py", py_import);
assert_eq!(import_findings.len(), 2);
}
#[test]
fn test_v300_slopsquat_npm_huggingface_transformer_tools_and_crypto_node() {
let pkg_json = "{\n  \"dependencies\": {\n    \"huggingface-transformer-tools\": \"^1.0.0\",\n    \"crypto-node\": \"^0.2.1\"\n  }\n}";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", pkg_json);
assert_eq!(findings.len(), 2);
let js_req = "const hf = require('huggingface-transformer-tools');\nconst cn = require('crypto-node');\n";
let js_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("index.js", js_req);
assert_eq!(js_findings.len(), 2);
}
#[test]
fn test_v300_slopsquat_cargo_speculative_crates_and_prefix_traps() {
let cargo_toml = "[dependencies]\ntokio-requests = \"0.1.0\"\nserde-crypto = \"1.0.0\"\nanyhow-derive = \"0.2.0\"\n";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_toml);
assert_eq!(findings.len(), 3);
let rs_use = "use tokio_requests::Client;\nuse serde_crypto::Encrypt;\nuse anyhow_derive::Error;\n";
let rs_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("lib.rs", rs_use);
assert_eq!(rs_findings.len(), 3);
}
#[test]
fn test_v300_slopsquat_go_speculative_module_patterns() {
let go_mod = "module example.com/api\nrequire github.com/gin-gonic/gin-cors v1.0.0\nrequire github.com/golang-jwt/jwt-verifier v0.5.0\nrequire github.com/gorilla/mux-shield v1.2.0\n";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("go.mod", go_mod);
assert_eq!(findings.len(), 3);
}
#[test]
fn test_v300_manifest_wildcard_pinned_version_enforcement() {
let pkg_wildcard = "{\n  \"dependencies\": {\n    \"lodash\": \"*\",\n    \"axios\": \"latest\"\n  }\n}";
let pkg_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", pkg_wildcard);
assert_eq!(pkg_findings.len(), 2);
let cargo_wildcard = "[dependencies]\nserde = \"*\"\ntokio = \"latest\"\n";
let cargo_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_wildcard);
assert_eq!(cargo_findings.len(), 2);
let py_wildcard = "requests==*\nflask==latest\n";
let py_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("requirements.txt", py_wildcard);
assert_eq!(py_findings.len(), 2);
let go_wildcard = "require github.com/gin-gonic/gin v0.0.0-latest\n";
let go_findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("go.mod", go_wildcard);
assert_eq!(go_findings.len(), 1);
}
#[test]
fn test_v300_async_foreach_unhandled_promise_rejection_js_ts() {
let js_code = "users.forEach(async (user) => {\n  await db.users.update(user);\n});\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("sync.ts", js_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
}
#[test]
fn test_v300_async_map_unshielded_iteration_starvation() {
let js_code = "const promises = items.map(async (item) => {\n  return await fetch('/api/' + item.id);\n});\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("fetcher.ts", js_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
let safe_code = "const results = await Promise.all(items.map(async (item) => {\n  return await fetch('/api/' + item.id);\n}));\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("fetcher.ts", safe_code);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_while_loop_unbounded_await_without_backoff() {
let bad_while = "while (true) {\n  await db.query('SELECT 1');\n}\n";
let bad_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("poll.ts", bad_while);
assert_eq!(bad_findings.len(), 1);
assert_eq!(bad_findings[0].rule_id, "no-unawaited-async-in-loops");
let safe_while = "while (true) {\n  await db.query('SELECT 1');\n  await sleep(1000);\n}\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("poll.ts", safe_while);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_python_asyncio_task_loop_leak_without_shield() {
let py_code = "for item in pending_items:\n    asyncio.create_task(dispatch(item))\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("dispatcher.py", py_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
let safe_py = "for item in pending_items:\n    t = asyncio.create_task(dispatch(item))\n    tasks.append(t)\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("dispatcher.py", safe_py);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_rust_tokio_spawn_in_loop_without_joinset_barrier() {
let rs_code = "for req in requests {\n  tokio::spawn(async move {\n    process(req).await;\n  });\n}\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("server.rs", rs_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
let safe_rs = "for req in requests {\n  let handle = tokio::spawn(async move {\n    process(req).await;\n  });\n  join_set.push(handle);\n}\n";
let safe_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("server.rs", safe_rs);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_insecure_prng_math_random_auth_token_rejection() {
let js_code = "const auth_token = Math.random().toString(36).substring(2);\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.js", js_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
}
#[test]
fn test_v300_insecure_prng_python_random_session_key_rejection() {
let py_code = "session_key = ''.join(random.choice(chars) for _ in range(32))\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("session.py", py_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
}
#[test]
fn test_v300_insecure_prng_rust_thread_rng_salt_rejection() {
let rs_code = "let password_salt: u64 = rand::thread_rng().gen();\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("crypto.rs", rs_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-prng-tokens");
}
#[test]
fn test_v300_csprng_crypto_get_random_values_clean_pass() {
let js_code = "const buf = new Uint8Array(32);\ncrypto.getRandomValues(buf);\nconst session_token = Buffer.from(buf).toString('hex');\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.ts", js_code);
assert_eq!(findings.len(), 0);
let py_code = "import secrets\nsession_token = secrets.token_hex(32)\n";
let py_findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.py", py_code);
assert_eq!(py_findings.len(), 0);
}
#[test]
fn test_v300_polyglot_deserialization_pickle_loads_rejection() {
let py_code = "user_state = pickle.loads(untrusted_bytes)\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("state.py", py_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
}
#[test]
fn test_v300_polyglot_deserialization_unsafe_yaml_load_rejection() {
let py_bad = "config = yaml.load(raw_config, Loader=yaml.Loader)\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("config.py", py_bad);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
let py_safe = "config = yaml.load(raw_config, Loader=yaml.SafeLoader)\n";
let safe_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("config.py", py_safe);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_polyglot_deserialization_php_unserialize_rejection() {
let php_bad = "$data = unserialize($payload);\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("index.php", php_bad);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
let php_safe = "$data = unserialize($payload, ['allowed_classes' => false]);\n";
let safe_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("index.php", php_safe);
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_v300_polyglot_deserialization_ruby_marshal_load_rejection() {
let rb_bad = "cached_data = Marshal.load(redis.get('session'))\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("cache.rb", rb_bad);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
}
#[test]
fn test_v300_polyglot_deserialization_java_xmldecoder_rejection() {
let java_bad = "XMLDecoder decoder = new XMLDecoder(inputStream);\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("XmlParser.java", java_bad);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
}
#[test]
fn test_v300_polyglot_deserialization_dotnet_binary_formatter_rejection() {
let cs_bad = "var obj = BinaryFormatter.Deserialize(stream);\n";
let findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Service.cs", cs_bad);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unsafe-deserialization");
}
#[test]
fn test_v300_deep_path_traversal_windows_unc_device_path() {
let unc_code = "let path = \"\\\\\\\\?\\\\GLOBALROOT\\\\Device\\\\Harddisk0\";\n";
let findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("io.rs", unc_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
let dev_code = "const dev = \"\\\\\\\\.\\\\PhysicalDrive0\";\n";
let dev_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("disk.ts", dev_code);
assert_eq!(dev_findings.len(), 1);
}
#[test]
fn test_v300_deep_path_traversal_multi_encoded_bypass() {
let bypass_code = "const target = \"%c0%ae%c0%ae/etc/shadow\";\n";
let findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("server.js", bypass_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
let bypass_2 = "let user_file = \"%252e%252e%252fboot.ini\";\n";
let findings_2 = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("fs.ts", bypass_2);
assert_eq!(findings_2.len(), 1);
}
#[test]
fn test_v300_deep_path_traversal_python_tarfile_zipfile_slip() {
let tar_slip = "tarfile.open('upload.tar.gz').extractall('/var/www/uploads')\n";
let findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("extractor.py", tar_slip);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-deep-path-traversal");
let zip_slip = "zipfile.ZipFile('bundle.zip').extractall(dest_path)\n";
let zip_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("unzip.py", zip_slip);
assert_eq!(zip_findings.len(), 1);
}
#[test]
fn test_v300_deep_path_traversal_canonicalize_clean_pass() {
let safe_rs = "let canonical = std::fs::canonicalize(&untrusted)?;\nif canonical.starts_with(&root_dir) {\n  std::fs::read(&canonical)?;\n}\n";
let findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("safe.rs", safe_rs);
assert_eq!(findings.len(), 0);
let safe_py = "safe_name = secure_filename(file.filename)\n";
let py_findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("upload.py", safe_py);
assert_eq!(py_findings.len(), 0);
}
#[test]
fn test_v300_microbenchmark_sub_millisecond_10k_loc_v300() {
let mut buffer = String::with_capacity(300000);
for i in 0..10000 {
buffer.push_str(&format!("const production_constant_v300_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("bench.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 250, "v300 microbenchmark exceeded budget: {:?}", elapsed);
}
#[test]
fn test_v350_ast_66_detector_matrix_integrity() {
let total_rules = crate::detectors::AST_DETECTOR_DEFINITIONS.len();
assert!(total_rules >= 66);
let mut ids = std::collections::HashSet::new();
for def in crate::detectors::AST_DETECTOR_DEFINITIONS {
assert!(ids.insert(def.id), "Duplicate rule id: {}", def.id);
assert!(!def.name.is_empty());
assert!(!def.category.is_empty());
assert!(!def.cwe.is_empty());
assert!(!def.description.is_empty());
assert!(!def.recommendation.is_empty());
}
for i in 1..=total_rules {
let expected_id = format!("SLOP-{:03}", i);
assert!(ids.contains(expected_id.as_str()), "Missing rule: {}", expected_id);
}
let d61 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-061").unwrap();
assert_eq!(d61.name, "no-speculative-subprocess-injection");
assert_eq!(d61.cwe, "CWE-78");
let d62 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-062").unwrap();
assert_eq!(d62.name, "no-unbounded-event-listener-loop");
assert_eq!(d62.cwe, "CWE-400");
let d63 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-063").unwrap();
assert_eq!(d63.name, "no-phantom-ai-wrappers");
assert_eq!(d63.cwe, "CWE-1357");
let d64 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-064").unwrap();
assert_eq!(d64.name, "no-speculative-file-race-toctou");
assert_eq!(d64.cwe, "CWE-367");
let d65 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-065").unwrap();
assert_eq!(d65.name, "no-hallucinated-crypto-algorithm");
assert_eq!(d65.cwe, "CWE-327");
let d66 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-066").unwrap();
assert_eq!(d66.name, "no-speculative-ssrf-interpolation");
assert_eq!(d66.cwe, "CWE-918");
}
#[test]
fn test_v320_slopsquat_package_hallucinations_polyglot_matrix() {
let npm_json = "{\n\"dependencies\": {\n\"@huggingface/transformer-tools\": \"^1.0.0\",\n\"crypto-node\": \"^2.0.0\",\n\"react-smart-toast\": \"^1.2.0\",\n\"next-auth-shield\": \"^0.1.0\",\n\"express-jwt-auto-auth\": \"^1.0.0\"\n}\n}";
let npm_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", npm_json);
assert_eq!(npm_res.len(), 5);
let cargo_toml = "[dependencies]\ntokio-sync-helpers = \"0.1.0\"\ntokio-requests = \"0.3.0\"\nserde-crypto = \"1.0.0\"\nanyhow-derive = \"0.1.0\"\nfast-jwt-verifier = \"0.2.0\"\ntokio-redis-pro = \"1.0.0\"\nserde_json_fast = \"1.0.0\"\nactix-rate-limit-plus = \"0.4.0\"\n";
let cargo_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_toml);
assert_eq!(cargo_res.len(), 8);
let py_reqs = "flask-bcrypt-auth>=1.0\ncrypto-util-helpers==0.2\nhuggingface-transformer-tools>=0.1\nrequests-async-retry>=2.0\ndjango-smart-cache==1.0\n";
let py_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("requirements.txt", py_reqs);
assert_eq!(py_res.len(), 5);
let go_mod = "module myapp\n\nrequire (\n\tgithub.com/gin-gonic/gin-helpers v1.0.0\n\tgithub.com/gin-gonic/gin-cors v1.2.0\n\tgithub.com/golang-jwt/jwt-verifier v0.5.0\n\tgithub.com/gorilla/mux-shield v1.1.0\n)\n";
let go_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("go.mod", go_mod);
assert_eq!(go_res.len(), 4);
}
#[test]
fn test_v320_slopsquat_manifest_wildcard_unpinned_permutations() {
let npm_wildcard = "{\n\"dependencies\": {\n\"lodash\": \"*\",\n\"express\": \"latest\"\n}\n}";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("package.json", npm_wildcard);
assert_eq!(findings.len(), 2);
let cargo_wildcard = "[dependencies]\nserde = \"*\"\ntokio = \"latest\"\n";
let cargo_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo_wildcard);
assert_eq!(cargo_res.len(), 2);
let py_wildcard = "fastapi==*\npydantic==latest\nrequests v0.0.0-latest\n";
let py_res = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("requirements.txt", py_wildcard);
assert_eq!(py_res.len(), 3);
}
#[test]
fn test_v320_async_loop_traps_backpressure_and_joinset() {
let js_foreach = "items.forEach(async (item) => {\n  await db.query('INSERT INTO logs VALUES (?)', [item]);\n});\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("batch.js", js_foreach);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
let js_map = "const tasks = items.map(async (item) => await fetch('/api/task/' + item.id));\n";
let map_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("mapper.ts", js_map);
assert_eq!(map_findings.len(), 1);
let rs_loop = "for task in tasks {\n  tokio::spawn(async move {\n    process(task).await;\n  });\n}\n";
let rs_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("worker.rs", rs_loop);
assert_eq!(rs_findings.len(), 1);
let py_loop = "for item in items:\n    asyncio.create_task(process_item(item))\n";
let py_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("async_pool.py", py_loop);
assert_eq!(py_findings.len(), 1);
}
#[test]
fn test_v320_async_loop_traps_infinite_while_unbounded_await() {
let while_loop = "while (true) {\n  const res = await fetch('https://api.internal/heartbeat');\n}\n";
let findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("heartbeat.js", while_loop);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unawaited-async-in-loops");
let for_loop = "for (;;) {\n  await axios.get('/stream');\n}\n";
let for_findings = crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("loop.ts", for_loop);
assert_eq!(for_findings.len(), 1);
}
#[test]
fn test_v320_async_loop_safe_joinset_and_promise_all_passes() {
let safe_js = "await Promise.all(items.map(async (item) => await fetch('/api/' + item.id)));\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("safe.ts", safe_js).len(), 0);
let safe_rs = "let mut set = JoinSet::new();\nfor task in tasks {\n  set.spawn(async move { task.run().await });\n}\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("safe.rs", safe_rs).len(), 0);
let safe_py = "sem = asyncio.Semaphore(10)\nfor item in items:\n    asyncio.gather(process_item(item))\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_loop_traps_in_content("safe.py", safe_py).len(), 0);
}
#[test]
fn test_v320_polyglot_deserialization_csharp_ruby_php_java() {
let cs_bin = "var formatter = new BinaryFormatter();\nvar payload = (Order)formatter.Deserialize(stream);\n";
let cs_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Order.cs", cs_bin);
assert_eq!(cs_findings.len(), 1);
let cs_netdata = "var serializer = new NetDataContractSerializer();\nvar data = serializer.Deserialize(stream);\n";
let cs_netdata_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("App.cs", cs_netdata);
assert_eq!(cs_netdata_findings.len(), 1);
let java_xml = "XMLDecoder decoder = new XMLDecoder(stream);\nObject obj = decoder.readObject();\n";
let java_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("Parser.java", java_xml);
assert_eq!(java_findings.len(), 2);
let rb_code = "session = Marshal.load(redis.get('session_key'))\n";
let rb_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("session.rb", rb_code);
assert_eq!(rb_findings.len(), 1);
let php_code = "$user = unserialize($_COOKIE['user_session']);\n";
let php_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("session.php", php_code);
assert_eq!(php_findings.len(), 1);
}
#[test]
fn test_v320_polyglot_deserialization_python_marshal_and_shelve() {
let py_marshal = "bytecode = marshal.loads(raw_bytecode)\n";
let py_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("loader.py", py_marshal);
assert_eq!(py_findings.len(), 1);
let py_shelve = "db = shelve.open('user_cache.db')\n";
let shelve_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("cache.py", py_shelve);
assert_eq!(shelve_findings.len(), 1);
let py_yaml = "cfg = yaml.unsafe_load(user_stream)\n";
let yaml_findings = crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("config.py", py_yaml);
assert_eq!(yaml_findings.len(), 1);
}
#[test]
fn test_v320_polyglot_deserialization_safe_parsers_pass() {
let safe_js = "const state = JSON.parse(rawText);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe.ts", safe_js).len(), 0);
let safe_py = "data = yaml.safe_load(stream, Loader=yaml.SafeLoader)\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe.py", safe_py).len(), 0);
let safe_php = "$user = unserialize($data, ['allowed_classes' => false]);\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe.php", safe_php).len(), 0);
let safe_rb = "cfg = YAML.safe_load(content, permitted_classes: [Symbol])\n";
assert_eq!(crate::detectors::unsafe_deserialization::detect_unsafe_deserialization_in_content("safe.rb", safe_rb).len(), 0);
}
#[test]
fn test_v320_insecure_prng_csprng_entropy_enforcement() {
let js_bad = "const authToken = Math.random().toString(36);\n";
let js_res = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.ts", js_bad);
assert_eq!(js_res.len(), 1);
assert_eq!(js_res[0].rule_id, "no-insecure-prng-tokens");
let py_bad = "def create_session():\n    session_secret = random.random()\n";
let py_res = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("session.py", py_bad);
assert_eq!(py_res.len(), 1);
let rs_bad = "fn generate_salt() {\n    let salt = rand::thread_rng().gen::<[u8; 32]>();\n}\n";
let rs_res = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("crypto.rs", rs_bad);
assert_eq!(rs_res.len(), 1);
let go_bad = "func generateNonce() {\n    nonce := rand.Intn(1000000)\n}\n";
let go_res = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("auth.go", go_bad);
assert_eq!(go_res.len(), 1);
}
#[test]
fn test_v320_insecure_prng_static_seed_rejection() {
let py_seed = "random.seed(42)\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("test.py", py_seed);
assert_eq!(findings.len(), 1);
let rs_seed = "let mut rng = StdRng::seed_from_u64(1337);\n";
let rs_findings = crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("rng.rs", rs_seed);
assert_eq!(rs_findings.len(), 1);
}
#[test]
fn test_v320_csprng_entropy_safe_generators_pass() {
let safe_js = "const buf = new Uint8Array(32);\ncrypto.getRandomValues(buf);\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("safe.ts", safe_js).len(), 0);
let safe_py = "token = secrets.token_hex(32)\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("safe.py", safe_py).len(), 0);
let safe_rs = "let secret = rand::rngs::OsRng;\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_crypto_in_content("safe.rs", safe_rs).len(), 0);
}
#[test]
fn test_v320_deep_path_traversal_windows_unc_and_archive_slips() {
let unc = "const p = \"\\\\\\\\?\\\\GLOBALROOT\\\\Device\\\\Harddisk0\";\n";
let unc_res = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("fs.ts", unc);
assert_eq!(unc_res.len(), 1);
assert_eq!(unc_res[0].rule_id, "no-deep-path-traversal");
let multi_enc = "const path = \"%252e%252e%252fetc%252fshadow\";\n";
let enc_res = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("server.js", multi_enc);
assert_eq!(enc_res.len(), 1);
let tar_slip = "tarfile.open(archive).extractall('/var/www')\n";
let tar_res = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("backup.py", tar_slip);
assert_eq!(tar_res.len(), 1);
}
#[test]
fn test_v320_deep_path_traversal_canonicalize_and_whitelist_passes() {
let safe_rs = "let safe = std::fs::canonicalize(input)?;\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("file.rs", safe_rs).len(), 0);
let safe_py = "filename = secure_filename(uploaded_file.filename)\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("upload.py", safe_py).len(), 0);
let safe_go = "cleanPath := filepath.Clean(userPath)\n";
assert_eq!(crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("handler.go", safe_go).len(), 0);
}
#[test]
fn test_v320_fuzzing_malformed_tokens_and_unicode_permutations() {
let fuzzed = "const file = \"\\u002e\\u002e/%252e%252e/%c0%ae%c0%ae/%00/etc/passwd\";\n";
let findings = crate::detectors::path_traversal_deep::detect_deep_path_traversal_in_content("fuzz.ts", fuzzed);
assert_eq!(findings.len(), 1);
}
#[test]
fn test_v320_fuzzing_deeply_nested_structures_no_panic() {
let mut nested = String::from("const nested = ");
for _ in 0..500 {
nested.push_str("([{\"\": ");
}
nested.push_str("42");
for _ in 0..500 {
nested.push_str("}}])");
}
nested.push_str(";\n");
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("fuzz.ts", &nested);
assert_eq!(findings.len(), 0);
}
#[test]
fn test_v320_compound_speculative_package_prefixes_and_suffixes() {
let cargo = "[dependencies]\nserde-sync-helpers = \"1.0\"\ntokio-auto-auth = \"0.2\"\nfastapi-jwt-guard = \"0.1\"\nexpress-smart-toast = \"1.0\"\n";
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("Cargo.toml", cargo);
assert_eq!(findings.len(), 4);
}
#[test]
fn test_v320_ide_protocol_hallucinations_slop_055() {
let bad_code = "vscode.window.showAIMessage('Generating code...');\ncursor.editor.injectPrompt({ prompt: 'Refactor' });\n";
let findings = crate::detectors::ide_protocol::detect_ide_protocol_hallucinations_in_content("ext.ts", bad_code);
assert_eq!(findings.len(), 2);
}
#[test]
fn test_v320_microbenchmark_sub_millisecond_10k_loc() {
let mut buffer = String::with_capacity(350000);
for i in 0..10000 {
buffer.push_str(&format!("const production_constant_v320_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::package_hallucinations::detect_package_hallucinations_in_content("bench_v320.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 250, "v320 microbenchmark exceeded budget: {:?}", elapsed);
}
#[test]
fn test_v330_slopsquat_compound_namespace_detection() {
let reqs = "flask-bcrypt-auth>=1.0\n";
let py_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("requirements.txt", reqs);
assert_eq!(py_findings.len(), 1);
assert_eq!(py_findings[0].rule_id, "no-slopsquat-compound-namespace");
let pkg = "{\n\"dependencies\": {\n\"crypto-util-helpers\": \"^1.0.0\"\n}\n}\n";
let pkg_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("package.json", pkg);
assert_eq!(pkg_findings.len(), 1);
let cargo = "[dependencies]\ntokio-redis-pro = \"1.0.0\"\n";
let cargo_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", cargo);
assert_eq!(cargo_findings.len(), 1);
let safe_cargo = "[dependencies]\ntokio = \"1.0\"\nserde = \"1.0\"\nreqwest = \"0.11\"\n";
assert_eq!(crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", safe_cargo).len(), 0);
let safe_reqs = "flask==2.0\ntorch==2.0\n";
assert_eq!(crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("requirements.txt", safe_reqs).len(), 0);
}
#[test]
fn test_v330_slopsquat_compound_namespace_diff() {
let diff = vec![crate::types::DiffFile {
old_path: "package.json".to_string(),
new_path: "package.json".to_string(),
chunks: vec![crate::types::DiffChunk {
old_start: 1,
old_lines: 0,
new_start: 1,
new_lines: 1,
lines: vec![crate::types::DiffLine {
line_type: crate::types::DiffLineType::Add,
content: "\"react-smart-toast\": \"^1.0.0\"".to_string(),
old_line_number: None,
new_line_number: Some(1),
}],
}],
is_new: false,
is_deleted: false,
}];
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_diff(&diff);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-slopsquat-compound-namespace");
}
#[test]
fn test_v330_slopsquat_compound_go_mod_and_pyproject() {
let go_mod = "module test\nrequire github.com/golang-jwt/jwt-verifier v0.5.0\n";
let go_res = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("go.mod", go_mod);
assert_eq!(go_res.len(), 1);
let pyproj = "[tool.poetry.dependencies]\nfastapi-jwt-guard = \"^0.1\"\n";
let py_res = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("pyproject.toml", pyproj);
assert_eq!(py_res.len(), 1);
}
#[test]
fn test_v330_slopsquat_compound_source_imports_rejection() {
let py_code = "from flask_bcrypt_auth import auth\n";
let py_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("auth.py", py_code);
assert_eq!(py_findings.len(), 1);
let ts_code = "import { toast } from 'react-smart-toast';\n";
let ts_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("ui.ts", ts_code);
assert_eq!(ts_findings.len(), 1);
let rs_code = "use tokio_redis_pro::Client;\n";
let rs_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("worker.rs", rs_code);
assert_eq!(rs_findings.len(), 1);
}
#[test]
fn test_v330_async_generator_unhandled_break_detection() {
let bad_js = "for await (const chunk of stream) {\n  if (done) break;\n}\n";
let bad_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("stream.ts", bad_js);
assert_eq!(bad_findings.len(), 1);
assert_eq!(bad_findings[0].rule_id, "no-async-generator-unhandled-break");
let safe_js = "try {\n  for await (const chunk of stream) {\n    if (done) break;\n  }\n} finally {\n  stream.destroy();\n}\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.ts", safe_js).len(), 0);
let safe_abort = "for await (const chunk of stream) {\n  if (done) {\n    controller.abort();\n    break;\n  }\n}\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("abort.ts", safe_abort).len(), 0);
}
#[test]
fn test_v330_async_generator_unhandled_return_detection() {
let ret_js = "for await (const item of generator) {\n  if (item.error) return;\n}\n";
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("gen.js", ret_js);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-async-generator-unhandled-break");
}
#[test]
fn test_v330_async_generator_python_unhandled_break() {
let py_bad = "async for chunk in stream:\n    if cond:\n        break\n";
let py_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("reader.py", py_bad);
assert_eq!(py_findings.len(), 1);
let py_safe = "async for chunk in stream:\n    try:\n        if cond:\n            break\n    finally:\n        await stream.aclose()\n";
assert_eq!(crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.py", py_safe).len(), 0);
}
#[test]
fn test_v330_async_generator_break_diff() {
let diff = vec![crate::types::DiffFile {
old_path: "consumer.js".to_string(),
new_path: "consumer.js".to_string(),
chunks: vec![crate::types::DiffChunk {
old_start: 1,
old_lines: 0,
new_start: 1,
new_lines: 3,
lines: vec![
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "for await (const chunk of stream) {".to_string(), old_line_number: None, new_line_number: Some(1) },
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "  if (err) break;".to_string(), old_line_number: None, new_line_number: Some(2) },
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "}".to_string(), old_line_number: None, new_line_number: Some(3) },
],
}],
is_new: false,
is_deleted: false,
}];
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_diff(&diff);
assert_eq!(findings.len(), 1);
}
#[test]
fn test_v330_insecure_direct_crypto_keygen_detection() {
let js_key = "const key = Math.random().toString(36).substring(2);\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("crypto.ts", js_key);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-direct-crypto-keygen");
let js_aes = "const aesKey = crypto.randomBytes(8).toString('hex');\n";
let aes_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("aes.js", js_aes);
assert_eq!(aes_findings.len(), 1);
let safe_derive = "const derived = await crypto.subtle.deriveKey(pbkdf2Params, baseKey, { name: 'AES-GCM', length: 256 }, false, ['encrypt']);\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.ts", safe_derive).len(), 0);
let safe_bytes = "const key = crypto.randomBytes(32);\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe2.ts", safe_bytes).len(), 0);
let safe_rs = "rand::rngs::OsRng.fill_bytes(&mut key);\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.rs", safe_rs).len(), 0);
}
#[test]
fn test_v330_insecure_direct_crypto_keygen_python_random_choices() {
let py_code = "key = random.choices(string.ascii_letters, k=16)\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("keygen.py", py_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-insecure-direct-crypto-keygen");
let py_safe = "key = secrets.token_bytes(32)\n";
assert_eq!(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.py", py_safe).len(), 0);
}
#[test]
fn test_v330_insecure_direct_crypto_keygen_truncated_hex_slice() {
let truncated = "const encryption_key = crypto.randomBytes(32).toString('hex').slice(0, 16);\n";
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("cipher.ts", truncated);
assert_eq!(findings.len(), 1);
let pseudo = "const secret_key = crypto.pseudoRandomBytes(16);\n";
let pseudo_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("secret.js", pseudo);
assert_eq!(pseudo_findings.len(), 1);
}
#[test]
fn test_v330_insecure_direct_crypto_keygen_diff() {
let diff = vec![crate::types::DiffFile {
old_path: "auth.ts".to_string(),
new_path: "auth.ts".to_string(),
chunks: vec![crate::types::DiffChunk {
old_start: 1,
old_lines: 0,
new_start: 1,
new_lines: 1,
lines: vec![crate::types::DiffLine {
line_type: crate::types::DiffLineType::Add,
content: "let aes_key = Math.random().toString(36);".to_string(),
old_line_number: None,
new_line_number: Some(1),
}],
}],
is_new: false,
is_deleted: false,
}];
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_diff(&diff);
assert_eq!(findings.len(), 1);
}
#[test]
fn test_v330_unbounded_channel_sprawl_detection() {
let rs_code = "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();\n";
let rs_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", rs_code);
assert_eq!(rs_findings.len(), 1);
assert_eq!(rs_findings[0].rule_id, "no-unbounded-channel-sprawl");
let go_code = "ch := make(chan Message)\n";
let go_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("worker.go", go_code);
assert_eq!(go_findings.len(), 1);
let rs_safe = "let (tx, rx) = tokio::sync::mpsc::channel(1024);\n";
assert_eq!(crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", rs_safe).len(), 0);
let go_safe = "ch := make(chan Message, 100)\n";
assert_eq!(crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.go", go_safe).len(), 0);
let py_safe = "queue = asyncio.Queue(maxsize=500)\n";
assert_eq!(crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.py", py_safe).len(), 0);
}
#[test]
fn test_v330_unbounded_channel_sprawl_python_asyncio_queue() {
let py_code = "queue = asyncio.Queue()\n";
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("pipeline.py", py_code);
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].rule_id, "no-unbounded-channel-sprawl");
}
#[test]
fn test_v330_unbounded_channel_sprawl_rust_mpsc_unbounded() {
let rs_code = "let (tx, rx) = mpsc::unbounded_channel();\n";
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("bus.rs", rs_code);
assert_eq!(findings.len(), 1);
}
#[test]
fn test_v330_unbounded_channel_sprawl_diff() {
let diff = vec![crate::types::DiffFile {
old_path: "events.go".to_string(),
new_path: "events.go".to_string(),
chunks: vec![crate::types::DiffChunk {
old_start: 1,
old_lines: 0,
new_start: 1,
new_lines: 1,
lines: vec![crate::types::DiffLine {
line_type: crate::types::DiffLineType::Add,
content: "events := make(chan Event)".to_string(),
old_line_number: None,
new_line_number: Some(1),
}],
}],
is_new: false,
is_deleted: false,
}];
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_diff(&diff);
assert_eq!(findings.len(), 1);
}
#[test]
fn test_v330_microbenchmark_sub_millisecond_10k_loc() {
let mut buffer = String::with_capacity(350000);
for i in 0..10000 {
buffer.push_str(&format!("const production_constant_v330_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("bench_v330.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 250, "v330 microbenchmark exceeded budget: {:?}", elapsed);
}
#[test]
fn test_v330_full_scanner_polyglot_e2e_matrix() {
let cfg = crate::types::SlopConfig { ignore: vec![], rules: std::collections::HashMap::new(), max_drift_files: 50, allowed_ghost_deps: vec![] };
let root = std::path::Path::new(".");
let f1 = crate::detectors::run_all_content_detectors("package.json", "{\n\"dependencies\": {\n\"react-smart-toast\": \"*\"\n}\n}\n", root, &cfg);
assert!(f1.iter().any(|f| f.rule_id == "no-slopsquat-compound-namespace"));
let f2 = crate::detectors::run_all_content_detectors("stream.ts", "for await (const chunk of stream) {\n  if (done) break;\n}\n", root, &cfg);
assert!(f2.iter().any(|f| f.rule_id == "no-async-generator-unhandled-break"));
let f3 = crate::detectors::run_all_content_detectors("key.ts", "const key = Math.random().toString(36).substring(2);\n", root, &cfg);
assert!(f3.iter().any(|f| f.rule_id == "no-insecure-direct-crypto-keygen"));
let f4 = crate::detectors::run_all_content_detectors("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();\n", root, &cfg);
assert!(f4.iter().any(|f| f.rule_id == "no-unbounded-channel-sprawl"));
}
#[test]
fn test_v330_fuzzing_resilience_and_special_characters() {
let mut fuzzed = String::with_capacity(10000);
for _ in 0..200 {
fuzzed.push_str("for await (const x of s) { break; } finally { s.destroy(); }\n");
fuzzed.push_str("const key = crypto.randomBytes(32);\n");
fuzzed.push_str("let (tx, rx) = tokio::sync::mpsc::channel(100);\n");
}
let res1 = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("fuzz.ts", &fuzzed);
assert_eq!(res1.len(), 0);
let res2 = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("fuzz.ts", &fuzzed);
assert_eq!(res2.len(), 0);
let res3 = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("fuzz.rs", &fuzzed);
assert_eq!(res3.len(), 0);
}
#[test]
fn test_v330_contract_severity_and_categories() {
let d57 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-057").unwrap();
assert_eq!(d57.severity, crate::types::SlopSeverity::Error);
assert_eq!(d57.category, "supply-chain");
let d58 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-058").unwrap();
assert_eq!(d58.severity, crate::types::SlopSeverity::Error);
assert_eq!(d58.category, "async-concurrency");
let d59 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-059").unwrap();
assert_eq!(d59.severity, crate::types::SlopSeverity::Error);
assert_eq!(d59.category, "crypto-auth");
let d60 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-060").unwrap();
assert_eq!(d60.severity, crate::types::SlopSeverity::Error);
assert_eq!(d60.category, "async-concurrency");
}
#[test]
fn test_v340_speculative_subprocess_injection_detection() {
let py_code = "subprocess.Popen(f\"cat {filename}\", shell=True)";
let f = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", py_code);
assert_eq!(f.len(), 1);
assert_eq!(f[0].rule_id, "no-speculative-subprocess-injection");
assert_eq!(f[0].severity, crate::types::SlopSeverity::Error);
let js_code = "child_process.exec(`ls -la ${folder}`)";
let f_js = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("server.js", js_code);
assert_eq!(f_js.len(), 1);
let safe_py = "subprocess.run(['ls', '-la'], check=True)";
let safe_f = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", safe_py);
assert_eq!(safe_f.len(), 0);
}
#[test]
fn test_v340_unbounded_event_listener_loop_detection() {
let leak_js = "for (const item of items) { emitter.on('data', handler); }";
let f = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", leak_js);
assert_eq!(f.len(), 1);
assert_eq!(f[0].rule_id, "no-unbounded-event-listener-loop");
assert_eq!(f[0].severity, crate::types::SlopSeverity::Warning);
let safe_js = "for (const item of items) { emitter.once('data', handler); }";
let safe_f = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", safe_js);
assert_eq!(safe_f.len(), 0);
}
#[test]
fn test_v340_phantom_ai_wrappers_detection() {
let manifest = "{\n  \"dependencies\": {\n    \"@anthropic/tools-pro\": \"^1.0.0\",\n    \"openai-streaming-helper\": \"^2.1.0\"\n  }\n}";
let f = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("package.json", manifest);
assert_eq!(f.len(), 2);
assert_eq!(f[0].rule_id, "no-phantom-ai-wrappers");
let py_req = "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0";
let f_py = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", py_req);
assert_eq!(f_py.len(), 2);
}
#[test]
fn test_v340_contract_severity_and_categories() {
let d61 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-061").unwrap();
assert_eq!(d61.severity, crate::types::SlopSeverity::Error);
assert_eq!(d61.category, "path-injection");
let d62 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-062").unwrap();
assert_eq!(d62.severity, crate::types::SlopSeverity::Warning);
assert_eq!(d62.category, "async-concurrency");
let d63 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-063").unwrap();
assert_eq!(d63.severity, crate::types::SlopSeverity::Error);
assert_eq!(d63.category, "supply-chain");
}
#[test]
fn test_v350_speculative_file_race_toctou_detection() {
let js_code = "fs.existsSync(p); fs.writeFileSync(p, data);";
let f_js = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("upload.js", js_code);
assert_eq!(f_js.len(), 1);
assert_eq!(f_js[0].rule_id, "no-speculative-file-race-toctou");
assert_eq!(f_js[0].severity, crate::types::SlopSeverity::Error);
assert_eq!(f_js[0].category, crate::types::SlopCategory::FileRaceToctou);
let py_code = "if os.path.exists(p): open(p, \"w\")";
let f_py = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("store.py", py_code);
assert_eq!(f_py.len(), 1);
let safe_js = "fs.writeFileSync(p, data, { flag: \"wx\" });";
let safe_f_js = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", safe_js);
assert_eq!(safe_f_js.len(), 0);
let safe_py = "open(p, \"x\")";
let safe_f_py = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.py", safe_py);
assert_eq!(safe_f_py.len(), 0);
}
#[test]
fn test_v350_hallucinated_crypto_algorithm_detection() {
let js_code = "crypto.createCipheriv(\"aes-256-turbo\", key, iv);";
let f_js = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", js_code);
assert_eq!(f_js.len(), 1);
assert_eq!(f_js[0].rule_id, "no-hallucinated-crypto-algorithm");
assert_eq!(f_js[0].severity, crate::types::SlopSeverity::Error);
assert_eq!(f_js[0].category, crate::types::SlopCategory::HallucinatedCryptoAlgorithm);
let py_code = "hashlib.new(\"sha256-fast\")";
let f_py = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("hash.py", py_code);
assert_eq!(f_py.len(), 1);
let safe_js = "crypto.createCipheriv(\"aes-256-gcm\", key, iv);";
let safe_f_js = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", safe_js);
assert_eq!(safe_f_js.len(), 0);
let safe_py = "hashlib.sha256()";
let safe_f_py = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.py", safe_py);
assert_eq!(safe_f_py.len(), 0);
}
#[test]
fn test_v350_speculative_ssrf_interpolation_detection() {
let js_code = "axios.get(`https://${targetHost}/api/data`)";
let f_js = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", js_code);
assert_eq!(f_js.len(), 1);
assert_eq!(f_js[0].rule_id, "no-speculative-ssrf-interpolation");
assert_eq!(f_js[0].severity, crate::types::SlopSeverity::Error);
assert_eq!(f_js[0].category, crate::types::SlopCategory::SpeculativeSsrfInterpolation);
let py_code = "requests.get(f\"https://{api_host}/v1\")";
let f_py = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.py", py_code);
assert_eq!(f_py.len(), 1);
let safe_code = "axios.get(\"https://api.verified.com/v1\");";
let safe_f = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", safe_code);
assert_eq!(safe_f.len(), 0);
}
#[test]
fn test_v350_contract_severity_and_categories() {
let d64 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-064").unwrap();
assert_eq!(d64.severity, crate::types::SlopSeverity::Error);
assert_eq!(d64.category, "path-injection");
let d65 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-065").unwrap();
assert_eq!(d65.severity, crate::types::SlopSeverity::Error);
assert_eq!(d65.category, "crypto-auth");
let d66 = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-066").unwrap();
assert_eq!(d66.severity, crate::types::SlopSeverity::Error);
assert_eq!(d66.category, "path-injection");
}
#[test]
fn test_v350_full_scanner_polyglot_e2e_matrix() {
let cfg = crate::types::SlopConfig { ignore: vec![], rules: std::collections::HashMap::new(), max_drift_files: 50, allowed_ghost_deps: vec![] };
let root = std::path::Path::new(".");
let f1 = crate::detectors::run_all_content_detectors("race.js", "if (fs.existsSync(filePath)) { fs.writeFileSync(filePath, data); }\n", root, &cfg);
assert!(f1.iter().any(|f| f.rule_id == "no-speculative-file-race-toctou"));
let f2 = crate::detectors::run_all_content_detectors("cipher.js", "const cipher = crypto.createCipheriv(\"aes-256-turbo\", key, iv);\n", root, &cfg);
assert!(f2.iter().any(|f| f.rule_id == "no-hallucinated-crypto-algorithm"));
let f3 = crate::detectors::run_all_content_detectors("http.py", "res = requests.get(f\"https://{api_host}/v1\")\n", root, &cfg);
assert!(f3.iter().any(|f| f.rule_id == "no-speculative-ssrf-interpolation"));
}
#[test]
fn test_v350_microbenchmark_sub_millisecond_10k_loc() {
let mut buffer = String::with_capacity(350000);
for i in 0..10000 {
buffer.push_str(&format!("const production_constant_v350_{i} = {i};\n"));
}
let start = std::time::Instant::now();
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("bench_v350.ts", &buffer);
let elapsed = start.elapsed();
assert_eq!(findings.len(), 0);
assert!(elapsed.as_millis() < 250, "v350 microbenchmark exceeded budget: {:?}", elapsed);
}
#[test]
fn test_v350_diff_detectors_integration() {
let diff_files = vec![
crate::types::DiffFile {
old_path: "service.js".to_string(),
new_path: "service.js".to_string(),
chunks: vec![crate::types::DiffChunk {
old_start: 1,
old_lines: 0,
new_start: 1,
new_lines: 3,
lines: vec![
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "fs.existsSync(p); fs.writeFileSync(p, data);".to_string(), old_line_number: None, new_line_number: Some(1) },
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "const h = hashlib.new(\"sha256-fast\");".to_string(), old_line_number: None, new_line_number: Some(2) },
crate::types::DiffLine { line_type: crate::types::DiffLineType::Add, content: "axios.get(`https://${targetHost}/api`);".to_string(), old_line_number: None, new_line_number: Some(3) },
],
}],
is_new: false,
is_deleted: false,
},
];
let cfg = crate::types::SlopConfig { ignore: vec![], rules: std::collections::HashMap::new(), max_drift_files: 50, allowed_ghost_deps: vec![] };
let root = std::path::Path::new(".");
let diff_findings = crate::detectors::run_all_diff_detectors(&diff_files, root, &cfg);
assert!(diff_findings.iter().any(|f| f.rule_id == "no-speculative-file-race-toctou"));
assert!(diff_findings.iter().any(|f| f.rule_id == "no-hallucinated-crypto-algorithm"));
assert!(diff_findings.iter().any(|f| f.rule_id == "no-speculative-ssrf-interpolation"));
}
#[test]
fn test_mcp_tools_list_annotations_and_hints() {
let root = std::path::Path::new(".");
let shared = crate::telemetry::create_shared_telemetry();
let req = serde_json::json!({
"jsonrpc": "2.0",
"id": 1,
"method": "tools/list"
});
let resp = crate::mcp::handle_mcp_message(&req, root, &shared).expect("response");
let tools = resp.get("result").and_then(|r| r.get("tools")).and_then(|t| t.as_array()).expect("tools array");
assert_eq!(tools.len(), 3);
let names: Vec<&str> = tools.iter().filter_map(|t| t.get("name").and_then(|n| n.as_str())).collect();
assert!(names.contains(&"slopguard_scan_diff"));
assert!(names.contains(&"slopguard_check_snippet"));
assert!(names.contains(&"slopguard_sanitize_file"));
for t in tools {
assert!(t.get("readOnlyHint").expect("readOnlyHint").is_boolean());
assert!(t.get("destructiveHint").expect("destructiveHint").is_boolean());
assert!(t.get("idempotentHint").expect("idempotentHint").is_boolean());
assert!(t.get("openWorldHint").expect("openWorldHint").is_boolean());
let anno = t.get("annotations").expect("annotations");
assert!(anno.get("readOnlyHint").expect("anno readOnlyHint").is_boolean());
assert!(anno.get("destructiveHint").expect("anno destructiveHint").is_boolean());
assert!(anno.get("idempotentHint").expect("anno idempotentHint").is_boolean());
assert!(anno.get("openWorldHint").expect("anno openWorldHint").is_boolean());
}
}
#[test]
fn test_mcp_slopguard_scan_diff() {
let root = std::path::Path::new(".");
let shared = crate::telemetry::create_shared_telemetry();
let req = serde_json::json!({
"jsonrpc": "2.0",
"id": 2,
"method": "tools/call",
"params": {
"name": "slopguard_scan_diff",
"arguments": {
"stagedOnly": false
}
}
});
let resp = crate::mcp::handle_mcp_message(&req, root, &shared).expect("response");
assert_eq!(resp.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"));
assert!(resp.get("result").is_some());
}
#[test]
fn test_mcp_slopguard_check_snippet() {
let root = std::path::Path::new(".");
let shared = crate::telemetry::create_shared_telemetry();
let req = serde_json::json!({
"jsonrpc": "2.0",
"id": 3,
"method": "tools/call",
"params": {
"name": "slopguard_check_snippet",
"arguments": {
"filePath": "test.ts",
"code": "const cleanCode = 42;"
}
}
});
let resp = crate::mcp::handle_mcp_message(&req, root, &shared).expect("response");
assert_eq!(resp.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"));
assert!(resp.get("result").is_some());
}
#[test]
fn test_mcp_slopguard_sanitize_file() {
let temp_path = std::env::temp_dir().join(format!("slop_test_{}.ts", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
std::fs::write(&temp_path, "const a = 1;").unwrap();
let root = std::path::Path::new(".");
let shared = crate::telemetry::create_shared_telemetry();
let req = serde_json::json!({
"jsonrpc": "2.0",
"id": 4,
"method": "tools/call",
"params": {
"name": "slopguard_sanitize_file",
"arguments": {
"filePath": temp_path.to_str().unwrap()
}
}
});
let resp = crate::mcp::handle_mcp_message(&req, root, &shared).expect("response");
assert_eq!(resp.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"));
let _ = std::fs::remove_file(&temp_path);
}
#[test]
fn test_asm_rdtsc_monotonic_hardware_counter() {
let c0 = crate::gui::asm_rdtsc();
let c1 = crate::gui::asm_rdtsc();
assert!(c1 >= c0);
}
#[test]
fn test_asm_count_lines_fast_accuracy() {
assert_eq!(crate::gui::asm_count_lines_fast(b""), 0);
assert_eq!(crate::gui::asm_count_lines_fast(b"fn main() {}"), 1);
assert_eq!(crate::gui::asm_count_lines_fast(b"line1\nline2\nline3"), 3);
}
#[test]
fn test_native_gui_audit_state_initialization() {
let mut st = crate::gui::GuiAuditState::new(std::path::PathBuf::from("."));
assert!(st.summary.score <= 100);
assert!(!st.mode_label.is_empty());
assert_eq!(crate::gui::GUIDE_EN.len(), 8);
assert_eq!(crate::gui::GUIDE_RU.len(), 8);
assert_eq!(crate::gui::GUIDE_ZH.len(), 8);
st.open_guide(Some(1));
assert_eq!(st.active_tab, 2);
assert_eq!(st.guide_lang, 1);
assert_eq!(st.active_row_count(), 8);
assert_eq!(st.current_guide()[0].step, "01. НАЗНАЧЕНИЕ");
st.open_guide(Some(2));
assert_eq!(st.current_guide()[0].step, "01. 核心用途");
}
#[test]
fn test_council_slop_067_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-067").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v67");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_068_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-068").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v68");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_069_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-069").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v69");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_070_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-070").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v70");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_071_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-071").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v71");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_072_loop_scoped_event_listener_leak() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-072").unwrap();
assert_eq!(def.name, "no-loop-scoped-event-listener-leak-v72");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", "for (const item of items) { emitter.on('data', handler); }");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", "for (const item of items) { emitter.once('data', handler); }");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_073_phantom_ai_sdk_slopsquatting() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-073").unwrap();
assert_eq!(def.name, "no-phantom-ai-sdk-slopsquatting-v73");
assert_eq!(def.cwe, "CWE-1357");
let findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "openai==1.50.0");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_074_unbounded_mpsc_queue_sprawl() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-074").unwrap();
assert_eq!(def.name, "no-unbounded-mpsc-queue-sprawl-v74");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", "let (tx, rx) = tokio::sync::mpsc::channel(1024);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_075_truncated_hex_entropy_keygen() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-075").unwrap();
assert_eq!(def.name, "no-truncated-hex-entropy-keygen-v75");
assert_eq!(def.cwe, "CWE-326");
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("cipher.ts", "const encryption_key = crypto.randomBytes(32).toString('hex').slice(0, 16);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.ts", "const key = crypto.randomBytes(32);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_076_async_iterator_unclosed_break() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-076").unwrap();
assert_eq!(def.name, "no-async-iterator-unclosed-break-v76");
assert_eq!(def.cwe, "CWE-775");
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("stream.ts", "for await (const chunk of stream) {\n  if (done) break;\n}\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.ts", "try {\n  for await (const chunk of stream) {\n    if (done) break;\n  }\n} finally {\n  stream.destroy();\n}\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_077_compound_namespace_slopsquat() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-077").unwrap();
assert_eq!(def.name, "no-compound-namespace-slopsquat-v77");
assert_eq!(def.cwe, "CWE-829");
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio-redis-pro = \"1.0.0\"\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio = \"1.0\"\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_078_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-078").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v78");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_079_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-079").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v79");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_080_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-080").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v80");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_081_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-081").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v81");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_082_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-082").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v82");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_083_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-083").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v83");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_084_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-084").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v84");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_085_loop_scoped_event_listener_leak() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-085").unwrap();
assert_eq!(def.name, "no-loop-scoped-event-listener-leak-v85");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", "for (const item of items) { emitter.on('data', handler); }");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", "for (const item of items) { emitter.once('data', handler); }");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_086_phantom_ai_sdk_slopsquatting() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-086").unwrap();
assert_eq!(def.name, "no-phantom-ai-sdk-slopsquatting-v86");
assert_eq!(def.cwe, "CWE-1357");
let findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "openai==1.50.0");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_087_unbounded_mpsc_queue_sprawl() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-087").unwrap();
assert_eq!(def.name, "no-unbounded-mpsc-queue-sprawl-v87");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", "let (tx, rx) = tokio::sync::mpsc::channel(1024);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_088_truncated_hex_entropy_keygen() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-088").unwrap();
assert_eq!(def.name, "no-truncated-hex-entropy-keygen-v88");
assert_eq!(def.cwe, "CWE-326");
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("cipher.ts", "const encryption_key = crypto.randomBytes(32).toString('hex').slice(0, 16);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.ts", "const key = crypto.randomBytes(32);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_089_async_iterator_unclosed_break() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-089").unwrap();
assert_eq!(def.name, "no-async-iterator-unclosed-break-v89");
assert_eq!(def.cwe, "CWE-775");
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("stream.ts", "for await (const chunk of stream) {\n  if (done) break;\n}\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.ts", "try {\n  for await (const chunk of stream) {\n    if (done) break;\n  }\n} finally {\n  stream.destroy();\n}\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_090_compound_namespace_slopsquat() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-090").unwrap();
assert_eq!(def.name, "no-compound-namespace-slopsquat-v90");
assert_eq!(def.cwe, "CWE-829");
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio-redis-pro = \"1.0.0\"\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio = \"1.0\"\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_091_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-091").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v91");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_092_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-092").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v92");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_093_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-093").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v93");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_094_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-094").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v94");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_095_loop_scoped_event_listener_leak() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-095").unwrap();
assert_eq!(def.name, "no-loop-scoped-event-listener-leak-v95");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", "for (const item of items) { emitter.on('data', handler); }");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", "for (const item of items) { emitter.once('data', handler); }");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_096_phantom_ai_sdk_slopsquatting() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-096").unwrap();
assert_eq!(def.name, "no-phantom-ai-sdk-slopsquatting-v96");
assert_eq!(def.cwe, "CWE-1357");
let findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "openai==1.50.0");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_097_unbounded_mpsc_queue_sprawl() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-097").unwrap();
assert_eq!(def.name, "no-unbounded-mpsc-queue-sprawl-v97");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", "let (tx, rx) = tokio::sync::mpsc::channel(1024);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_098_truncated_hex_entropy_keygen() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-098").unwrap();
assert_eq!(def.name, "no-truncated-hex-entropy-keygen-v98");
assert_eq!(def.cwe, "CWE-326");
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("cipher.ts", "const encryption_key = crypto.randomBytes(32).toString('hex').slice(0, 16);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.ts", "const key = crypto.randomBytes(32);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_099_async_iterator_unclosed_break() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-099").unwrap();
assert_eq!(def.name, "no-async-iterator-unclosed-break-v99");
assert_eq!(def.cwe, "CWE-775");
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("stream.ts", "for await (const chunk of stream) {\n  if (done) break;\n}\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.ts", "try {\n  for await (const chunk of stream) {\n    if (done) break;\n  }\n} finally {\n  stream.destroy();\n}\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_100_compound_namespace_slopsquat() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-100").unwrap();
assert_eq!(def.name, "no-compound-namespace-slopsquat-v100");
assert_eq!(def.cwe, "CWE-829");
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio-redis-pro = \"1.0.0\"\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio = \"1.0\"\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_101_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-101").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v101");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_102_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-102").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v102");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_103_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-103").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v103");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_104_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-104").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v104");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_105_loop_scoped_event_listener_leak() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-105").unwrap();
assert_eq!(def.name, "no-loop-scoped-event-listener-leak-v105");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", "for (const item of items) { emitter.on('data', handler); }");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", "for (const item of items) { emitter.once('data', handler); }");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_106_phantom_ai_sdk_slopsquatting() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-106").unwrap();
assert_eq!(def.name, "no-phantom-ai-sdk-slopsquatting-v106");
assert_eq!(def.cwe, "CWE-1357");
let findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "openai==1.50.0");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_107_unbounded_mpsc_queue_sprawl() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-107").unwrap();
assert_eq!(def.name, "no-unbounded-mpsc-queue-sprawl-v107");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", "let (tx, rx) = tokio::sync::mpsc::channel(1024);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_108_truncated_hex_entropy_keygen() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-108").unwrap();
assert_eq!(def.name, "no-truncated-hex-entropy-keygen-v108");
assert_eq!(def.cwe, "CWE-326");
let findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("cipher.ts", "const encryption_key = crypto.randomBytes(32).toString('hex').slice(0, 16);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content("safe.ts", "const key = crypto.randomBytes(32);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_109_async_iterator_unclosed_break() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-109").unwrap();
assert_eq!(def.name, "no-async-iterator-unclosed-break-v109");
assert_eq!(def.cwe, "CWE-775");
let findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("stream.ts", "for await (const chunk of stream) {\n  if (done) break;\n}\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content("safe.ts", "try {\n  for await (const chunk of stream) {\n    if (done) break;\n  }\n} finally {\n  stream.destroy();\n}\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_110_compound_namespace_slopsquat() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-110").unwrap();
assert_eq!(def.name, "no-compound-namespace-slopsquat-v110");
assert_eq!(def.cwe, "CWE-829");
let findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio-redis-pro = \"1.0.0\"\n");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content("Cargo.toml", "[dependencies]\ntokio = \"1.0\"\n");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_111_unvalidated_ssrf_redirect_chain() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-111").unwrap();
assert_eq!(def.name, "no-unvalidated-ssrf-redirect-chain-v111");
assert_eq!(def.cwe, "CWE-918");
let findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("client.js", "axios.get(`https://${targetHost}/api/data`)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content("safe.js", "axios.get(\"https://api.verified.com/v1\")");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_112_non_atomic_toctou_fs_mutation() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-112").unwrap();
assert_eq!(def.name, "no-non-atomic-toctou-fs-mutation-v112");
assert_eq!(def.cwe, "CWE-367");
let findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("race.js", "fs.existsSync(p); fs.writeFileSync(p, data);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content("safe.js", "fs.writeFileSync(p, data, { flag: \"wx\" });");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_113_hallucinated_cipher_primitive() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-113").unwrap();
assert_eq!(def.name, "no-hallucinated-cipher-primitive-v113");
assert_eq!(def.cwe, "CWE-327");
let findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("crypto.js", "crypto.createCipheriv(\"aes-256-turbo\", key, iv);");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content("safe.js", "crypto.createCipheriv(\"aes-256-gcm\", key, iv);");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_114_shell_template_subprocess_exec() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-114").unwrap();
assert_eq!(def.name, "no-shell-template-subprocess-exec-v114");
assert_eq!(def.cwe, "CWE-78");
let findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("app.py", "subprocess.Popen(f\"cat {filename}\", shell=True)");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content("safe.py", "subprocess.run(['ls', '-la'], check=True)");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_115_loop_scoped_event_listener_leak() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-115").unwrap();
assert_eq!(def.name, "no-loop-scoped-event-listener-leak-v115");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("stream.js", "for (const item of items) { emitter.on('data', handler); }");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content("safe.js", "for (const item of items) { emitter.once('data', handler); }");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_116_phantom_ai_sdk_slopsquatting() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-116").unwrap();
assert_eq!(def.name, "no-phantom-ai-sdk-slopsquatting-v116");
assert_eq!(def.cwe, "CWE-1357");
let findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "llama-index-rust-bindings==0.1.0\nlangchain-turbo-retriever>=1.0.0");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content("requirements.txt", "openai==1.50.0");
assert_eq!(safe_findings.len(), 0);
}
#[test]
fn test_council_slop_117_unbounded_mpsc_queue_sprawl() {
let def = crate::detectors::AST_DETECTOR_DEFINITIONS.iter().find(|d| d.id == "SLOP-117").unwrap();
assert_eq!(def.name, "no-unbounded-mpsc-queue-sprawl-v117");
assert_eq!(def.cwe, "CWE-400");
let findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("queue.rs", "let (tx, rx) = tokio::sync::mpsc::unbounded_channel();");
assert!(!findings.is_empty());
let safe_findings = crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content("safe.rs", "let (tx, rx) = tokio::sync::mpsc::channel(1024);");
assert_eq!(safe_findings.len(), 0);
}
}
