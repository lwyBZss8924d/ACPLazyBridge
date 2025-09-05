/**
 * @name Avoid logging potential secrets
 * @description Variables with secret-like names should not be logged
 * @kind problem
 * @problem.severity error
 * @id rust/acplazybridge/no-secret-logging
 * @tags security
 *       acplazybridge
 *       secrets
 * @precision medium
 */

import rust

// Simplified: Look for logging macros that might output sensitive data
// Focus on detecting environment variable access in logging contexts
from MacroCall mc
where 
  // Logging macros
  (mc.getPath().toString().regexpMatch(".*(eprintln|println|print|info|debug|warn|error|trace).*") or
   mc.getPath().toString().matches("%log%")) and
  // In files that handle sensitive operations
  (mc.getLocation().getFile().getRelativePath().matches("%auth%") or
   mc.getLocation().getFile().getRelativePath().matches("%codex_proto%") or
   mc.getLocation().getFile().getRelativePath().matches("%transport%"))
select mc, "Review logging statement for potential sensitive data exposure"
