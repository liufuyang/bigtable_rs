# Bigtable Sessions — Rust Port Plan

Design record for porting the Bigtable Sessions subsystem from `googleapis/google-cloud-go/bigtable/` to `bigtable_rs`. Companion to the five vendored specs at the repo root: `SESSION_SPEC.md`, `SESSION_CLIENT_SPEC.md`, `SESSION_POOL_SPEC.md`, `SESSION_COMPONENT_SPEC.md`, `CLIENT_SIDE_METRICS_SPEC.md`.

## Scope

**In scope — full port.** Session state machine, SessionClient, SessionPool with AFE picker (K-choice + PeakEwma), Diverter, TableShim, PoolSizer, ClientConfigurationManager, retry oracle, per-attempt + session-tracer OTel metrics, z-pages (`sessionz`, `afez`, `flightz`, `loadz`, `channelz`, `configz`, `tcpz`, `debugtagsz`), all boundary rules from `SESSION_COMPONENT_SPEC.md`.

**Out of scope — the direct-access checker only.** Drop `getClientConfigDirectAccessChecker` and its two config knobs (`DirectAccessCheckInterval`, `DirectAccessErrorThreshold`). Channel-pool warmup for the session channel pool becomes a no-op (no priming). Everything else — including `PeerInfo`/AFE ID plumbing, AFE picker, PeakEwma steering, session-tracer histograms — stays.

**Rollout.** Additive. The existing `bigtable_rs::bigtable::{BigTableConnection, BigTable}` public surface stays unchanged. The Sessions machinery lands under a new `sessions` Cargo feature (default-off in the first PRs, flipped default-on later). New public surface: `TableShim` (mirrors Go's mixed-mode entry point) plus `Table` / `SessionClient`.

## Proto strategy

`session.proto` is published in `googleapis/googleapis/google/bigtable/v2/session.proto` (contains OpenSession, CloseSession, GetClientConfiguration, GoAwayResponse, SessionRefreshConfig, and ~30 supporting types). The Rust `googleapis-tonic-google-bigtable-v2` crate hasn't been regenerated to include it.

**Plan:** vendor `session.proto` and its imports into `bigtable_rs/proto/`, add `tonic-build` at build time, and generate types into a `crate::proto::session` module. Continue depending on `googleapis-tonic-google-bigtable-v2` for the existing data-plane types (`ReadRowsRequest`, etc.); only session-family types come from the local build. Both crates share the same `google.bigtable.v2.*` package namespace at the proto level, but Rust module paths are distinct — no collision. `PeerInfo` and `ResponseParams` are already in the third-party crate; we'll re-export from there.

Feature-gate the `tonic-build` step and the generated module on `feature = "sessions"` so users who don't enable the feature don't pay the build-time cost.

## Module layout

Mirrors Go's `bigtable/internal/{transport,session}` + `bigtable/{table_shim.go,debugview,internal/metrics}`:

```
bigtable_rs/
├── build.rs                              # tonic-build for session.proto (feature-gated)
├── proto/
│   └── google/bigtable/v2/session.proto  # vendored
└── src/
    ├── bigtable.rs                       # UNCHANGED public surface (BigTable/BigTableConnection)
    ├── auth_service.rs                   # existing
    ├── proto/
    │   └── session.rs                    # tonic-generated; pub(crate)
    ├── transport/                        # SESSION_SPEC + SESSION_POOL_SPEC territory
    │   ├── mod.rs
    │   ├── session.rs                    # state machine, heartbeat, Invoke, hooks
    │   ├── session_vrpc.rs               # AttemptState tagging (SESSION_SPEC §9)
    │   ├── session_list.rs               # per-AFE afeHandle + I1–I6 (SESSION_POOL_SPEC §6)
    │   ├── session_pool.rs               # pool + waiter queue + picker dispatch
    │   ├── afe_picker.rs                 # SimpleAfePicker, LeastInFlight, LeastLatency, K-choice
    │   ├── peak_ewma.rs                  # PeakEwma tracker (transport/e2e seeds)
    │   ├── pick_decision.rs              # PickDecision, pickHistory ring
    │   ├── pool_sizer.rs                 # DesiredCapacity formula
    │   ├── budget.rs                     # NewSessionCreationBudget + circuit breaker
    │   ├── diverter.rs                   # policy: session vs classic
    │   ├── retrying.rs                   # RetryingVRpc + retry oracle
    │   ├── connpool.rs                   # channel pool (session-side, tonic balancer)
    │   ├── snapshot.rs                   # SessionDebugProvider + DTOs
    │   ├── heartbeat.rs                  # armed-only-during-vRPC watchdog
    │   ├── goaway.rs                     # GOAWAY 7-step handler
    │   └── session_tracer.rs             # 4 session-lifetime histograms
    ├── session/                          # SESSION_CLIENT_SPEC territory
    │   ├── mod.rs
    │   ├── client.rs                     # SessionClient + createSessionPoolForPayload
    │   ├── table.rs                      # sessionTable + stampAttempt (proto-native)
    │   ├── lazy_pool.rs                  # defers OpenSession until first use
    │   ├── client_configuration_manager.rs  # server config polling
    │   └── table_cache.rs                # self-healing sessionTableHandle cache
    ├── table_shim.rs                     # PUBLIC: mixed-mode router (mirrors Go)
    ├── table.rs                          # PUBLIC: public API (ReadRow/Apply/etc.)
    ├── metrics/                          # CLIENT_SIDE_METRICS_SPEC territory
    │   ├── mod.rs
    │   ├── tracer.rs                     # per-op Tracer
    │   ├── attempt_tracer.rs             # per-attempt AttemptTracer
    │   ├── factory.rs                    # client_uid + OTel registration
    │   ├── extract.rs                    # x-goog-cbt-* header parsing
    │   └── connectivity.rs               # 3-prong classifier
    └── debugview/                        # z-pages (behind `debug-pages` feature)
        ├── mod.rs
        ├── sessionz.rs
        ├── afez.rs
        ├── flightz.rs
        ├── loadz.rs
        ├── channelz.rs
        ├── configz.rs
        ├── tcpz.rs
        ├── debugtagsz.rs
        └── server.rs                     # axum HTTP server + templates
```

**Boundary discipline** (SESSION_COMPONENT_SPEC Part B, adapted from Go's package boundaries to Rust's `pub(crate)`/`pub(super)` visibility):

- `src/session/**` MUST stay proto-native — never imports `crate::table_shim` or user-facing types (B1).
- `src/transport/**` MUST NOT import `crate::session::*` (B2).
- `src/debugview/**` accesses pool/session state ONLY through `SessionDebugProvider` trait + snapshot DTOs (B3).
- `Diverter` surface = one input (SessionLoad), one output (`use_session() -> bool`), two counters. Zero RPC knowledge (B4).
- `TableShim` holds `Box<dyn SessionTableApi>`, never concrete `SessionPool`/`Session` (B5).
- `ClientConfigurationManager` is the SOLE writer of pool-shaping config (B6).
- `Session` MUST NOT hold pool-level counters; `afeHandle` holds per-AFE state (B7).
- Lock order: `pool.mu` before `sl.mu`; `sl.mu` innermost (B8).
- Retry classification lives at the vRPC boundary; RetryingVRpc reads via `ClassifyErr` (B9).
- Snapshot DTOs carry derived values; z-pages render only (B10).
- No re-implementation of runtime invariants in other layers (B11, B12).

## Concurrency model translation (Go → Rust)

Go primitives → Rust equivalents used throughout:

| Go | Rust |
|---|---|
| `atomic.Int32`/`Value` | `std::sync::atomic::AtomicI32` / `arc_swap::ArcSwap` |
| `sync.Mutex` | `parking_lot::Mutex` (short critical sections) / `tokio::sync::Mutex` (crossing await) |
| `sync.Once` | `std::sync::OnceLock` / `once_cell::sync::OnceCell` |
| `chan T` (unbuffered) | `tokio::sync::oneshot` / `mpsc(1)` |
| `context.Context` | `tokio_util::sync::CancellationToken` + explicit `Instant` deadlines |
| `go func()` | `tokio::spawn` with `JoinHandle` for teardown |
| `sync.WaitGroup` | `tokio::task::JoinSet` |
| `select { case <-ctx.Done(): ... }` | `tokio::select!` |

Session lifecycle needs `tokio::spawn` for the heartbeat watchdog and the OpenSession bidi stream reader — the existing crate has zero background tasks today, so this port defines the shutdown-token pattern.

## Direct-access checker exclusion

Concrete places we skip:

- Don't port `getClientConfigDirectAccessChecker`.
- Session channel pool: skip the warmup that Go does through the checker; no priming.
- `ClientConfiguration` polled fields: drop `DirectAccessCheckInterval` and `DirectAccessErrorThreshold` from the Rust `ClientConfiguration` type (either omit or accept-and-ignore — leaning toward accept-and-ignore for forward-compat if server sends them).
- Everything else that references AFE stays — AFE routing is a general Bigtable concept, not direct-access-specific.

## PR sequence

Layered from bottom to top so each PR is reviewable in isolation. Estimated line counts include tests.

| # | Title | Scope | Est. LoC | Depends on |
|---|---|---|---|---|
| 1 | Vendor + generate session protos | `proto/`, `build.rs`, `src/proto/session.rs` + smoke test | 800 | — |
| 2 | Transport: Session lifecycle | state machine, hooks, heartbeat, GOAWAY handler, `session.rs` + `heartbeat.rs` + `goaway.rs` + unit tests | 2,500 | 1 |
| 3 | Transport: Retry oracle | `AttemptState` tagging, `RetryingVRpc`, `session_vrpc.rs` + `retrying.rs` + unit tests | 1,500 | 2 |
| 4 | Transport: sessionList + PeakEwma + picker | `session_list.rs`, `peak_ewma.rs`, `afe_picker.rs`, `pick_decision.rs` | 2,000 | 2 |
| 5 | Transport: SessionPool + PoolSizer + budget + Diverter | `session_pool.rs`, `pool_sizer.rs`, `budget.rs`, `diverter.rs` | 2,500 | 3, 4 |
| 6 | Session layer: SessionClient + lazy pools + config manager | `session/client.rs`, `session/lazy_pool.rs`, `session/table.rs`, `session/client_configuration_manager.rs`, `session/table_cache.rs` | 2,500 | 5 |
| 7 | Public surface: TableShim + Table API | `table_shim.rs`, `table.rs` (ReadRow/Apply/ReadRows/SampleRowKeys/etc.), public types | 1,500 | 6 |
| 8 | Metrics: classic-path Tracer + AttemptTracer | `metrics/tracer.rs`, `metrics/attempt_tracer.rs`, `metrics/factory.rs`, `metrics/extract.rs`, `metrics/connectivity.rs` + 5 OTel histograms | 2,500 | 5 |
| 9 | Metrics: session-tracer histograms | `transport/session_tracer.rs` + 4 histograms + Wire into Session/pool | 1,000 | 6, 8 |
| 10 | Debug surface: SessionDebugProvider + snapshot DTOs | `transport/snapshot.rs`, provider trait, snapshot types | 800 | 6 |
| 11 | Debug surface: z-pages HTTP server | `debugview/*.rs`, axum handlers, HTML templates | 3,500 | 10 |
| 12 | Emulator integration tests | round-trip tests: OpenSession → vRPC → GOAWAY → close; pool scaling; picker distribution | 1,500 | 7 |

**Rough total: ~22,600 lines.** Realistically 8–12 weeks for one engineer, longer with review cycles.

## Risks and unknowns

1. **`session.proto` server-side availability.** The generated types will compile, but calling `OpenSession` against the production Bigtable service requires the server to accept these RPCs. On the public `bigtable.googleapis.com` endpoint this may fail with `Unimplemented`. Verify against the emulator first; verify against production once we have credentials.
2. **Bigtable emulator support for sessions.** The Go tests presumably run against the emulator with a session-aware build. Confirm the version we ship in CI (`google/cloud-sdk:latest`) has session support — if not, we may need to pin a specific image.
3. **Background-task shutdown.** The current crate has none. PR 2 needs to establish the shutdown-token pattern that PRs 5, 6, 8, 9 will follow. Getting this wrong once means every subsequent PR inherits the bug.
4. **Lock granularity crossing await.** Go uses `sync.Mutex` freely; Rust needs `tokio::sync::Mutex` when the lock must be held across `.await`. Getting confused between `parking_lot::Mutex` (fast, no-await) and `tokio::sync::Mutex` (await-safe) is a common porting mistake. B8's lock-order rule needs a per-mutex annotation in the code.
5. **Java-parity requirements in the specs.** SESSION_SPEC #6 (`LastRpcIdAdmitted` is "deprecated / not implemented in Go") explicitly says any reintroduction requires paired code+spec+test with Java-parity verification. We should follow the same rule — if we implement any behavior the Go client doesn't, we're diverging from Java, which is not what parity means.
6. **z-pages templating.** Go's `text/template` doesn't map cleanly to a Rust equivalent; `askama` or `minijinja` is close. Templates are a chunk of PR 11 that could balloon if we try for pixel-perfect parity vs "same information, Rust-idiomatic layout."
7. **OTel SDK version churn.** `opentelemetry` Rust ecosystem is still stabilizing; picking versions that stay compatible over the course of a 12-week port is a coordination cost.

## Open questions requiring user input before starting implementation

None currently blocking — scope is settled. Any of the risks above may escalate into questions as we hit them.

## Companion documents

- `SESSION_SPEC.md` — per-Session lifecycle (10 invariants)
- `SESSION_CLIENT_SPEC.md` — SessionClient topology/config/handshake (4 invariants)
- `SESSION_POOL_SPEC.md` — pool topology, picking, routing, scaling (5 invariants)
- `SESSION_COMPONENT_SPEC.md` — 12 boundary MUSTs + ownership matrix
- `CLIENT_SIDE_METRICS_SPEC.md` — per-attempt metrics field provenance (3 invariants)
