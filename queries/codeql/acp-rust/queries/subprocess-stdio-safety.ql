/**
 * @name Subprocess stdio must be explicitly configured
 * @description Spawned processes should use Stdio::piped() to enforce strict I/O separation per WARP logging rules
 * @kind problem
 * @problem.severity warning
 * @id rust/acplazybridge/subprocess-stdio-safety
 * @tags correctness
 *       security
 *       acplazybridge
 *       subprocess
 * @precision medium
 */

import rust

// Simplified: Look for Command::new calls without proper stdio configuration
// This is a basic check for now
from MacroCall mc, File f
where 
  // Look for Command-related code patterns
  (mc.getPath().toString().matches("%Command%") or
   mc.getPath().toString().matches("%spawn%")) and
  f = mc.getLocation().getFile() and
  f.getRelativePath().matches("%codex_proto%")
select mc, "Ensure subprocess spawn() explicitly configures stdin/stdout/stderr with Stdio::piped() for I/O separation"
