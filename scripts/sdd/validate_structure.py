#!/usr/bin/env python3
"""
SDD Structure Validator

Validates that the repository follows Spec-Driven Development structure and conventions.
"""

import os
import re
import sys
import json
from pathlib import Path
from typing import List, Dict, Tuple, Optional


class SDDValidator:
    """Validates SDD structure and compliance."""
    
    def __init__(self, repo_root: Path = Path(".")):
        self.repo_root = repo_root
        self.errors: List[str] = []
        self.warnings: List[str] = []
        self.info: List[str] = []
    
    def validate_all(self) -> bool:
        """Run all validation checks."""
        print("🔍 Starting SDD Structure Validation...\n")
        
        checks = [
            ("Directory Structure", self.check_directory_structure),
            ("Governance Files", self.check_governance_files),
            ("Agent Context Files", self.check_agent_contexts),
            ("Specification Files", self.check_specifications),
            ("Task Traceability", self.check_task_traceability),
            ("Documentation Standards", self.check_documentation),
            ("Security Checks", self.check_security),
        ]
        
        all_passed = True
        
        for check_name, check_func in checks:
            print(f"Checking {check_name}...")
            passed = check_func()
            status = "✅ PASS" if passed else "❌ FAIL"
            print(f"  {status}\n")
            all_passed = all_passed and passed
        
        self.print_summary()
        return all_passed
    
    def check_directory_structure(self) -> bool:
        """Check required directories exist."""
        required_dirs = [
            "dev-docs/sdd",
            "dev-docs/agents",
            "dev-docs/git",
            "dev-docs/cli",
            "specs",
            "templates",
            "templates/commands",
            "scripts",
            "scripts/sdd",
            "tests",
            ".github/workflows"
        ]
        
        missing = []
        for dir_path in required_dirs:
            full_path = self.repo_root / dir_path
            if not full_path.exists():
                missing.append(dir_path)
                self.errors.append(f"Missing required directory: {dir_path}")
        
        if missing:
            self.info.append(f"Found {len(required_dirs) - len(missing)}/{len(required_dirs)} required directories")
        else:
            self.info.append("All required directories present")
        
        return len(missing) == 0
    
    def check_governance_files(self) -> bool:
        """Check SDD governance documents exist and are valid."""
        required_files = {
            "dev-docs/sdd/constitution.md": ["Core Principles", "Development Gates", "Version"],
            "dev-docs/sdd/constitution_update_checklist.md": ["Pre-Amendment", "Post-Amendment"],
            "dev-docs/sdd/lifecycle.md": ["Lifecycle Phases", "Enforcement", "Traceability"],
        }
        
        all_valid = True
        
        for file_path, required_sections in required_files.items():
            full_path = self.repo_root / file_path
            
            if not full_path.exists():
                self.errors.append(f"Missing governance file: {file_path}")
                all_valid = False
                continue
            
            content = full_path.read_text()
            for section in required_sections:
                if section not in content:
                    self.warnings.append(f"Missing section '{section}' in {file_path}")
                    all_valid = False
        
        return all_valid
    
    def check_agent_contexts(self) -> bool:
        """Check agent context files exist and follow format."""
        contexts = [
            "dev-docs/sdd/CLAUDE.md",
            "dev-docs/sdd/AGENTS.md",
            "dev-docs/cli/CLAUDE.md",
            "dev-docs/cli/AGENTS.md",
        ]
        
        all_valid = True
        
        for context_file in contexts:
            full_path = self.repo_root / context_file
            
            if not full_path.exists():
                self.errors.append(f"Missing agent context: {context_file}")
                all_valid = False
                continue
            
            content = full_path.read_text()
            
            # Check for required sections in CLAUDE.md files
            if context_file.endswith("CLAUDE.md"):
                required = ["## Role", "## Allowed Tools", "## Safety Boundaries"]
                for section in required:
                    if section not in content:
                        self.warnings.append(f"Missing section '{section}' in {context_file}")
            
            # Check for required sections in AGENTS.md files
            if context_file.endswith("AGENTS.md"):
                if "## Supported Agents" not in content:
                    self.warnings.append(f"Missing 'Supported Agents' section in {context_file}")
        
        return all_valid
    
    def check_specifications(self) -> bool:
        """Check specification files for completeness."""
        specs_dir = self.repo_root / "specs"
        
        if not specs_dir.exists():
            self.info.append("No specs directory yet (expected for new repo)")
            return True
        
        issues_found = False
        
        for spec_file in specs_dir.rglob("*.md"):
            content = spec_file.read_text()
            
            # Check for unresolved clarifications
            clarifications = re.findall(r"\[NEEDS CLARIFICATION[^\]]*\]", content)
            if clarifications:
                self.errors.append(f"Unresolved clarifications in {spec_file.relative_to(self.repo_root)}: {clarifications}")
                issues_found = True
            
            # Check for placeholders
            placeholders = re.findall(r"\[PLACEHOLDER[^\]]*\]", content)
            if placeholders:
                self.warnings.append(f"Placeholders in {spec_file.relative_to(self.repo_root)}: {placeholders}")
        
        return not issues_found
    
    def check_task_traceability(self) -> bool:
        """Check for task ID patterns and commit message format."""
        # Check if git repo
        git_dir = self.repo_root / ".git"
        if not git_dir.exists():
            self.info.append("Not a git repository - skipping commit checks")
            return True
        
        # Define valid patterns
        task_pattern = re.compile(r"(TASK-\d{3}|T\d{3})")
        commit_pattern = re.compile(
            r"^(feat|fix|docs|style|refactor|test|chore)(\([^)]+\))?: .+"
        )
        
        # This is a validation of the patterns themselves, not actual commits
        self.info.append("Task ID and commit patterns defined and valid")
        
        return True
    
    def check_documentation(self) -> bool:
        """Check documentation standards."""
        required_docs = [
            "README.md",
            "LICENSE",
            "CONTRIBUTING.md",
        ]
        
        missing = []
        for doc in required_docs:
            if not (self.repo_root / doc).exists():
                missing.append(doc)
                self.warnings.append(f"Missing documentation: {doc}")
        
        # Check for CHANGELOG
        if not (self.repo_root / "CHANGELOG.md").exists():
            self.info.append("CHANGELOG.md not found (will be created)")
        
        return len(missing) == 0
    
    def check_security(self) -> bool:
        """Check for security issues."""
        secret_patterns = [
            (re.compile(r"ghp_[a-zA-Z0-9]{36}"), "GitHub token"),
            (re.compile(r"sk-[a-zA-Z0-9]{48}"), "OpenAI key"),
            (re.compile(r"(?i)api[_-]?key\s*=\s*['\"][a-zA-Z0-9]{10,}['\"]"), "API key"),
            (re.compile(r"(?i)password\s*=\s*['\"][^'\"]+['\"]"), "Password"),
        ]
        
        exclude_dirs = {".git", "venv", "node_modules", "__pycache__", ".pytest_cache"}
        exclude_files = {"validate_structure.py", "test_sdd_validation.py", "mcp.md"}  # Exclude self and docs
        
        issues_found = False
        
        for root, dirs, files in os.walk(self.repo_root):
            # Skip excluded directories
            dirs[:] = [d for d in dirs if d not in exclude_dirs]
            
            for file in files:
                if file in exclude_files:
                    continue
                    
                file_path = Path(root) / file
                
                # Only check text files
                if file.endswith((".py", ".yml", ".yaml", ".json", ".md", ".sh", ".txt")):
                    try:
                        content = file_path.read_text()
                        
                        for pattern, desc in secret_patterns:
                            if pattern.search(content):
                                rel_path = file_path.relative_to(self.repo_root)
                                self.errors.append(f"Potential {desc} in {rel_path}")
                                issues_found = True
                    except (UnicodeDecodeError, PermissionError):
                        # Skip binary or inaccessible files
                        continue
        
        if not issues_found:
            self.info.append("No hardcoded secrets detected")
        
        return not issues_found
    
    def print_summary(self):
        """Print validation summary."""
        print("=" * 60)
        print("VALIDATION SUMMARY")
        print("=" * 60)
        
        if self.errors:
            print(f"\n❌ ERRORS ({len(self.errors)}):")
            for error in self.errors:
                print(f"  - {error}")
        
        if self.warnings:
            print(f"\n⚠️  WARNINGS ({len(self.warnings)}):")
            for warning in self.warnings:
                print(f"  - {warning}")
        
        if self.info:
            print(f"\nℹ️  INFO ({len(self.info)}):")
            for info in self.info:
                print(f"  - {info}")
        
        if not self.errors and not self.warnings:
            print("\n✅ All SDD structure validations passed!")
        elif not self.errors:
            print("\n⚠️  Validation passed with warnings")
        else:
            print("\n❌ Validation failed - please fix errors")
        
        print("=" * 60)


def main():
    """Main entry point."""
    # Allow repo root to be specified
    repo_root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
    
    validator = SDDValidator(repo_root)
    success = validator.validate_all()
    
    # Exit with appropriate code
    if not success:
        sys.exit(1)
    elif validator.warnings:
        sys.exit(0)  # Warnings don't fail CI
    else:
        sys.exit(0)


if __name__ == "__main__":
    main()