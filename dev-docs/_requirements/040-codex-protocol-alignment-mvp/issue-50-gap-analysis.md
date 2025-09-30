# Issue #50 - Codex Protocol Alignment MVP: Gap Analysis

**Date:** 2025-09-30
**Issue URI:** (to be created) #50
**Related Specs:** `specs/040-codex-protocol-alignment-mvp/`
**Analysis Artifacts:**
- Codex Protocol Mapping: `_artifacts/reports/codex-protocol-analysis/`
- ACP Protocol Mapping: `_artifacts/reports/acp-protocol-complete-mapping.md`
- Claude-Code-ACP Reference: `_artifacts/reports/claude-code-acp-analysis/`

---

## Executive Summary

This gap analysis compares the **current codex-cli-acp implementation** against the **complete Codex CLI capabilities** and **ACP protocol requirements** to identify what needs to be implemented for the Milestone 0.1.0 MVP.

**Current Coverage:** ~20% of Codex event types, ~60% of ACP SessionUpdate variants
**MVP Target:** 100% of core Codex events, 100% of ACP requirements
**Implementation Effort:** Medium-High (estimated 2-3 weeks)

---

## 1. Current Implementation State

### ✅ What Works Today

**Event Handling ([codex_proto.rs](../../crates/codex-cli-acp/src/codex_proto.rs)):**
```rust
// Currently mapped Codex events → ACP
CodexEvent::AgentMessage { message }           → SessionUpdate::AgentMessageChunk
CodexEvent::AgentMessageDelta { delta }        → SessionUpdate::AgentMessageChunk
CodexEvent::AgentReasoning { text }            → SessionUpdate::AgentThoughtChunk
CodexEvent::AgentReasoningDelta { delta }      → SessionUpdate::AgentThoughtChunk
CodexEvent::ToolCall { id, name, arguments }   → SessionUpdate::ToolCall
CodexEvent::ToolCalls { calls }                → SessionUpdate::ToolCall (batch)
CodexEvent::PlanUpdate { plan }                → SessionUpdate::Plan
CodexEvent::McpListToolsResponse { tools }     → SessionUpdate::AvailableCommandsUpdate
CodexEvent::SessionConfigured { model }        → SessionUpdate::CurrentModeUpdate
CodexEvent::TaskComplete { reason }            → (finalize flag)
CodexEvent::Error { message, code }            → ToolCallUpdate (failed)
```

**Tool Call Features ([tool_calls.rs](../../crates/codex-cli-acp/src/tool_calls.rs)):**
- ✅ Shell command extraction (`extract_shell_command`)
- ✅ Shell parameter parsing (`extract_shell_params`: workdir, timeout, sudo)
- ✅ Basic tool kind mapping (10 categories)
- ✅ UTF-8 safe output truncation
- ✅ Raw input/output preservation

**Runtime ([codex_agent.rs](../../crates/codex-cli-acp/src/codex_agent.rs)):**
- ✅ Permission mode mapping (ACP → Codex CLI flags)
- ✅ Process spawning with proto mode
- ✅ Notify integration (file/FIFO turn completion)
- ✅ Idle timeout handling
- ✅ Cancellation support

**Strengths:**
1. Solid foundation for event streaming
2. Good tool call lifecycle tracking (pending → in_progress → completed)
3. Shell tool parameter extraction is comprehensive
4. Notify-based turn completion works reliably

---

## 2. Missing Codex Event Mappings

### 🔴 Critical Gaps (Blocking MVP)

These event types are **essential** for a complete Codex workflow but **not yet handled**:

#### 2.1. Execution Tool Lifecycle
**Codex Events:**
```rust
// Not yet mapped:
ExecCommandBegin {
    id: String,
    command: Vec<String>,  // Array form, needs joining
    cwd: String,
    timeout_ms: Option<u64>,
    with_escalated_permissions: bool,
}

ExecCommandStdout {
    id: String,
    content: String,        // Incremental stdout
}

ExecCommandStderr {
    id: String,
    content: String,        // Incremental stderr
}

ExecCommandEnd {
    id: String,
    exit_code: i32,
    output: Option<ExecOutput>,  // Contains stdout/stderr/exit_code
}

ExecApprovalRequest {
    id: String,
    command: Vec<String>,
    cwd: String,
    timeout_ms: Option<u64>,
    with_escalated_permissions: bool,
}

ExecApproved {
    id: String,
    approved: bool,
}
```

**Required ACP Mapping:**
```rust
// Begin → ToolCall (pending)
ExecCommandBegin → SessionUpdate::ToolCall {
    id: id,
    title: format!("{}", command.join(" ")),
    kind: ToolKind::Execute,
    status: ToolCallStatus::Pending,
    locations: vec![ToolCallLocation { path: cwd, line: None }],
    raw_input: json!({
        "command": command,
        "cwd": cwd,
        "timeout_ms": timeout_ms,
        "with_escalated_permissions": with_escalated_permissions
    }),
    content: vec![],
    raw_output: None,
}

// Stdout/Stderr → ToolCallUpdate (in_progress + incremental content)
ExecCommandStdout → SessionUpdate::ToolCallUpdate {
    id: id,
    fields: ToolCallUpdateFields {
        status: Some(ToolCallStatus::InProgress),
        content: Some(vec![ToolCallContent::from(content)]),
        ..Default::default()
    },
}

// End → ToolCallUpdate (completed/failed + final output)
ExecCommandEnd → SessionUpdate::ToolCallUpdate {
    id: id,
    fields: ToolCallUpdateFields {
        status: Some(if exit_code == 0 { Completed } else { Failed }),
        content: Some(vec![
            ToolCallContent::from(format!("exit code: {}", exit_code))
        ]),
        raw_output: Some(json!({
            "stdout": output.stdout,
            "stderr": output.stderr,
            "exit_code": exit_code
        })),
        ..Default::default()
    },
}

// Approval → client.requestPermission() then send ExecApproved op back to Codex
ExecApprovalRequest → (call ACP permission flow, await response, submit approval)
```

**Gap Impact:** **HIGH** - Without this, shell commands appear as single events with no streaming output or approval flow.

---

#### 2.2. Patch/Edit Tool Lifecycle
**Codex Events:**
```rust
// Not yet mapped:
PatchApplyBegin {
    id: String,
    changes: HashMap<String, Vec<FileChange>>,  // path → changes
}

PatchApplyProgress {
    id: String,
    file_path: String,
    change_index: usize,
}

PatchApplyEnd {
    id: String,
    success: bool,
    applied_files: Vec<String>,
}

PatchApprovalRequest {
    id: String,
    changes: HashMap<String, Vec<FileChange>>,
}

PatchApproved {
    id: String,
    approved: bool,
}

// FileChange variants:
enum FileChange {
    Add { path: String, content: String },
    Update { path: String, old_lines: Range, new_content: String },
    Delete { path: String },
}
```

**Required ACP Mapping:**
```rust
// Begin → ToolCall per file (or one with multi-file content)
PatchApplyBegin → {
    for (path, changes) in changes {
        let kind = match changes.first() {
            FileChange::Add { .. } => ToolKind::Edit,
            FileChange::Update { .. } => ToolKind::Edit,
            FileChange::Delete { .. } => ToolKind::Delete,
        };

        SessionUpdate::ToolCall {
            id: format!("{}:{}", id, path),
            title: format!("Edit {}", path),
            kind: kind,
            status: ToolCallStatus::Pending,
            locations: vec![ToolCallLocation { path, line: changes.start_line }],
            content: vec![ContentBlock::Diff {
                path: path,
                oldText: compute_old_text(changes),  // Need file content cache
                newText: compute_new_text(changes),
            }],
            raw_input: json!({ "changes": changes }),
        }
    }
}

// Progress → ToolCallUpdate (in_progress)
PatchApplyProgress → SessionUpdate::ToolCallUpdate {
    id: format!("{}:{}", id, file_path),
    fields: ToolCallUpdateFields {
        status: Some(ToolCallStatus::InProgress),
        ..Default::default()
    },
}

// End → ToolCallUpdate (completed/failed)
PatchApplyEnd → {
    for path in applied_files {
        SessionUpdate::ToolCallUpdate {
            id: format!("{}:{}", id, path),
            fields: ToolCallUpdateFields {
                status: Some(if success { Completed } else { Failed }),
                ..Default::default()
            },
        }
    }
}

// Approval → client.requestPermission() with diff preview
PatchApprovalRequest → (call ACP permission flow with full diff content)
```

**Gap Impact:** **HIGH** - File edits are core to Codex workflows; without this, no apply-patch approval flows work.

**Additional Requirement:** **File content cache** needed to generate diffs (see claude-code-acp pattern).

---

#### 2.3. MCP Tool Call Lifecycle
**Codex Events:**
```rust
// Not yet mapped:
McpToolCallBegin {
    id: String,
    server: String,
    tool: String,
    arguments: Value,
}

McpToolCallEnd {
    id: String,
    result: Option<Value>,
    error: Option<String>,
}
```

**Required ACP Mapping:**
```rust
// Begin → ToolCall
McpToolCallBegin → SessionUpdate::ToolCall {
    id: id,
    title: format!("{}:{}", server, tool),
    kind: infer_tool_kind_from_name(&tool),  // Heuristic mapping
    status: ToolCallStatus::Pending,
    raw_input: Some(arguments),
    content: vec![],
}

// End → ToolCallUpdate
McpToolCallEnd → SessionUpdate::ToolCallUpdate {
    id: id,
    fields: ToolCallUpdateFields {
        status: Some(if error.is_some() { Failed } else { Completed }),
        content: if let Some(err) = error {
            Some(vec![ToolCallContent::from(format!("Error: {}", err))])
        } else {
            Some(vec![ToolCallContent::from(format_mcp_result(result))])
        },
        raw_output: Some(result.or(json!({ "error": error }))),
        ..Default::default()
    },
}
```

**Gap Impact:** **MEDIUM** - MCP tools work via `McpListToolsResponse` (already handled), but call lifecycle is missing.

---

#### 2.4. Web Search Lifecycle
**Codex Events:**
```rust
// Not yet mapped:
WebSearchBegin {
    id: String,
    query: String,
}

WebSearchEnd {
    id: String,
    results: Vec<SearchResult>,
}
```

**Required ACP Mapping:**
```rust
WebSearchBegin → SessionUpdate::ToolCall {
    id: id,
    title: format!("Search: {}", query),
    kind: ToolKind::Fetch,
    status: ToolCallStatus::Pending,
    raw_input: Some(json!({ "query": query })),
}

WebSearchEnd → SessionUpdate::ToolCallUpdate {
    id: id,
    fields: ToolCallUpdateFields {
        status: Some(ToolCallStatus::Completed),
        content: Some(vec![ToolCallContent::from(format_search_results(results))]),
        raw_output: Some(json!({ "results": results })),
        ..Default::default()
    },
}
```

**Gap Impact:** **LOW** - Web search is optional feature, but good to have for completeness.

---

#### 2.5. User Input Events
**Codex Events:**
```rust
// Partially handled (UserMessage), but missing fields:
UserMessage {
    message: String,
    kind: Option<String>,         // Not used yet
    images: Option<Vec<String>>,  // Mapped to ImageContent
}

// Not yet mapped:
UserFileAttachment {
    path: String,
    content: String,              // @-mention file content
}

UserInputReceived {
    input: String,                // Echo user input
}
```

**Required ACP Mapping:**
```rust
UserFileAttachment → SessionUpdate::UserMessageChunk {
    content: ContentBlock::Resource(EmbeddedResource {
        resource: TextResourceContents {
            text: content,
            uri: format!("file://{}", path),
            mime_type: Some(infer_mime_type(path)),
        },
    }),
}

UserInputReceived → (optional, for echo/confirmation; may skip)
```

**Gap Impact:** **LOW** - File attachments are implicit in Codex submissions; this is metadata.

---

#### 2.6. Extended Reasoning Events
**Codex Events:**
```rust
// Partially handled (AgentReasoning), but missing granular events:
AgentReasoningRawContent { text }       // Already mapped ✅
AgentReasoningRawContentDelta { delta } // Already mapped ✅
AgentReasoningSectionBreak              // Ignored (separator)

// Not yet mapped:
ThinkingBegin
ThinkingEnd
```

**Required ACP Mapping:**
```rust
// Current mapping is sufficient; Begin/End are metadata
ThinkingBegin → (no-op, or start buffering if needed)
ThinkingEnd → (no-op, or flush buffered thought)
```

**Gap Impact:** **NONE** - Current implementation handles reasoning content correctly.

---

### 🟡 Medium Priority Gaps (Nice-to-Have for MVP)

#### 2.7. Session Metadata Events
```rust
// Not yet mapped:
SessionMetadata {
    model: String,
    context_window: u32,
    settings: HashMap<String, Value>,
}

TaskStarted {
    model_context_window: Option<u32>,
}
```

**Impact:** Currently `SessionConfigured.model` is used for `CurrentModeUpdate`; additional metadata could be stored in `session.meta` field but not critical.

---

#### 2.8. Slash Command Execution
```rust
// Not yet mapped:
SlashCommandInvoked {
    command: String,
    args: Option<String>,
}

SlashCommandResult {
    command: String,
    output: String,
}
```

**Impact:** Slash commands are available via `AvailableCommandsUpdate` (already handled), but execution tracking is missing. Could map to `ToolCall` with `kind: Other`.

---

### 🟢 Low Priority (Post-MVP)

- Experimental JSON stream events
- Notify event pass-through (already handled via external notify source)
- Internal Codex debug events (not user-facing)

---

## 3. Missing ACP Features

### 🔴 Critical Gaps

#### 3.1. Submission Context Handling
**Required:** Capture and expose submission metadata to ACP clients.

**Current State:**
```rust
// codex_agent.rs:483-513 (build_codex_submission)
// Only handles prompt.messages (text content)
// Missing: cwd, sandbox, model, reasoning_mode, @-mentions
```

**What's Missing:**
```rust
struct SubmissionContext {
    cwd: PathBuf,                    // From session.working_dir ✅ (used in spawn)
    sandbox_mode: String,             // From permission_mode mapping ✅
    approval_policy: String,          // From permission_mode mapping ✅
    model: Option<String>,            // ❌ Not captured
    reasoning_enabled: bool,          // ❌ Not exposed
    file_attachments: Vec<String>,    // ❌ @-mentions not extracted
}
```

**Required Changes:**
1. Store `model` in session state (from `SessionConfigured` event or initial config)
2. Parse `@-mentions` from prompt text (regex `@[filename]` or `[@filename](uri)`)
3. Extract file attachments from `ContentBlock::Resource` and `ContentBlock::ResourceLink`
4. Expose in session metadata:
   ```rust
   SessionUpdate::CurrentModeUpdate {
       current_mode_id: format!("{}:{}", permission_mode, model),
       meta: Some(json!({
           "cwd": cwd,
           "sandbox": sandbox_mode,
           "approval": approval_policy,
           "model": model,
           "attachments": file_attachments,
       })),
   }
   ```

---

#### 3.2. Content Block Handling
**Required:** Support all ACP content types in prompts and responses.

**Current State:**
```rust
// codex_agent.rs:483-498 (build_codex_submission)
ContentBlock::Text(text) → items.push(json!({"type": "text", "text": text.text}))
// All other content types rejected with Error::invalid_params
```

**What's Missing:**
- ❌ `ContentBlock::Image` → Codex image submission
- ❌ `ContentBlock::Audio` → Codex audio submission (if supported)
- ❌ `ContentBlock::Resource` → Embed as context
- ❌ `ContentBlock::ResourceLink` → Convert to @-mention

**Required Mapping:**
```rust
match block {
    ContentBlock::Text(text) => {
        items.push(json!({ "type": "text", "text": text.text }));
    }

    ContentBlock::Image(img) => {
        items.push(json!({
            "type": "image",
            "source": {
                "type": if img.uri.is_some() { "url" } else { "base64" },
                "data": img.data,
                "media_type": img.mime_type,
                "url": img.uri,
            }
        }));
    }

    ContentBlock::Resource(res) => {
        // Extract URI and content
        let (uri, text) = match res.resource {
            TextResourceContents { uri, text, .. } => (uri, text),
            BlobResourceContents { uri, blob, .. } => {
                // Base64 decode if needed, or skip
                (uri, base64_decode(blob)?)
            }
        };

        // Add as @-mention + context
        items.push(json!({ "type": "text", "text": format_uri_as_link(&uri) }));
        items.push(json!({
            "type": "text",
            "text": format!("\n<context ref=\"{}\">\n{}\n</context>", uri, text)
        }));
    }

    ContentBlock::ResourceLink(link) => {
        items.push(json!({
            "type": "text",
            "text": format_uri_as_link(&link.uri)
        }));
    }

    ContentBlock::Audio(audio) => {
        // Codex may not support audio; return error or skip
        return Err(Error::invalid_params()
            .with_data("Audio content not supported by Codex CLI"));
    }
}
```

**Reference:** See claude-code-acp's `promptToClaude` ([analysis section 3](./claude-code-acp-analysis.md#3-content-block-handling)).

---

#### 3.3. Tool Call Diff Content
**Required:** For `Edit` tool calls, provide `ContentBlock::Diff` with oldText/newText.

**Current State:**
```rust
// codex_proto.rs:514-523 (send_tool_call)
// Only sends formatted output preview as text
content_blocks.push(ToolCallContent::from(formatted));
```

**What's Missing:**
- ❌ No diff generation for `PatchApply*` events
- ❌ No file content cache to compute oldText

**Required Pattern (from claude-code-acp):**
```rust
// 1. Cache file content on reads
let file_content_cache: Arc<RwLock<HashMap<String, String>>>;

// When processing UserMessage or file reads:
if let Some(read_result) = detect_file_read(&event) {
    file_content_cache.write().await.insert(read_result.path, read_result.content);
}

// 2. Generate diff for edits
PatchApplyBegin { changes } => {
    for (path, file_changes) in changes {
        let old_content = file_content_cache.read().await
            .get(&path)
            .cloned()
            .unwrap_or_default();

        let new_content = apply_changes(old_content.clone(), &file_changes)?;

        SessionUpdate::ToolCall {
            content: vec![ToolCallContent::Diff {
                path: PathBuf::from(path),
                old_text: Some(old_content),
                new_text: new_content,
            }],
            // ...
        }
    }
}
```

**Reference:** See claude-code-acp's `replaceAndCalculateLocation` ([analysis section 2](./claude-code-acp-analysis.md#pattern-2-edit-diff-calculation)).

---

### 🟡 Medium Priority Gaps

#### 3.4. Permission Request Integration
**Required:** Call `client.request_permission()` for approval flows.

**Current State:**
```rust
// Permission mode mapping exists (permissions.rs)
// But no active permission request flow
```

**What's Missing:**
```rust
// When receiving ExecApprovalRequest or PatchApprovalRequest:
ExecApprovalRequest { id, command, cwd, .. } => {
    let response = client.request_permission(RequestPermissionRequest {
        session_id: session_id.clone(),
        tool_call: Some(ToolCallReference {
            tool_call_id: ToolCallId(Arc::from(id.as_str())),
            raw_input: Some(json!({
                "command": command,
                "cwd": cwd,
            })),
        }),
        options: vec![
            PermissionRequestOption {
                kind: PermissionRequestOptionKind::AllowOnce,
                name: "Allow".into(),
                option_id: "allow".into(),
            },
            PermissionRequestOption {
                kind: PermissionRequestOptionKind::RejectOnce,
                name: "Reject".into(),
                option_id: "reject".into(),
            },
        ],
    }).await?;

    // Submit approval back to Codex
    let approved = matches!(
        response.outcome,
        Some(PermissionOutcome::Selected { option_id, .. }) if option_id == "allow"
    );

    write_line(process.stdin(), &json!({
        "id": uuid::Uuid::new_v4(),
        "op": {
            "type": "exec_approval",
            "approval_id": id,
            "approved": approved,
        }
    }).to_string()).await?;
}
```

**Impact:** Without this, all approval flows are auto-denied or ignored.

---

#### 3.5. Plan Entry Priority
**Required:** Map Codex plan items to ACP with priority.

**Current State:**
```rust
// codex_proto.rs:397-418 (send_plan_update)
let entries: Vec<PlanEntry> = update.plan.into_iter()
    .map(plan_entry_from_codex)  // Converts status only
    .collect();

fn plan_entry_from_codex(item: CodexPlanItem) -> PlanEntry {
    PlanEntry {
        content: item.step,
        priority: PlanEntryPriority::Medium,  // Hardcoded!
        status: match item.status { ... },
        meta: None,
    }
}
```

**What's Missing:**
- Codex doesn't provide priority in `CodexPlanItem`
- Need heuristic or default

**Options:**
1. Keep `Medium` default (simplest)
2. Infer from keywords ("URGENT", "CRITICAL" → High)
3. Add to Codex protocol (not feasible for MVP)

**Recommendation:** Keep current behavior; document as limitation.

---

### 🟢 Low Priority (Post-MVP)

- Session persistence (`session/load` support)
- Model switching via `session/setModel`
- Audio content support
- MCP HTTP/SSE transports (stdio only for MVP)

---

## 4. Mapping Tables

### 4.1. Codex Event → ACP SessionUpdate (Complete)

| Codex Event | ACP SessionUpdate | Status | Priority |
|-------------|-------------------|--------|----------|
| `AgentMessage` | `AgentMessageChunk` | ✅ Done | Critical |
| `AgentMessageDelta` | `AgentMessageChunk` | ✅ Done | Critical |
| `AgentReasoning` | `AgentThoughtChunk` | ✅ Done | High |
| `AgentReasoningDelta` | `AgentThoughtChunk` | ✅ Done | High |
| `AgentReasoningRawContent` | `AgentThoughtChunk` | ✅ Done | High |
| `AgentReasoningRawContentDelta` | `AgentThoughtChunk` | ✅ Done | High |
| `UserMessage` | `UserMessageChunk` | ✅ Partial (images done, files missing) | Medium |
| **`ExecCommandBegin`** | **`ToolCall`** | ❌ **Missing** | **Critical** |
| **`ExecCommandStdout`** | **`ToolCallUpdate`** | ❌ **Missing** | **Critical** |
| **`ExecCommandStderr`** | **`ToolCallUpdate`** | ❌ **Missing** | **Critical** |
| **`ExecCommandEnd`** | **`ToolCallUpdate`** | ❌ **Missing** | **Critical** |
| **`ExecApprovalRequest`** | **(permission flow)** | ❌ **Missing** | **Critical** |
| **`PatchApplyBegin`** | **`ToolCall` (with Diff)** | ❌ **Missing** | **Critical** |
| **`PatchApplyProgress`** | **`ToolCallUpdate`** | ❌ **Missing** | **High** |
| **`PatchApplyEnd`** | **`ToolCallUpdate`** | ❌ **Missing** | **Critical** |
| **`PatchApprovalRequest`** | **(permission flow)** | ❌ **Missing** | **Critical** |
| **`McpToolCallBegin`** | **`ToolCall`** | ❌ **Missing** | **Medium** |
| **`McpToolCallEnd`** | **`ToolCallUpdate`** | ❌ **Missing** | **Medium** |
| `McpListToolsResponse` | `AvailableCommandsUpdate` | ✅ Done | Medium |
| **`WebSearchBegin`** | **`ToolCall`** | ❌ **Missing** | **Low** |
| **`WebSearchEnd`** | **`ToolCallUpdate`** | ❌ **Missing** | **Low** |
| `ToolCall` | `ToolCall` | ✅ Done (generic fallback) | High |
| `ToolCalls` | `ToolCall` (batch) | ✅ Done | High |
| `PlanUpdate` | `Plan` | ✅ Done | High |
| `SessionConfigured` | `CurrentModeUpdate` | ✅ Done | Medium |
| `TaskStarted` | (metadata only) | ⚠️ Ignored | Low |
| `TaskComplete` | (finalize flag) | ✅ Done | Critical |
| `Error` | `ToolCallUpdate` (failed) | ✅ Done | High |

**Summary:**
- ✅ **Done:** 11 / 25 (44%)
- ❌ **Missing:** 14 / 25 (56%)
- **Critical Missing:** 8 events (ExecCommand*, PatchApply*, Approval flows)

---

### 4.2. Codex Tool Types → ACP ToolKind

| Codex Tool Pattern | ACP ToolKind | Detection Rule | Status |
|--------------------|--------------|----------------|--------|
| `ExecCommand*` | `Execute` | Event type | ❌ Missing |
| `PatchApply*` (Add/Update) | `Edit` | FileChange enum | ❌ Missing |
| `PatchApply*` (Delete) | `Delete` | FileChange enum | ❌ Missing |
| `McpToolCall*` | `Other` (or heuristic) | Event type + tool name | ❌ Missing |
| `WebSearch*` | `Fetch` | Event type | ❌ Missing |
| `PlanUpdate` | `Think` | Event type | ✅ Done |
| Generic `ToolCall` (shell) | `Execute` | name.contains("shell\|bash\|exec") | ✅ Done |
| Generic `ToolCall` (read) | `Read` | name.contains("read\|get\|view") | ✅ Done |
| Generic `ToolCall` (write/edit) | `Edit` | name.contains("write\|edit\|update") | ✅ Done |
| Generic `ToolCall` (delete) | `Delete` | name.contains("delete\|remove\|rm") | ✅ Done |
| Generic `ToolCall` (search) | `Search` | name.contains("search\|find\|grep") | ✅ Done |
| Generic `ToolCall` (fetch) | `Fetch` | name.contains("fetch\|download\|http") | ✅ Done |
| Generic `ToolCall` (think) | `Think` | name.contains("think\|reason\|plan") | ✅ Done |
| Generic `ToolCall` (move) | `Move` | name.contains("move\|rename\|mv") | ✅ Done |
| Generic `ToolCall` (switch) | `SwitchMode` | name.contains("switch_mode") | ✅ Done |
| Generic `ToolCall` (other) | `Other` | fallback | ✅ Done |

**Note:** Generic mappings work but miss rich metadata from explicit event types.

---

### 4.3. ACP Content Types → Codex Submission

| ACP ContentBlock | Codex Submission Format | Status | Priority |
|------------------|-------------------------|--------|----------|
| `Text` | `{"type": "text", "text": "..."}` | ✅ Done | Critical |
| **`Image`** | **`{"type": "image", "source": {...}}`** | ❌ **Missing** | **High** |
| `Audio` | (not supported by Codex?) | ⚠️ N/A | Low |
| **`Resource`** | **`@-mention + <context> tag`** | ❌ **Missing** | **High** |
| **`ResourceLink`** | **`@-mention text`** | ❌ **Missing** | **High** |

---

### 4.4. Permission Mode Mapping

| ACP PermissionMode | Codex CLI Flags | Status |
|--------------------|-----------------|--------|
| `ask_every_time` | `-c approval_policy=always` | ✅ Done |
| `ask_on_new_tools` | `-c approval_policy=per_session` | ✅ Done |
| `auto_approve` | `-c approval_policy=never` | ✅ Done |
| (custom modes) | (not applicable) | N/A |

**Additional Codex Flags:**
- `-c sandbox_mode=limited` / `full` / `none` (mapped from permission_mode in `permissions.rs`)
- `-c network_access=allow` / `deny` (via env `ACPLB_NETWORK_ACCESS`)

---

## 5. Implementation Roadmap

### Phase 1: Critical Event Mappings (Week 1)

**Goal:** Handle all execution and patch tool lifecycles.

#### Task 1.1: ExecCommand* Events
**Files:** `codex_proto.rs`, `tool_calls.rs`

**Changes:**
1. Add `ExecCommandBegin`, `ExecCommandStdout`, `ExecCommandStderr`, `ExecCommandEnd` to `CodexEvent` enum
2. Implement event handlers:
   ```rust
   CodexEvent::ExecCommandBegin { id, command, cwd, .. } => {
       self.send_exec_tool_call_begin(id, command, cwd, ...).await?;
   }

   CodexEvent::ExecCommandStdout { id, content } => {
       self.send_exec_output_update(id, content, "stdout").await?;
   }

   // Similar for Stderr, End
   ```
3. Track tool state (pending → in_progress → completed) in `tool_calls: HashMap<String, ToolCallRecord>`
4. Test with JSONL scenario: shell command execution with streaming output

**Acceptance:**
- ✅ Shell commands show as ToolCall with `kind: Execute`
- ✅ Stdout/stderr appear as incremental ToolCallUpdate chunks
- ✅ Exit code determines final status (0 → Completed, non-zero → Failed)
- ✅ Raw input/output preserved

---

#### Task 1.2: PatchApply* Events
**Files:** `codex_proto.rs`, `tool_calls.rs`

**Changes:**
1. Add `PatchApplyBegin`, `PatchApplyProgress`, `PatchApplyEnd` to `CodexEvent` enum
2. Add `FileChange` enum:
   ```rust
   #[derive(Deserialize, Serialize)]
   #[serde(tag = "type", rename_all = "snake_case")]
   pub enum FileChange {
       Add { content: String },
       Update { old_lines: Range<usize>, new_content: String },
       Delete,
   }
   ```
3. Implement file content cache:
   ```rust
   struct CodexStreamManager {
       file_content_cache: Arc<RwLock<HashMap<PathBuf, String>>>,
       // ...
   }
   ```
4. Generate diffs in `send_patch_tool_call`:
   ```rust
   let old_content = self.file_content_cache.read().await.get(path).cloned().unwrap_or_default();
   let new_content = apply_changes(&old_content, &file_changes)?;

   content: vec![ToolCallContent::Diff {
       path: path.clone(),
       old_text: Some(old_content),
       new_text: new_content,
   }]
   ```
5. Test with JSONL scenario: multi-file patch application

**Acceptance:**
- ✅ File edits show as ToolCall with `kind: Edit` or `Delete`
- ✅ Diff content includes full oldText and newText
- ✅ Multiple file changes tracked separately
- ✅ Progress updates show in_progress status

---

#### Task 1.3: Approval Flows
**Files:** `codex_proto.rs`, `codex_agent.rs`

**Changes:**
1. Add `ExecApprovalRequest`, `PatchApprovalRequest`, `ExecApproved`, `PatchApproved` to `CodexEvent`
2. Implement approval handler:
   ```rust
   CodexEvent::ExecApprovalRequest { id, command, cwd, .. } => {
       let response = self.request_permission_from_client(
           session_id,
           id.clone(),
           json!({ "command": command, "cwd": cwd }),
       ).await?;

       // Submit approval back to Codex
       write_line(process.stdin(), &json!({
           "id": Uuid::new_v4(),
           "op": {
               "type": "exec_approval",
               "approval_id": id,
               "approved": response.approved,
           }
       })).await?;
   }
   ```
3. Add `client: Arc<dyn Client>` to `CodexStreamManager` for permission requests
4. Test with JSONL scenario: shell command requiring approval

**Acceptance:**
- ✅ Approval requests trigger `client.request_permission()`
- ✅ User choice (allow/reject) sent back to Codex as approval op
- ✅ Rejected commands show as Failed ToolCallUpdate
- ✅ Approved commands proceed to execution

---

### Phase 2: MCP and Web Search (Week 2)

#### Task 2.1: McpToolCall* Events
**Files:** `codex_proto.rs`, `tool_calls.rs`

**Changes:**
1. Add `McpToolCallBegin`, `McpToolCallEnd` to `CodexEvent`
2. Implement heuristic tool kind mapping:
   ```rust
   fn infer_mcp_tool_kind(server: &str, tool: &str) -> ToolKind {
       let name = format!("{}:{}", server, tool).to_lowercase();
       map_tool_kind(&name)  // Reuse existing heuristic
   }
   ```
3. Test with JSONL scenario: MCP server tool invocation

**Acceptance:**
- ✅ MCP tools show as ToolCall with inferred kind
- ✅ Server and tool name in title
- ✅ Result/error mapped to ToolCallUpdate

---

#### Task 2.2: WebSearch* Events
**Files:** `codex_proto.rs`

**Changes:**
1. Add `WebSearchBegin`, `WebSearchEnd` to `CodexEvent`
2. Map to `ToolKind::Fetch`
3. Format search results as markdown list

**Acceptance:**
- ✅ Web searches show as ToolCall with `kind: Fetch`
- ✅ Results formatted as readable list

---

### Phase 3: Content and Submission Enhancements (Week 2-3)

#### Task 3.1: Image Support
**Files:** `codex_agent.rs` (`build_codex_submission`)

**Changes:**
1. Handle `ContentBlock::Image`:
   ```rust
   ContentBlock::Image(img) => {
       if let Some(uri) = img.uri {
           items.push(json!({
               "type": "image",
               "source": { "type": "url", "url": uri }
           }));
       } else {
           items.push(json!({
               "type": "image",
               "source": {
                   "type": "base64",
                   "data": img.data,
                   "media_type": img.mime_type
               }
           }));
       }
   }
   ```

**Acceptance:**
- ✅ Image prompts accepted (no error)
- ✅ Base64 and URL formats supported

---

#### Task 3.2: Resource Embedding
**Files:** `codex_agent.rs`

**Changes:**
1. Handle `ContentBlock::Resource` and `ContentBlock::ResourceLink`:
   ```rust
   ContentBlock::Resource(res) => {
       let (uri, text) = extract_resource_content(res)?;
       items.push(json!({ "type": "text", "text": format_uri_as_link(&uri) }));
       context_blocks.push(json!({
           "type": "text",
           "text": format!("<context ref=\"{}\">\n{}\n</context>", uri, text)
       }));
   }

   ContentBlock::ResourceLink(link) => {
       items.push(json!({ "type": "text", "text": format_uri_as_link(&link.uri) }));
   }
   ```
2. Append `context_blocks` after main `items`

**Acceptance:**
- ✅ @-mentions rendered as `[@name](uri)`
- ✅ Embedded resources wrapped in `<context>` tags

---

#### Task 3.3: Submission Metadata
**Files:** `codex_agent.rs`, `codex_proto.rs`

**Changes:**
1. Add `model: Option<String>` to session state
2. Capture model from `SessionConfigured` event
3. Emit metadata in `CurrentModeUpdate`:
   ```rust
   SessionUpdate::CurrentModeUpdate {
       current_mode_id: format!("{}:{}", permission_mode, model.unwrap_or("default")),
       meta: Some(json!({
           "cwd": session.working_dir,
           "sandbox": sandbox_mode,
           "approval": approval_policy,
           "model": model,
       })),
   }
   ```

**Acceptance:**
- ✅ Session metadata exposed to client
- ✅ Model name visible in mode ID or meta

---

### Phase 4: Testing and Evidence (Week 3)

#### Task 4.1: JSONL Test Scenarios
**Files:** `_artifacts/tests/protocol-baseline/`

**New Scenarios:**
1. `exec_command_streaming.jsonl` - Shell with stdout/stderr
2. `patch_apply_multi_file.jsonl` - Edit multiple files
3. `approval_flow_exec.jsonl` - Command requiring approval
4. `approval_flow_patch.jsonl` - Patch requiring approval
5. `mcp_tool_call.jsonl` - MCP server tool
6. `web_search.jsonl` - Web search query
7. `image_submission.jsonl` - Prompt with image

**Test Execution:**
```bash
for scenario in _artifacts/tests/protocol-baseline/*.jsonl; do
    echo "Testing $scenario..."
    cat "$scenario" | cargo run -p codex-cli-acp | jq '.' > "$(basename $scenario .jsonl)_output.jsonl"
    # Validate output schema
    jsonschema -i "$(basename $scenario .jsonl)_output.jsonl" schema.json
done
```

---

#### Task 4.2: Integration Tests
**Files:** `crates/codex-cli-acp/tests/e2e_test.rs`

**New Tests:**
```rust
#[tokio::test]
async fn test_exec_command_lifecycle() {
    // Spawn codex-cli-acp
    // Send session/new + session/prompt with shell command
    // Verify ToolCall (pending) → ToolCallUpdate (in_progress) → ToolCallUpdate (completed)
}

#[tokio::test]
async fn test_patch_approval_flow() {
    // Send session/prompt triggering patch
    // Verify PatchApprovalRequest → requestPermission RPC
    // Send approval response
    // Verify patch proceeds and completes
}

// Similar for MCP, web search, images
```

---

#### Task 4.3: Evidence Collection
**Files:** `_artifacts/040-codex-protocol-alignment-mvp/`

**Artifacts:**
1. `logs/` - Test run logs (stdout/stderr)
2. `jq/` - JQ validation results
3. `reports/` - Coverage reports (which events handled)
4. `schemas/` - JSON schema validation results

**Commands:**
```bash
# Test suite
./scripts/sdd/run-e2e-tests.sh 2>&1 | tee _artifacts/040-.../logs/e2e_$(date +%Y%m%d_%H%M%S).log

# Schema validation
for file in _artifacts/tests/protocol-baseline/*_output.jsonl; do
    jsonschema -i "$file" schema.json > "_artifacts/040-.../schemas/$(basename $file .jsonl)_validation.txt"
done

# Coverage report
./scripts/sdd/generate-event-coverage-report.sh > _artifacts/040-.../reports/coverage.md
```

---

## 6. Risk Mitigation

### Risk 1: Codex Event Schema Drift
**Probability:** Medium
**Impact:** High
**Mitigation:**
- Pin Codex CLI version in Docker image
- Add integration tests against known Codex version
- Document event schema assumptions in code comments
- Run tests with multiple Codex versions (if available)

---

### Risk 2: File Content Cache Memory Usage
**Probability:** Low
**Impact:** Medium
**Mitigation:**
- Implement LRU cache with size limit (e.g., 100 files)
- Clear cache on session end
- Add memory usage metrics
- Truncate large files (>1MB) before caching

---

### Risk 3: Permission Flow Blocking
**Probability:** Medium
**Impact:** High
**Mitigation:**
- Add timeout to `client.request_permission()` (default 5 minutes)
- Allow cancellation during permission wait
- Test with client that doesn't respond (should timeout gracefully)

---

### Risk 4: Diff Generation Errors
**Probability:** Medium
**Impact:** Medium
**Mitigation:**
- Wrap diff computation in try-catch; fallback to text-only on error
- Add unit tests for edge cases (empty file, binary file, large file)
- Log diff errors with file path and change details
- Use claude-code-acp's `replaceAndCalculateLocation` as reference

---

## 7. Success Criteria

### MVP Acceptance (All Must Pass)

1. **✅ Event Coverage**
   - [ ] 100% of critical events mapped (ExecCommand*, PatchApply*, Approval flows)
   - [ ] 80% of all Codex events mapped
   - [ ] All tool lifecycles tested (pending → in_progress → completed/failed)

2. **✅ ACP Compliance**
   - [ ] All JSONL scenarios pass schema validation
   - [ ] No protocol errors in Zed IDE integration tests
   - [ ] `protocolVersion: 1` (integer) in initialize response

3. **✅ Permission Flows**
   - [ ] Exec approval requests work end-to-end
   - [ ] Patch approval requests work with diff preview
   - [ ] Timeout and cancellation handled gracefully

4. **✅ Content Support**
   - [ ] Text, Image, Resource, ResourceLink supported in prompts
   - [ ] @-mentions parsed and embedded correctly
   - [ ] Diff content generated for patch tool calls

5. **✅ Evidence**
   - [ ] All test logs stored in `_artifacts/040-.../logs/`
   - [ ] Schema validation results in `_artifacts/040-.../schemas/`
   - [ ] Coverage report shows >95% event handling

6. **✅ Documentation**
   - [ ] Issue #50 spec/plan/tasks updated with findings
   - [ ] `dev-docs/references/codex.md` updated with event mappings
   - [ ] PR description includes before/after comparison

---

## 8. Post-MVP Enhancements

### Future Work (Not Required for 0.1.0)

1. **Slash Command Execution Tracking**
   - Map `SlashCommandInvoked`/`SlashCommandResult` to ToolCall with `kind: Other`
   - Show command output in content blocks

2. **Audio Content Support**
   - If Codex adds audio support, map to `ContentBlock::Audio`

3. **Session Persistence**
   - Implement `session/load` to resume previous sessions
   - Store session state in SQLite or JSON file

4. **Model Switching**
   - Implement `session/setModel` to change Codex model mid-session

5. **MCP HTTP/SSE Transports**
   - Add support for remote MCP servers (currently stdio only)

6. **Experimental JSON Mode**
   - Handle Codex experimental JSON output format (alternative to proto)

7. **Extended Reasoning Metadata**
   - Track `ThinkingBegin`/`ThinkingEnd` for reasoning session boundaries
   - Expose reasoning duration in metadata

---

## Appendix A: Reference Documents

### Research Artifacts
1. **Codex Protocol Analysis:** `_artifacts/reports/codex-protocol-analysis/`
   - Complete event mapping (50+ events)
   - Tool structures and parameters
   - Submission format specifications
   - 1800+ lines of comprehensive documentation

2. **ACP Protocol Mapping:** `_artifacts/reports/acp-protocol-complete-mapping.md`
   - Complete type enumerations
   - Protocol flows and constraints
   - Validation rules and best practices
   - 800+ lines of requirements

3. **Claude-Code-ACP Analysis:** (embedded in this document, section references)
   - Reference implementation patterns
   - Tool call lifecycle (two-phase)
   - Permission flow (three-tier)
   - Content transformation recipes

### Related Code Files
- **Current Implementation:**
  - `crates/codex-cli-acp/src/main.rs` - Entry point
  - `crates/codex-cli-acp/src/codex_proto.rs` - Event handling
  - `crates/codex-cli-acp/src/tool_calls.rs` - Tool utilities
  - `crates/codex-cli-acp/src/codex_agent.rs` - Runtime
  - `crates/acp-lazy-core/src/permissions.rs` - Permission mapping

- **Test Scenarios:**
  - `_artifacts/tests/protocol-baseline/*.jsonl`

---

## Appendix B: Event Mapping Quick Reference

### Critical Priority (Must Implement)
```
ExecCommandBegin → ToolCall (Execute, pending)
ExecCommandStdout → ToolCallUpdate (in_progress, stdout content)
ExecCommandStderr → ToolCallUpdate (in_progress, stderr content)
ExecCommandEnd → ToolCallUpdate (completed/failed, exit code)
ExecApprovalRequest → client.request_permission() → ExecApproved op
PatchApplyBegin → ToolCall (Edit/Delete, pending, with Diff)
PatchApplyProgress → ToolCallUpdate (in_progress)
PatchApplyEnd → ToolCallUpdate (completed/failed)
PatchApprovalRequest → client.request_permission() → PatchApproved op
```

### High Priority (Should Implement)
```
McpToolCallBegin → ToolCall (inferred kind, pending)
McpToolCallEnd → ToolCallUpdate (completed/failed, result)
ContentBlock::Image → Codex image submission
ContentBlock::Resource → @-mention + <context> tag
ContentBlock::ResourceLink → @-mention text
```

### Medium Priority (Nice-to-Have)
```
WebSearchBegin → ToolCall (Fetch, pending)
WebSearchEnd → ToolCallUpdate (completed, results)
UserFileAttachment → Resource content block
Submission metadata capture (model, cwd, attachments)
```

---

**End of Gap Analysis**

**Next Steps:**
1. Review this document with team
2. Prioritize tasks for sprint planning
3. Create GitHub issue #50 with summary
4. Generate spec/plan/tasks from this analysis
5. Begin Phase 1 implementation

**Questions/Feedback:** Add comments to this document or open discussion in #50.