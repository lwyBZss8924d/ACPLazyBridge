/**
 * @name Avoid println!/print! in production code
 * @description println!/print! macros write to stdout which is reserved for JSON-RPC. Use eprintln! or structured logging to stderr instead.
 * @kind problem
 * @problem.severity warning
 * @id rust/acplazybridge/no-stdout-logging
 * @tags correctness
 *       acplazybridge
 *       logging
 * @precision high
 */

import rust

from MacroCall mc
where 
  mc.getPath().toString().regexpMatch(".*(println|print)$") and
  not mc.getLocation().getFile().getRelativePath().matches("%test%")
select mc, "Avoid println!/print! macro; use eprintln! or structured logging (tracing) for stderr output"
