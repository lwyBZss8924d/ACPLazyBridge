# Lifecycle (Authoritative)

This lifecycle defines the end-to-end workflow for every task (human or AI engineer).

📌 Wen any AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

---

Nodes and expected artifacts

1) Requirements analysis
   - Artifact: specs/<NNN>-<slug>/spec.md (WHAT/WHY; acceptance criteria)
   - Includes metadata (Issue-URI, Spec-URI, Evidence-URIs)

2) Product/architecture design
   - Artifact: specs/<NNN>-<slug>/design.md or data-model.md (optional)
   - May include contracts/diagrams/research

3) Technical plan
   - Artifact: specs/<NNN>-<slug>/plan.md (approach, components, trade-offs)

4) Issues
   - Artifact: GitHub Issue with links to Spec/Plan/Tasks/Evidence
   - Mirrors the metadata block from spec.md

5) Development (worktree-first)
   - Branch: feature|fix|perf|chore|docs/<kebab-slug>
   - Worktree from origin/main; follow quality gates

6) Acceptance & submit
   - Evidence placed under _artifacts/{tests,logs,jq,reports}/<task>/
   - PR description links Spec/Plan/Tasks and evidence; includes risks/rollback and CI summary

7) CI (GitHub Actions + Code Scanning)
   - Rust quality gates (fmt/clippy/test) across platforms (Ubuntu/macOS)
   - JSONL replay validation (protocol scenarios)
   - SDD validation gates (structure/language/semantic checks)
   - AST-grep code scanning with SARIF upload to GitHub Security
   - Report-only mode (current) → Enforcement mode (after Issue #31)
   - Evidence collection and artifact upload

8) PR review & merge
   - CODEOWNERS review; squash merge into main

9) Task end
   - Update specs/<NNN>-<slug>/tasks.md status
   - Close Issue; run drift checks if needed

References

- CONTRIBUTING.md
- sdd-rules/spec-template.md, sdd-rules/plan-template.md, sdd-rules/tasks-template.md

## CONSTITUTION - Supplementary Baseline

This section serves as the project’s "CONSTITUTION - Supplementary Baseline" and is maintained in this file (.specify/memory/lifecycle.md) per your directive. Teams may softlink or reference this section where "Supplementary Baseline" is expected.

### Operating base Rules

- CLI Tools: when using any command line tools, avoid interactive/paged commands; never expose secrets.
- Command allowlist & MCP servers: defer to <DeveloperTeamMembers AI-Engineer (Agents)>; do not duplicate here.
- Worktree-first: never develop on main; create a feature branch in a dedicated worktree.
- Branch categories (canonical): feature | fix | perf | chore | docs (kebab-case). The feature/<module>-<id> style is allowed as an alternative but not the canonical example.
- Logging discipline: stderr for logs; stdout reserved for JSON-RPC/JSONL only.
- Evidence: store all local scenario outputs and jq validations under _artifacts/{tests,logs,jq,reports}/<task>/.
- Respect human edits: do not override user modifications unless explicitly requested; reconcile conflicts conservatively.

### SDD compliance (must do for every task)

work in: (specs/)

- Create an SDD record under specs/<NNN>-<slug>/ with:
    - spec.md (WHAT/WHY; requirements and acceptance)
    - plan.md (technical plan; architecture and trade-offs)
    - tasks.md (subtasks, responsibilities, test plan)
- Add the following metadata block at the top of each file (and mirror in the GitHub Issue body):
    - Issue-URI: <link to the GitHub issue>
    - Spec-URI / Plan-URI / Tasks-URI: <self links>
    - Evidence-URIs: _artifacts/{tests,logs,jq,reports}/<task>/... linked with (specs/) TASK's artifacts outputs.
- PR description must include: links to Spec/Plan/Tasks, evidence files (tests/logs/jq/reports), risks/rollback, and CI pass summary.

### SDD commands (artifact generation)

- /specify — generate a new feature specification and branch/worktree; see sdd-rules/commands/specify.md
- /plan — create implementation plan and design docs; see sdd-rules/commands/plan.md
- /tasks — derive executable tasks from the plan; see sdd-rules/commands/tasks.md
- /sdd-task — initialize SDD task from GitHub issue; see .specify/commands/sdd-task.md

Notes:

- Use these commands to maintain the spec → plan → tasks flow described in (.specify/spec-driven.md) and (.specify/memory/lifecycle.md).

### Standard procedure

1) Context gathering
   - Inspect repository state, read relevant files, and list existing workflows.
2) Plan tasks
   - Draft a concise checklist; create a feature worktree from origin/main.
3) Implement & verify
   - Code changes via patch; run fmt/clippy/test; replay JSONL scenarios; record evidence.
4) Evidence
   - Store (specs/) TASK's artifacts outputs linked to (_artifacts/{tests,logs,jq,reports}/<task>/...; summarize pass/fail and link (specs/) TASK's (specs/) artifacts.
5) PR & merge
   - Open PR with summary and evidence; on approval, squash-merge and clean up worktrees.
   - After merge: MUST re-run the SDD Documentation Dynamic Consistency Check Workflow (.specify/memory/constitution_update_checklist.md) first! Then if needed to add any new sdd-rules or update .specify/memory/constitution.md and resync docs/templates if needed.

### SDD Rules

**SDD-RULES**: When AI engineers update the (specs/) Initialize Tasks & Process Tasks workflow process in accordance with the requirements and in strict compliance with the CONSTITUTION & "CONSTITUTION" - Link outher SDD decs, the SDD artifact: spec.md / plan.md / task.md needs to be explicitly linked to the specific rules (sdd-rules/rules/) {ssd-rules-xxx} if it needs to refer to specific rules. plan.md / task.md need to explicitly link to specific rules when (sdd-rules/rules/) {ssd-rules-xxx}

### Branch and worktree (canonical example)

For every formal TASK (e.g., `specs/<NNN>-<slug>/`), create a new worktree and branch off `origin/main`. (specs/) TASK's worktree branch Use existing GitHub Issues or create new ones, along with corresponding PRs, to track and manage the TASK’s status and progress any Issues and PRs comments fllow GitHub best practices.

- Branch categories: feature | fix | perf | chore | docs
- Create a new worktree and branch from origin/main:
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>
- Optional IDE navigation:
  ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

### Quality gates (must pass)

- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid JSONL.
- Code scanning (GitHub Code Scanning) is enabled.

### Constitutional gates (must pass)

- Library-First (Article I): Every feature begins as a standalone library or crate. See .specify/memory/constitution.md
- CLI Interface (Article II): Libraries expose CLI entrypoints with text I/O contracts. See .specify/memory/constitution.md
- Test-First (Article III): Write tests first and confirm failing (RED) before implementation. See .specify/memory/constitution.md
- Integration Testing (Article IV): Integration tests for contracts, cross-process workflows, and protocol behaviors. See .specify/memory/constitution.md
- Observability (Article V): stdout for protocol only; stderr for logs; structured logging for evidence. See .specify/memory/constitution.md
- Versioning (Article VI): Semantic versioning; documented breaking changes and migration paths. See .specify/memory/constitution.md
- Simplicity (Article VII): ≤3 projects; no future-proofing; avoid unnecessary patterns. See .specify/memory/constitution.md
- Anti-Abstraction (Article VIII): Use framework features directly; single model representation. See .specify/memory/constitution.md
- Integration-First (Article IX): Contracts defined; contract tests written before implementation; use real dependencies where practical. See .specify/memory/constitution.md

### SDD checks (pre-PR)

- scripts/ci/run-local-ci.sh — runs structure, language, markdown, semantic, and metadata checks
- Or on macOS, run individually:
    - scripts/sdd/validate-sdd-docs.sh — comprehensive SDD document validation (templates, metadata, structure)
    - scripts/sdd/check_language.sh — English-only policy enforcement
    - scripts/sdd/lint_docs.sh — markdown linting
    - scripts/sdd/run_semantic_checks.sh — cross-reference validation
    - scripts/sdd/validate-metadata.sh — validate YAML metadata format
    - scripts/sdd/check-sdd-consistency.sh — check global consistency

- Before submitting a PR, run the scripts in (scripts/sdd/) to perform the SDD consistency check and ensure compliance for (specs/) TASK's artifacts.

### Security & compliance

- Do not log secrets; never print secrets to CI logs; use env vars and GitHub secrets.
- Avoid running untrusted code or scripts without review.

### Communication

- Keep status short and actionable; when uncertain about intent, ask before proceeding.
- Escalate risks with options and trade-offs.

---

### SDD Team Workflows

#### New Feature Workflow (spec → plan → tasks → code)

1. **warp**: Co‑define requirements with human devs; capture the WHAT and WHY (no HOW). If needed, open/triage a GitHub Issue.
2. **warp**: Create a feature branch and worktree (auto‑numbered) and initialize `specs/NNN-feature/` using `/specify` or `/sdd-task <issue>` for issue-based initialization.
3. **claude**: Generate implementation plan via `/plan`, producing `plan.md`, and supporting docs (`data-model.md`, `contracts/`, `research.md`, `quickstart.md`).
4. **warp**: Validate plan against SDD gates (Simplicity, Anti‑Abstraction, Integration‑First, Test‑First). Mark ambiguities as `[NEEDS CLARIFICATION]`.
   - Library‑First Gate (Article I):
     - [ ] Feature implemented as a library first (package/module skeleton present)
     - [ ] Minimal testable structure exists (contract/integration scaffolds)
     - [ ] Build/test jobs include the library target
   - CLI Interface Gate (Article II):
     - [ ] CLI entrypoint(s) defined and discoverable (`<tool> --help`)
     - [ ] CLI supports stdin/stdout and JSON for structured IO
     - [ ] CLI contract tests present (help/usage snapshot + sample IO cases)
   - Integration Testing Gate (Article IV):
     - [ ] Integration tests written for new contracts
     - [ ] Cross-process workflows have test scenarios
     - [ ] Protocol/JSONL behaviors tested with realistic data
   - Observability Gate (Article V):
     - [ ] stdout reserved for protocol/tool output only
     - [ ] All logs directed to stderr
     - [ ] Structured logging configured for evidence capture
   - Versioning Gate (Article VI):
     - [ ] Version increment follows semantic versioning
     - [ ] Breaking changes documented with migration notes
     - [ ] Deprecation paths clearly defined
5. **claude**: Generate executable `tasks.md` via `/tasks`. Mark parallelizable tasks.
6. **claude**: Implement via strict TDD (contract → integration → e2e → unit), only writing code to make tests pass.
7. **warp**: Review artifacts in `specs/NNN-feature/`, update progress, and link the branch/commits to the Issue.
8. **warp**: Run local checks (structure, language policy, semantic, template drift). Push branch and open PR.
9. **warp + claude**: Monitor CI, process PR review, keep specs/tasks in sync with requested changes.
10. **warp**: Merge, clean up worktree, pull main, run SDD consistency pass, and update team‑wide SDD docs if required.

#### Bug Fix Workflow (spec‑first, reproduction‑driven)

Use the feature workflow adapted for bug reproduction and prevention. Code changes must be specification‑driven, not patch‑first.

1. **warp**: Open/triage a GitHub Issue. Create a bugfix worktree/branch `NNN-bug-[slug]`.
2. **warp**: In `specs/NNN-bug-[slug]/spec.md`, document:
   - Title, context, impacted versions, severity
   - Minimal Reproduction Steps (MRS)
   - Expected vs. Actual behavior
   - Scope (components, contracts, data)
   - Non‑functional impacts (perf, security, compatibility)
3. **claude**: Generate `plan.md` with root‑cause hypotheses and proposed fix strategies. Record validation points and potential regressions.
4. **claude**: Write failing tests first derived from MRS (contract/integration/e2e). No implementation until tests are red.
5. **claude**: Implement the fix to make tests pass; update contracts if behavior is clarified. Keep changes minimal per Simplicity/Anti‑Abstraction gates.
   - If the fix touches behavior contracts:
     - [ ] Update CLI help/usage and examples accordingly
     - [ ] Update CLI contract tests (help snapshot + sample IO)
     - [ ] Record rationale and impact in `spec.md`/`plan.md`
   - If the fix affects integration points (Article IV):
     - [ ] Update integration tests
     - [ ] Verify protocol behavior unchanged
   - If the fix affects logging/output (Article V):
     - [ ] Maintain stdout/stderr discipline
     - [ ] Update structured logging if needed
   - If the fix introduces breaking changes (Article VI):
     - [ ] Follow semantic versioning
     - [ ] Provide migration guide
6. **warp**: Ensure the change lands in a replaceable library unit (Article I) and the CLI surface remains consistent (Article II).
7. **warp**: Update `tasks.md` for the bugfix, mark status, and link commit messages to the Issue `[BUG-NNN]` (or `[TASK-XXX]` if unified).
8. **warp**: Run local CI (structure, language, semantic, drift). Push branch and open PR with reproduction, fix rationale, and test evidence.
9. **warp + claude**: Address PR feedback. If the bug implies spec ambiguity, update feature specs to remove `[NEEDS CLARIFICATION]` markers system‑wide.
10. **warp**: Merge, clean up branch. Backport if needed. Update CHANGELOG/Release notes.

#### SDD Documentation & CI Dynamic Consistency Update Workflow

Purpose: keep specifications, plans, tasks, and CI checks aligned with reality after any change (feature, fix, or refactor).

1. **Triggering Events**
   - PR merged to main; upstream template changes; ecosystem/library updates; constitution amendments; recurring drift or semantic alerts.

2. **Detection & Audit (local/CI)**
   - Run `scripts/ci/run-local-ci.sh` or enhanced `scripts/ci/run-sdd-gates.sh` to execute:
     - SDD structure lint (required directories, files)
     - Language policy (English‑only for normative artifacts)
     - Markdown lint (style, links)
     - Template drift (compare against upstream or pinned ref)
     - Semantic checks (broken cross‑refs, placeholders, `[NEEDS CLARIFICATION]`)
     - **Metadata validation** (`scripts/sdd/validate-metadata.sh`): YAML syntax, required fields, constitution version
     - **Consistency checks** (`scripts/sdd/check-sdd-consistency.sh`): document dependencies, cross-references
     - **AST-grep scanning** (`ast-grep scan -c sgconfig.yml`): code quality rules with SARIF reporting
     - Library‑First conformance (Article I): library modules present; packaging/build targets configured
     - CLI conformance (Article II): entrypoints exist and are executable; `--help` output matches documented usage/examples
     - Integration Testing conformance (Article IV): integration tests present and passing
     - Observability conformance (Article V): stdout/stderr properly separated; logs captured as evidence
     - Versioning conformance (Article VI): version numbers follow semver; breaking changes documented

3. **Documentation Sync**
   - For any deviation, update `specs/*/(spec|plan|tasks).md` and supporting docs (`research.md`, `data-model.md`, `contracts/`).
   - If CI workflows or governance changed, update `dev-docs/sdd/*` and project‑level `WARP.md`, `AGENTS.md`, `CLAUDE.md`.
   - Ensure updates are minimal and traceable; link Issues/PRs.

4. **Template & Manifest Alignment (optional)**
   - If improvements are generic, promote them into the template set under `templates/` (not repository‑specific roots).
   - Record template version and migration notes. Prepare `templates diff`/`templates update`.

5. **Validation & Publication**
   - Re‑run local checks. Open a PR focused on doc/CI consistency. Ensure passing SDD gates.
   - On merge, if templates changed, cut a release of templates (not repository‑specific content). Communicate channel (stable/canary).

6. **Roles**
   - **warp**: Orchestrates audits, updates normative docs, drives CI corrections, opens/merges doc PRs.
   - **claude**: Proposes concrete doc changes from diffs and runtime evidence; generates checklists and regression tests.

Outcome: documentation, plans, tasks, and CI checks remain a living, executable representation of the system, continuously synchronized with the implementation and upstream norms.

#### Metadata Validation Workflow

Purpose: Ensure all SDD documents maintain consistent YAML metadata for automated validation and querying.

1. **Metadata Structure**
   All SDD documents must include YAML metadata at the document footer:

   ```yaml
   constitution:
       version: "1.0.1"
       last_checked: "YYYY-MM-DDTHH:MM:SSZ"
   document:
       type: "document-type"
       path: "./relative/path.md"
       version: "X.Y.Z"
       last_updated: "YYYY-MM-DDTHH:MM:SSZ"
       dependencies:
           - "dependency1.md"
   ```

2. **Validation Tools**
   - `scripts/sdd/validate-metadata.sh`: Validate individual or all files
   - `scripts/sdd/query-metadata.sh`: Query documents by metadata fields
   - `scripts/sdd/check-sdd-consistency.sh`: Global consistency validation
   - `scripts/sdd/migrate-to-yaml-metadata.sh`: Migrate to unified format

3. **CI Integration**
   Metadata validation runs automatically:
   - On every PR modifying .md files
   - After merge to main
   - During `scripts/ci/run-local-ci.sh` execution

4. **Common Operations**

   ```bash
   # Validate all metadata
   ./scripts/sdd/validate-metadata.sh --check-consistency

   # Find outdated documents
   ./scripts/sdd/query-metadata.sh --outdated 30

   # Check global consistency
   ./scripts/sdd/check-sdd-consistency.sh --verbose

   # Migrate formats if needed
   ./scripts/sdd/migrate-to-yaml-metadata.sh --dry-run
   ```

5. **Roles**
   - **warp**: Monitors metadata consistency, triggers validation checks
   - **claude**: Updates metadata during document changes, runs validation tools

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema

---

```yaml
constitution:
    version: "1.0.1"
    ratified: "2025-09-15"
    last_amended: "2025-09-15"
document:
    type: "constitution-lifecycle"
    path: ".specify/memory/lifecycle.md"
    version: "1.0.3"
    last_updated: "2025-09-27T10:14:00Z"
    changelog: "Added validate-sdd-docs.sh for comprehensive SDD document validation"
    dependencies:
        - ".specify/memory/constitution.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
        - ".github/workflows/ci.yml"
        - "scripts/sdd/validate-sdd-docs.sh"
```
