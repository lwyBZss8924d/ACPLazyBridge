#!/usr/bin/env bash
# Drive Zed ↔ ACPLazyBridge ↔ Codex Docker end-to-end test while capturing logs.

set -euo pipefail

ACPLB_ROOT=${ACPLB_ROOT:-/Users/arthur/dev-space/ACPLazyBridge}
CODEX_ROOT=${CODEX_ROOT:-/Users/arthur/dev-space/codex/codex-cli}
WORKSPACE=${ACPLB_TEST_WORKSPACE:-/Users/arthur/dev-space/acplb-test-workspace}
WRAPPER=${ACPLB_ROOT}/scripts/codex-docker-wrapper.sh

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
EVIDENCE_DIR="${ACPLB_ROOT}/_artifacts/tests/T033/interactive-${TIMESTAMP}"
LOG_ACPLB="${EVIDENCE_DIR}/acplb.log"
LOG_DOCKER="${EVIDENCE_DIR}/docker.log"
LOG_ZED="${EVIDENCE_DIR}/zed-acp.log"

mkdir -p "${EVIDENCE_DIR}" "${WORKSPACE}"

log()  { echo -e "[1;33m$*[0m"; }
ok()   { echo -e "[0;32m$*[0m"; }
err()  { echo -e "[0;31m$*[0m"; }

log "[1/7] Building Codex E2E test image (see docker_build.log)"
(
  cd "${CODEX_ROOT}"
  ./scripts/build_acplb_codex_cli_container.sh >"${EVIDENCE_DIR}/docker_build.log" 2>&1
)
ok "acplb-codex-cli ready"

log "[2/7] Building ACPLazyBridge release binary (see cargo_build.log)"
(
  cd "${ACPLB_ROOT}"
  cargo build --release -p codex-cli-acp >"${EVIDENCE_DIR}/cargo_build.log" 2>&1
)
ok "codex-cli-acp built"

log "[3/7] Starting ACPLazyBridge adapter (foreground logs -> ${LOG_ACPLB})"
CODEX_RUN="${WRAPPER}" ACPLB_IDLE_TIMEOUT_MS=${ACPLB_IDLE_TIMEOUT_MS:-60000} ACPLB_NOTIFY_INJECT=${ACPLB_NOTIFY_INJECT:-auto} RUST_LOG=${RUST_LOG:-info} "${ACPLB_ROOT}/target/release/codex-cli-acp" --acp   >"${LOG_ACPLB}" 2>&1 &
ACPLB_PID=$!
trap 'kill ${ACPLB_PID} >/dev/null 2>&1 || true' EXIT
sleep 1

log "[4/7] Tailing Docker logs once container starts (-> ${LOG_DOCKER})"
(docker logs -f acplb-codex-cli-test >"${LOG_DOCKER}" 2>&1) &
DOCKER_TAIL_PID=$!
trap 'kill ${DOCKER_TAIL_PID} >/dev/null 2>&1 || true' EXIT

log "[5/7] Launching Zed in foreground; logs captured in ${LOG_ZED}"
log "     Use the GUI to open ‘dev: open acp logs’ from the palette."
log "     When you are done testing, press Ctrl-C in this terminal."

( script -q "${LOG_ZED}" zed "${WORKSPACE}" --foreground )

log "[6/7] Stopping background processes"
kill ${ACPLB_PID} >/dev/null 2>&1 || true
kill ${DOCKER_TAIL_PID} >/dev/null 2>&1 || true
trap - EXIT

log "[7/7] Artifacts captured at ${EVIDENCE_DIR}"
ok "ACPLazyBridge log  : ${LOG_ACPLB}"
ok "Docker Codex log   : ${LOG_DOCKER}"
ok "Zed ACP log       : ${LOG_ZED}"
