//! Permission mapping between ACP modes and Codex CLI parameters.
//!
//! This module handles the translation of ACP permission modes to Codex CLI
//! parameters to ensure non-interactive operation suitable for IDE integration.

use std::borrow::Cow;
use std::collections::HashMap;

/// ACP permission modes that control agent capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpPermissionMode {
    /// Default mode: read-only access, no network
    Default,
    /// Plan mode: same as default, agent is planning but not executing
    Plan,
    /// Accept edits: can modify files in workspace, no network
    AcceptEdits,
    /// Bypass permissions: workspace write with network access
    BypassPermissions,
    /// YOLO mode: full access (danger mode) - must be explicitly enabled
    Yolo,
}

impl AcpPermissionMode {
    /// Parse from string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Self::Default),
            "plan" => Some(Self::Plan),
            "acceptedits" | "accept-edits" | "accept_edits" => Some(Self::AcceptEdits),
            "bypasspermissions" | "bypass-permissions" | "bypass_permissions" => Some(Self::BypassPermissions),
            "yolo" | "danger" | "danger-full-access" => Some(Self::Yolo),
            _ => None,
        }
    }
}

/// Codex CLI override parameters for a turn.
#[derive(Debug, Clone)]
pub struct CodexTurnOverrides {
    /// Approval policy: "never" | "on-request" | "on-failure" | "untrusted"
    pub approval_policy: Cow<'static, str>,
    /// Sandbox mode: "read-only" | "workspace-write" | "danger-full-access"
    pub sandbox_mode: Cow<'static, str>,
    /// Whether network access is allowed
    pub network_access: bool,
    /// Additional CLI arguments to pass
    pub extra_args: Vec<String>,
}

impl Default for CodexTurnOverrides {
    fn default() -> Self {
        Self {
            approval_policy: Cow::Borrowed("never"),
            sandbox_mode: Cow::Borrowed("read-only"),
            network_access: false,
            extra_args: Vec::new(),
        }
    }
}

impl CodexTurnOverrides {
    /// Convert to Codex CLI arguments.
    pub fn to_cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "-c".to_string(),
            format!("approval_policy={}", self.approval_policy),
            "-c".to_string(),
            format!("sandbox_mode={}", self.sandbox_mode),
        ];

        // Add network access override if in workspace-write mode
        if self.sandbox_mode == "workspace-write" && self.network_access {
            args.push("-c".to_string());
            args.push("sandbox_workspace_write.network_access=true".to_string());
        }

        // Add any extra arguments
        args.extend(self.extra_args.clone());

        args
    }

    /// Create overrides for YOLO/danger mode.
    pub fn danger() -> Self {
        Self {
            approval_policy: Cow::Borrowed("never"),
            sandbox_mode: Cow::Borrowed("danger-full-access"),
            network_access: true,
            extra_args: vec!["--dangerously-bypass-approvals-and-sandbox".to_string()],
        }
    }
}

/// Map ACP permission modes to non-interactive Codex overrides.
///
/// This ensures that the Codex CLI never prompts for user approval,
/// which would block the IDE integration.
pub fn map_acp_to_codex(mode: AcpPermissionMode) -> CodexTurnOverrides {
    match mode {
        AcpPermissionMode::Default | AcpPermissionMode::Plan => {
            CodexTurnOverrides::default()
        }
        AcpPermissionMode::AcceptEdits => CodexTurnOverrides {
            approval_policy: Cow::Borrowed("never"),
            sandbox_mode: Cow::Borrowed("workspace-write"),
            network_access: false,
            extra_args: Vec::new(),
        },
        AcpPermissionMode::BypassPermissions => CodexTurnOverrides {
            approval_policy: Cow::Borrowed("never"),
            sandbox_mode: Cow::Borrowed("workspace-write"),
            network_access: true,
            extra_args: Vec::new(),
        },
        AcpPermissionMode::Yolo => CodexTurnOverrides::danger(),
    }
}

/// Environment-based permission override.
///
/// Allows overriding permissions via environment variables for testing.
/// 
/// # Environment Variables
/// 
/// The following environment variables are supported (using default prefix "ACPLB"):
/// - `ACPLB_APPROVAL_POLICY`: Override approval policy (never|on-request|on-failure|untrusted)
/// - `ACPLB_SANDBOX_MODE`: Override sandbox mode (read-only|workspace-write|danger-full-access)
/// - `ACPLB_NETWORK_ACCESS`: Override network access (true|false)
/// 
/// # Example
/// 
/// ```bash
/// export ACPLB_APPROVAL_POLICY=on-request
/// export ACPLB_SANDBOX_MODE=workspace-write
/// export ACPLB_NETWORK_ACCESS=true
/// ```
pub struct PermissionOverrides {
    env_prefix: String,
    cache: HashMap<String, String>,
}

impl Default for PermissionOverrides {
    fn default() -> Self {
        Self::new("ACPLB")
    }
}

impl PermissionOverrides {
    /// Create with a given environment variable prefix.
    /// 
    /// If you want the default "ACPLB" prefix, use `PermissionOverrides::default()`.
    pub fn new(env_prefix: impl Into<String>) -> Self {
        Self {
            env_prefix: env_prefix.into(),
            cache: HashMap::new(),
        }
    }

    /// Get an override value from environment.
    pub fn get(&mut self, key: &str) -> Option<&str> {
        let env_key = format!("{}_{}", self.env_prefix, key.to_uppercase());
        
        if !self.cache.contains_key(&env_key) {
            if let Ok(value) = std::env::var(&env_key) {
                self.cache.insert(env_key.clone(), value);
            }
        }
        
        self.cache.get(&env_key).map(String::as_str)
    }

    /// Apply environment overrides to Codex parameters.
    pub fn apply(&mut self, mut overrides: CodexTurnOverrides) -> CodexTurnOverrides {
        if let Some(policy) = self.get("approval_policy") {
            overrides.approval_policy = Cow::Owned(policy.to_string());
        }
        if let Some(sandbox) = self.get("sandbox_mode") {
            overrides.sandbox_mode = Cow::Owned(sandbox.to_string());
        }
        if let Some(network) = self.get("network_access") {
            overrides.network_access = network.parse().unwrap_or(false);
        }
        overrides
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_mode_parsing() {
        assert_eq!(AcpPermissionMode::from_str("default"), Some(AcpPermissionMode::Default));
        assert_eq!(AcpPermissionMode::from_str("Plan"), Some(AcpPermissionMode::Plan));
        assert_eq!(AcpPermissionMode::from_str("accept-edits"), Some(AcpPermissionMode::AcceptEdits));
        assert_eq!(AcpPermissionMode::from_str("bypass_permissions"), Some(AcpPermissionMode::BypassPermissions));
        assert_eq!(AcpPermissionMode::from_str("yolo"), Some(AcpPermissionMode::Yolo));
        assert_eq!(AcpPermissionMode::from_str("invalid"), None);
    }

    #[test]
    fn test_permission_mapping() {
        let default = map_acp_to_codex(AcpPermissionMode::Default);
        assert_eq!(default.approval_policy, "never");
        assert_eq!(default.sandbox_mode, "read-only");
        assert!(!default.network_access);

        let edit = map_acp_to_codex(AcpPermissionMode::AcceptEdits);
        assert_eq!(edit.approval_policy, "never");
        assert_eq!(edit.sandbox_mode, "workspace-write");
        assert!(!edit.network_access);

        let bypass = map_acp_to_codex(AcpPermissionMode::BypassPermissions);
        assert_eq!(bypass.approval_policy, "never");
        assert_eq!(bypass.sandbox_mode, "workspace-write");
        assert!(bypass.network_access);

        let yolo = map_acp_to_codex(AcpPermissionMode::Yolo);
        assert_eq!(yolo.sandbox_mode, "danger-full-access");
        assert!(yolo.network_access);
        assert!(!yolo.extra_args.is_empty());
    }

    #[test]
    fn test_cli_args_generation() {
        let overrides = CodexTurnOverrides {
            approval_policy: Cow::Borrowed("never"),
            sandbox_mode: Cow::Borrowed("workspace-write"),
            network_access: true,
            extra_args: vec![],
        };

        let args = overrides.to_cli_args();
        assert!(args.contains(&"-c".to_string()));
        assert!(args.contains(&"approval_policy=never".to_string()));
        assert!(args.contains(&"sandbox_mode=workspace-write".to_string()));
        assert!(args.contains(&"sandbox_workspace_write.network_access=true".to_string()));
    }

    #[test]
    fn test_env_overrides_no_leak() {
        // Test that environment overrides work without memory leaks
        std::env::set_var("TEST_APPROVAL_POLICY", "on-request");
        std::env::set_var("TEST_SANDBOX_MODE", "danger-full-access");
        std::env::set_var("TEST_NETWORK_ACCESS", "true");

        let mut overrides = PermissionOverrides::new("TEST");
        let base = CodexTurnOverrides::default();
        let modified = overrides.apply(base);

        // Check that overrides were applied correctly with owned strings
        assert_eq!(modified.approval_policy, "on-request");
        assert_eq!(modified.sandbox_mode, "danger-full-access");
        assert!(modified.network_access);

        // Verify we can use the strings in CLI args
        let args = modified.to_cli_args();
        assert!(args.contains(&"approval_policy=on-request".to_string()));
        assert!(args.contains(&"sandbox_mode=danger-full-access".to_string()));

        // Clean up env vars
        std::env::remove_var("TEST_APPROVAL_POLICY");
        std::env::remove_var("TEST_SANDBOX_MODE");
        std::env::remove_var("TEST_NETWORK_ACCESS");
    }

    #[test]
    fn test_default_env_prefix() {
        // Test that the default prefix is "ACPLB"
        std::env::set_var("ACPLB_APPROVAL_POLICY", "untrusted");
        std::env::set_var("ACPLB_SANDBOX_MODE", "read-only");
        
        let mut overrides = PermissionOverrides::default();
        let base = CodexTurnOverrides::default();
        let modified = overrides.apply(base);
        
        assert_eq!(modified.approval_policy, "untrusted");
        assert_eq!(modified.sandbox_mode, "read-only");
        
        // Clean up
        std::env::remove_var("ACPLB_APPROVAL_POLICY");
        std::env::remove_var("ACPLB_SANDBOX_MODE");
    }
}
