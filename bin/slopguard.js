#!/usr/bin/env node
import { runCli } from "../dist/cli.js";
runCli().catch((err) => {
  process.stderr.write(`[SlopGuard Fatal]: ${err?.message || err}\n`);
  process.exit(1);
});