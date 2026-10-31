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
    use crate::types::SlopCategory;
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
        let js_code = ["for (const item of items) {", "    await fetch('/api/process', { method: 'POST', body: JSON.stringify(item) });", "}"].join("\n");
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
}