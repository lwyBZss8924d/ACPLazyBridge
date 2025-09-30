#!/bin/bash
# ACPLazyBridge end-to-end smoke test for Codex Docker integration (T033).
# This script orchestrates container build, adapter build, wrapper validation,
# and the streaming notification regression test.

set -euo pipefail

SCRIPT_DIR=$(dirname "$(realpath "$0")")
REPO_ROOT="${SCRIPT_DIR}/.."
CODEX_ROOT=${CODEX_ROOT:-/Users/arthur/dev-space/codex/codex-cli}
EVIDENCE_ROOT="${REPO_ROOT}/_artifacts/tests/T033"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
EVIDENCE_DIR="${EVIDENCE_ROOT}/${TIMESTAMP}"

mkdir -p "${EVIDENCE_DIR}"

log_info() { echo -e "[1;33m$*[0m"; }
log_ok()   { echo -e "[0;32m$*[0m"; }
log_err()  { echo -e "[0;31m$*[0m"; }

log_info "[1/6] Building Codex E2E test Docker image (logs: ${EVIDENCE_DIR}/docker_build.log)"
(
  cd "${CODEX_ROOT}"
  ./scripts/build_acplb_codex_cli_container.sh >"${EVIDENCE_DIR}/docker_build.log" 2>&1
)
log_ok "Docker image acplb-codex-cli:latest ready"

log_info "[2/6] Building codex-cli-acp release binary (logs: ${EVIDENCE_DIR}/cargo_build.log)"
(
  cd "${REPO_ROOT}"
  cargo build --release -p codex-cli-acp >"${EVIDENCE_DIR}/cargo_build.log" 2>&1
)
log_ok "codex-cli-acp built successfully"

log_info "[3/6] Verifying Docker image executes Codex CLI"
if docker run --rm acplb-codex-cli /usr/local/share/npm-global/bin/codex --version >"${EVIDENCE_DIR}/docker_verify.log" 2>&1; then
  log_ok "Docker codex CLI responds"
else
  log_err "Docker codex CLI verification failed (see docker_verify.log)"
  exit 1
fi

WRAPPER_PATH="${REPO_ROOT}/scripts/codex-docker-wrapper.sh"
log_info "[4/6] Sanity testing docker wrapper (${WRAPPER_PATH})"
if [[ ! -x "${WRAPPER_PATH}" ]]; then
  log_err "Wrapper script missing or not executable: ${WRAPPER_PATH}"
  exit 1
fi

if printf '{"id":"ping","op":{"type":"user_input","items":[{"type":"text","text":"ping"}]}}
'     | timeout 10 "${WRAPPER_PATH}" proto >"${EVIDENCE_DIR}/wrapper_ping.jsonl" 2>&1; then
  log_ok "Wrapper produced output (see wrapper_ping.jsonl)"
else
  log_info "Wrapper ping timed out (acceptable for quick proto test)"
fi

log_info "[5/6] Running t033c_streaming_notifications regression test"
(
  cd "${REPO_ROOT}"
  export CODEX_RUN="${WRAPPER_PATH}"
  export CODEX_CMD="${WRAPPER_PATH}" # backwards compatibility
  cargo test --release -p codex-cli-acp --test codex_cli_smoke_test --     --ignored t033c_streaming_notifications --nocapture     >"${EVIDENCE_DIR}/smoke_test.log" 2>&1
)
log_ok "Streaming notifications test completed"

log_info "[6/6] Summarizing results"
if grep -q "test t033c_streaming_notifications ... ok" "${EVIDENCE_DIR}/smoke_test.log"; then
  log_ok "T033c passed"
else
  log_err "T033c test did not pass; inspect smoke_test.log"
fi

grep -o "Found [0-9]* SessionNotifications" "${EVIDENCE_DIR}/smoke_test.log" || true

log_info "Evidence stored under ${EVIDENCE_DIR}"
