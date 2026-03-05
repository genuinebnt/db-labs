---
description: Run cargo check, clippy, and tests to diagnose issues
---

# Check & Diagnose

Run diagnostics on the db-rs project without modifying any code.

// turbo-all

1. Run cargo check:

```bash
cd /Users/genuinebasilnt/projects/db-rs && cargo check 2>&1
```

2. Run clippy for lint suggestions:

```bash
cd /Users/genuinebasilnt/projects/db-rs && cargo clippy 2>&1
```

3. Run tests:

```bash
cd /Users/genuinebasilnt/projects/db-rs && cargo test 2>&1
```

Report the results to the user with:

- Compilation errors (if any) with conceptual hints on how to fix them
- Clippy warnings with links to the relevant Rust lint documentation
- Test failures with pointers to what concept might be wrong
