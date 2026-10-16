# Sailry integration patches

Based on SQLx v0.9.0 (`75bc0487eb661da811bb7a3c5d158f1bd463fef4`).
All upstream license notices remain unchanged.

- Accept libsqlite3-sys 0.38.x, matching upstream commit
  `94aafe3a68884d923b0798a767c8d7f6cfda89d2` (PR #4402).
- PostgreSQL and MySQL need separate TCP addresses and TLS host identities for
  authenticated SSH tunnels. `connect_addr` changes only TCP routing; TLS keeps
  the original `host`. SQLx v0.9.0 has no public equivalent.
- `ssl_verify_server(true)` preserves certificate and hostname verification when
  TLS is preferred but optional. Upstream's preferred modes negotiate TLS while
  accepting invalid certificates. A certificate error never retries plaintext.
  The default remains upstream behavior for other callers.

- `SqliteConnectOptions::no_follow` forwards SQLite's existing
  `SQLITE_OPEN_NOFOLLOW` flag, which v0.9.0 does not expose. This keeps file creation
  from following a substituted symbolic link after application path checks.

These additions are transport and file-open options only. Sailry uses ordinary SQLx drivers,
queries and native-handle access; no application workflow belongs in this fork.
