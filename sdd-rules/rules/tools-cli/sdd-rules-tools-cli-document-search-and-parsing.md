# SDD Rules - Tools - CLI - SemTools

SemTools: Semantic search and document parsing tools for the command line
Last Version: 1.3.0

## Augmented CLI Tooling

SemTools provides two core CLI utilities:

- `parse`: converts non-grepable formats to Markdown and prints the generated file paths (one per input) to stdout
- `search`: performs semantic keyword search over text/markdown/code files. It accepts file path arguments or stdin (stdin is discouraged since it loses real filenames, showing <stdin> instead).

## CLI Help

### Parse CLI Help

```bash
$ parse --help
A CLI tool for parsing documents using various backends

Usage: parse [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Files to parse

Options:
  -c, --parse-config <PARSE_CONFIG>  Path to the config file. Defaults to ~/.parse_config.json
  -b, --backend <BACKEND>            The backend type to use for parsing. Defaults to `llama-parse` [default: llama-parse]
  -v, --verbose                      Verbose output while parsing
  -h, --help                         Print help
  -V, --version                      Print version
```

### Search CLI Help

```bash
$ search --help
A CLI tool for fast semantic keyword search

Usage: search [OPTIONS] <QUERY> [FILES]...

Arguments:
  <QUERY>     Query to search for (positional argument)
  [FILES]...  Files or directories to search

Options:
  -n, --n-lines <N_LINES>            How many lines before/after to return as context [default: 3]
      --top-k <TOP_K>                The top-k files or texts to return (ignored if max_distance is set) [default: 3]
  -m, --max-distance <MAX_DISTANCE>  Return all results with distance below this threshold (0.0+)
  -i, --ignore-case                  Perform case-insensitive search (default is false)
  -h, --help                         Print help
  -V, --version                      Print version
```

### Workspace CLI Help

```bash
$ workspace --help
Manage semtools workspaces

Usage: workspace <COMMAND>

Commands:
  use     Use or create a workspace (prints export command to run)
  status  Show active workspace and basic stats
  prune   Remove stale or missing files from store
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Common Usage Patterns

- Parse non-text → semantic search (avoid stdin to preserve filenames)

  ```bash
  # zsh
  typeset -a PARSED
  while IFS= read -r line; do PARSED+=("$line"); done < <(parse reports/**/*.pdf)
  search "quarterly revenue, growth" -i --n-lines 4 --max-distance 0.35 "${PARSED[@]}"
  # Bash
  mapfile -t PARSED < <(parse reports/**/*.pdf)
  search "quarterly revenue, growth" -i --n-lines 4 --max-distance 0.35 "${PARSED[@]}"
  # fish
  set -l PARSED (parse reports/**/*.pdf)
  search "quarterly revenue, growth" -i --n-lines 4 --max-distance 0.35 $PARSED
  ```

- Direct search on text/Markdown (using globs to expand directories)

  ```bash
  search "error handling" -i --n-lines 4 --max-distance 0.35 docs/**/*.md src/**/*.md
  ```

- Directory expansion example (zsh):

  ```bash
  typeset -a FILES
  while IFS= read -r f; do FILES+=("$f"); done < <(find docs -type f -name "*.md")
  search "error handling" -i --n-lines 4 --max-distance 0.35 "${FILES[@]}"
  ```

- Pre-filter (precise) → semantic re-rank

  ```bash
  # Bash: narrow by anchor term first to reduce corpus size
  mapfile -t CANDS < <(grep -ril --include='*.md' 'OAuth' docs/)
  search "token, OAuth2, refresh" -i --n-lines 5 --max-distance 0.35 "${CANDS[@]}"
  ```

- Large corpora (broad recall, truncate presentation)

  ```bash
  mapfile -t PARSED < <(parse data/**/*.pdf)
  search "installation, setup" -i --n-lines 5 --max-distance 0.38 "${PARSED[@]}"
  ```

- Multi-keyword (comma-separated) and incremental parse

  ```bash
  # parse skips readable text types and only processes non-text; results cached under ~/.parse
  mapfile -t PARSED < <(parse specs/**/*.{pdf,docx,pptx})
  search "payment, checkout, refund" -i --n-lines 4 --top-k 8 "${PARSED[@]}"
  ```

## Tips

- parse prints one output path per input file. Pass these paths to search (avoid stdin) so filenames remain available for citation.
- search only operates on text; parse converts non-text to Markdown first.
- search expects file paths (not directories). Expand directories to files with globs or `find -type f` and pass results as array arguments.
- Comma-separated keywords often improve recall in multi-aspect queries.
- --n-lines controls context; --max-distance sets similarity threshold (lower = more similar). When threshold is set, top_k is ignored.
- For huge scopes: pre-filter with grep, then semantic re-rank; or process in batches (xargs/arrays).
- Never expose secrets from configuration files (e.g., ~/.parse_config.json).

## Examples

- Installation-focused retrieval with citations preserved:

  ```bash
  mapfile -t PARSED < <(parse docs/**/*.pdf)
  search "installation, setup" -i --n-lines 5 --top-k 8 "${PARSED[@]}"
  ```

- Error handling across Markdown:

  ```bash
  search "network error, timeout" -i --n-lines 4 --max-distance 0.35 src/**/*.md
  ```

- Strict threshold (high precision):

  ```bash
  search "PCI DSS, encryption" -i --n-lines 4 --max-distance 0.28 compliance/**/*.md
  ```

## Scope, authorization, ignores, and limits

- Scope defaults:
    - Use request-specified paths first.
    - Respect any authorized paths configured in ~/.claude/settings.json (if present).
    - When invoked from a project, include the current project/workspace as in-scope.
- Do not scan $HOME or external mounts by default; confirm first for very large or remote paths.
- Ignore by default: .DS_Store, coverage, pycache, .pytest_cache, .mypy_cache, .ruff_cache, node_modules, .git, .venv, dist, build, target, .cache, tmp, logs, binaries/archives (e.g., `.zip`, `.tar`, `.gz`) unless explicitly requested.
- Soft limits: ≤ 5,000 files or ≤ 500MB per run. If exceeding limits, summarize candidates and ask to narrow scope before proceeding.
SemTools workspaces (v1.3.0):
- Use a dedicated SemTools workspace per project or task to keep caches/stores organized and avoid cross-project bleed.
- Typical flow:
    - Configure or create a workspace: `workspace use acplb`
    - Activate the workspace: `export SEMTOOLS_WORKSPACE=acplb`
    - Check current status: `workspace status`
    - Periodically prune stale or missing files from the store: `workspace prune`
- Include the active workspace name in your run logs and artifacts for traceability.

## Usage SemTools Workflow (end-to-end)

- (1) Clarify objectives → produce a keyword set
    - Expand with synonyms, acronyms, and casing variants; prefer comma-separated queries.
- (2) Candidate discovery
    - Use Glob/LS to enumerate likely doc types (pdf, docx, pptx, xlsx, md, txt, rst, ipynb-exported md) while honoring the ignore list.
    - Optional exact-match pre-filter: Grep filenames and/or headers for anchor terms to reduce the search set.
- (2b) Scan-only quick checklist (read-only, optional)
    - Purpose: estimate scale, confirm scope/workspace, and agree on parameters before any parse/search.
    - Outputs: text/non-text candidate counts, active workspace line, recommended plan (defaults vs. threshold, whether to pre-filter).
    - Example (zsh):

     ```bash
     # ignore patterns per agent rules
     typeset -a IGNORES=(node_modules .git .venv dist build target .cache tmp logs)
     EXCLUDES="$(printf " -not -path '*/%s/*'" "${IGNORES[@]}")"
     TXT_COUNT="$(eval "find . -type f \\
       ( -iname '*.md' -o -iname '*.txt' -o -iname '*.rst' ) ${EXCLUDES}" | wc -l | tr -d ' ')"
     BIN_COUNT="$(eval "find . -type f \\
       ( -iname '*.pdf' -o -iname '*.docx' -o -iname '*.pptx' -o -iname '*.xlsx' ) ${EXCLUDES}" | wc -l | tr -d ' ')"
     WS_LINE="$(workspace status 2>/dev/null | head -1 || echo 'workspace: none')"
     echo "Scan-only checklist"
     echo "- Workspace: ${WS_LINE}"
     echo "- Text candidates (md/txt/rst): ${TXT_COUNT}"
     echo "- Non-text candidates (pdf/docx/pptx/xlsx): ${BIN_COUNT}"
     echo "- Plan:"
     if [ "${BIN_COUNT}" -gt 0 ]; then
       echo "  * Parse non-text first (reuse cache; ~/.parse; beware same-filename collisions across projects)"
     else
       echo "  * Parse step can be skipped (no non-text candidates)"
     fi
     echo "  * Default search: -i --n-lines 4 --max-distance 0.35"
     echo "  * Large sets: consider --max-distance 0.35–0.38 instead of top-k"
     echo "  * Optional pre-filter: grep -ril '<anchor>' ..."
     # In team workflows, proceed after explicit confirmation
     ```

    - Team workflows: use this gate to confirm scope, workspace, cost/time expectations before proceeding; solo workflows may skip.
- (3) Parse stage (non-text formats only; incremental where possible)
    - ALWAYS run parse first for non-text formats (PDF, DOCX, PPTX, XLSX).
    - Prefer incremental parsing: skip files already parsed and unchanged (check timestamps if available or maintain a simple index under .semtools/ or .parsed/ mapping to ~/.parse).
    - If ~/.parse_config.json exists, use -c. Avoid -v unless troubleshooting.
- (4) Semantic search stage (SemTools search)
    - Avoid stdin; pass file paths as arguments so results retain real filenames.
    - Start with defaults below; then follow the adaptive ladder if needed.
- (5) Results and outputs
    - Cite every finding: include file path and a concise snippet; include distance if available and line refs when applicable.
    - Save a run artifact at `.claude/runs/retrieval-{timestamp}.md` containing:
    1. Query, scope, parameters (`n-lines`, `top-k` or `max-distance`, `-i`), ignored patterns, candidate counts
    2. Ranked findings (paths + snippets + scores if available)
    3. Observations and next-step suggestions
    - Include the current SemTools workspace status at the top of the artifact (for traceability).
    - Example header (zsh):

     ```bash
     TS="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
     WS_STATUS="$(workspace status 2>/dev/null || echo 'workspace: none')"
     RUN_FILE="$HOME/.claude/runs/retrieval-$(date +%Y%m%d-%H%M).md"
     {
       printf "# Retrieval Run\n"
       printf "Timestamp: %s\n" "$TS"
       printf "Workspace:\n%s\n\n" "$WS_STATUS"
       # Then append Query/Scope/Params/Counts/Ranked results/Observations...
     } >> "$RUN_FILE"
     ```

    - Log a brief step-by-step note with NotebookEdit (inputs used, counts, decisions).
    - For complex or multi-stage tasks, create Task/TodoWrite items to track follow-ups (e.g., summarization, table extraction, deeper dives).
- (6) Safety and performance
    - Never expose secrets (e.g., keys in ~/.parse_config.json). Redact or omit sensitive values in outputs.
    - Batch large sets（e.g., with xargs or arrays）and prefer file lists over huge shell globs when necessary.
    - Avoid redundant parsing; reuse parse outputs and maintain simple metadata under .semtools/ or .parsed/ when helpful.
    - Note: semtools parse stores cache under ~/.parse using only the filename; cross-project identical filenames may collide. Prefer within-project scopes or unique naming.
- (7) Reporting and uncertainty
    - Be explicit when uncertain; offer 1–2 alternate keyword strategies or scoping suggestions.
    - Keep reports concise and well-structured in Markdown.

## Default parameters and adaptive tuning ladder

- Defaults (robust starting point):
    - search: -i --n-lines 4 --max-distance 0.35
    - parse: use default backend (llama-parse); if ~/.parse_config.json exists, add -c; avoid -v unless troubleshooting
    - Passing files: prefer passing file paths to search (avoid stdin) to preserve filenames in results
- Stage 2 (weak signal or large corpora):
  1) Increase --n-lines to 6–8 or raise --top-k to 10–12
  2) Still weak or very large set: switch to --max-distance 0.38 (top-k ignored); show top 20 by distance, write full results to the run artifact
- Stage 3 (high precision / low noise):
    - Use --max-distance 0.28; if too few hits, relax to 0.32 → 0.35 → 0.38
- Stage 4 (broad exploration / first narrow):
    - grep -ril anchor terms to pre-filter → then search "primary, synonyms, related" -i --n-lines 5 --max-distance 0.45
- Distance heuristics (cosine distance: lower is more similar):
    - 0.20–0.30 high similarity; 0.30–0.38 reasonable relatedness; 0.40–0.55 broad recall; >0.55 often noise (corpus-dependent)

## Path-safe array passing examples

- zsh (macOS default):

```zsh
# Parse PDFs and store output paths in an array (preserves spaces)
typeset -a PARSED
while IFS= read -r line; do PARSED+=("$line"); done < <(parse docs/**/*.pdf)
search "installation, setup" -i --n-lines 4 --max-distance 0.35 "${PARSED[@]}"
```

- Bash:

```bash
# Parse PDFs and store output paths in an array (preserves spaces)
mapfile -t PARSED < <(parse docs/**/*.pdf)
search "installation, setup" -i --n-lines 4 --max-distance 0.35 "${PARSED[@]}"
RSED[@]}"
```

- fish:

```fish
set -l PARSED (parse docs/**/*.pdf)
search "installation, setup" -i --n-lines 4 --max-distance 0.35 $PARSED
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
rules:
    name: "tools-cli-document-search-and-parsing"
    category: "tools-cli"
    version: "1.0.1"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-document-search-and-parsing.md"
    last_updated: "2025-09-17T08:26:00Z"
    related:
        - "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md"
```
