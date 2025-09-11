# ACPLazyBridge Project Overview

## Purpose
ACPLazyBridge is an IDE-agnostic Agent Client Protocol (ACP) bridge that provides unified adapter implementations for various AI agents including Claude, Gemini, and Codex.

## Tech Stack
- **Language**: Rust (stable toolchain)
- **Protocol**: ACP (Agent Client Protocol) over stdio
- **Format**: JSON-RPC 2.0 with line-separated JSON (JSONL)
- **Build System**: Cargo with workspace configuration

## Key Features
- Streaming support for real-time agent responses
- Tool call handling with permission mapping
- Non-interactive permission modes to avoid UI approval prompts
- Extensible plugin system (planned)
- Multi-agent support (Codex native, Claude/Gemini proxy planned)

## Project Status
- M0: Completed - Basic workspace setup
- M1: In Progress - Codex Native Adapter implementation
- M2-M5: Planned - Proxy adapters, plugin system, native adapters, HTTP/SSE bridge

## Repository Structure
- Follows ACP specifications strictly
- Check `local_refs/` for reference implementations
- Chinese documentation in `dev-docs/` for detailed guidance
- Evidence-based development with artifacts in `dev-docs/review/_artifacts/`