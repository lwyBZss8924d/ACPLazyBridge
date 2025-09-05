/**
 * @name Avoid unwrap/expect/panic in protocol handling code
 * @description Protocol and stdio code should return structured JSON-RPC errors instead of panicking
 * @kind problem
 * @problem.severity error
 * @id rust/acplazybridge/no-panics-in-protocol
 * @tags reliability
 *       security
 *       acplazybridge
 *       protocol
 * @precision high
 */

import rust

/**
 * Identifies files that handle protocol, JSON-RPC, or stdio operations
 */
class ProtocolFile extends File {
  ProtocolFile() {
    this.getRelativePath().regexpMatch(".*(protocol|jsonrpc|stdio|acp|transport|codex_proto)\\.rs$")
  }
}

// For now, we'll detect panic! macro calls in protocol files
from MacroCall mc, ProtocolFile f
where 
  f = mc.getLocation().getFile() and
  (mc.getPath().toString().matches("%panic%") or
   mc.getPath().toString().matches("%unwrap_failed%") or
   mc.getPath().toString().matches("%expect_failed%")) and
  not mc.getLocation().getFile().getRelativePath().matches("%test%")
select mc, "Avoid panic!/unwrap/expect in protocol code; return structured JSON-RPC errors instead"
