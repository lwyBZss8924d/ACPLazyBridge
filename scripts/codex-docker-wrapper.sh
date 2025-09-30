#!/bin/bash
# Wrapper script for running Codex CLI inside the acplb-codex-cli Docker image.
# Designed for ACPLazyBridge E2E tests and Zed integration.

set -euo pipefail

# Allow overriding via env vars; fall back to the canonical dev paths.
CODEX_ENV_FILE=${CODEX_ENV_FILE:-/Users/arthur/dev-space/codex/codex-cli/.env}
CODEX_CONFIG_PATH=${CODEX_CONFIG_PATH:-/Users/arthur/dev-space/codex/codex-cli/config.toml}
CODEX_WORKSPACE_DIR=${CODEX_WORKSPACE_DIR:-/Users/arthur/dev-space/acplb-test-workspace}
CODEX_DOCKER_IMAGE=${CODEX_DOCKER_IMAGE:-acplb-codex-cli}
CODEX_CONTAINER_NAME=${CODEX_CONTAINER_NAME:-acplb-codex-cli-test}

# Ensure OPENAI_API_KEY is available. Try loading from .env if not set.
if [[ -z "${OPENAI_API_KEY:-}" && -f "${CODEX_ENV_FILE}" ]]; then
  # Load and strip quotes from .env value
  OPENAI_API_KEY=$(grep -E ^OPENAI_API_KEY= "${CODEX_ENV_FILE}" | cut -d= -f2- | tr -d '"')
  export OPENAI_API_KEY
fi

if [[ -z "${OPENAI_API_KEY:-}" ]]; then
  echo "[codex-docker-wrapper] OPENAI_API_KEY is not set; cannot launch Codex CLI" >&2
  exit 1
fi

# Resolve paths.
CONFIG_REAL_PATH=$(realpath "${CODEX_CONFIG_PATH}")
if [[ ! -f "${CONFIG_REAL_PATH}" ]]; then
  echo "[codex-docker-wrapper] Codex config not found: ${CONFIG_REAL_PATH}" >&2
  exit 1
fi

mkdir -p "${CODEX_WORKSPACE_DIR}"
WORKSPACE_REAL_PATH=$(cd "${CODEX_WORKSPACE_DIR}" && pwd)

CODEX_HOME_DIR="${CODEX_HOME_DIR:-/tmp/acplb-codex-cli-home}"
mkdir -p "${CODEX_HOME_DIR}"
chmod 777 "${CODEX_HOME_DIR}"

# Always start with a clean container name.
docker rm -f "${CODEX_CONTAINER_NAME}" >/dev/null 2>&1 || true

COMMON_ARGS=(
  --name "${CODEX_CONTAINER_NAME}"
  --env "OPENAI_API_KEY=${OPENAI_API_KEY}"
  --env CODEX_UNSAFE_ALLOW_NO_SANDBOX=1
  --env CODEX_HOME=/home/node/.codex
  --env CODEX_CWD=/home/node/workspace
  --env CODEX_ALLOWED_DOMAINS="${OPENAI_ALLOWED_DOMAINS:-api.openai.com}"
  -v "${CODEX_HOME_DIR}:/home/node/.codex:rw"
  -v "${CONFIG_REAL_PATH}:/home/node/.codex/config.toml:ro"
  -v "${CODEX_ENV_FILE}:/home/node/.codex/.env:ro"
  -v "${WORKSPACE_REAL_PATH}:${WORKSPACE_REAL_PATH}:rw"
  -w "${WORKSPACE_REAL_PATH}"
  --user node
  "${CODEX_DOCKER_IMAGE}"
)

BIN_PATH=/usr/local/share/npm-global/bin/codex

if [[ "${1:-}" == "proto" ]]; then
  shift
  exec docker run -i --rm "${COMMON_ARGS[@]}" "${BIN_PATH}" proto "$@"
else
  exec docker run -i --rm "${COMMON_ARGS[@]}" "${BIN_PATH}" "$@"
fi
