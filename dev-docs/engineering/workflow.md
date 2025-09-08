# Engineering Workflow: Using External References

This document outlines the team's process for using and maintaining the technical reference documentation stored in `dev-docs/references`. Following this process ensures that all engineers (both human and AI) are working with the same information and assumptions about our external dependencies.

## When to Consult the Reference Hub

The `dev-docs/references` directory is the single source of truth for information about third-party tools, protocols, and libraries that this project integrates with.

Engineers should consult the relevant documents in the reference hub during the following phases of the development lifecycle:

1.  **Issue Analysis & Planning:** Before starting work on a new feature or bug fix, check the reference docs for any components involved. This helps in understanding the capabilities, limitations, and correct integration patterns of the dependency.
2.  **Technical Design:** When writing a technical design document, link directly to the specific pages in our reference hub (e.g., `../references/acp.md`). This ensures that design decisions are based on the correct information.
3.  **Implementation:** During implementation, the reference docs serve as a quick guide to the official repositories and documentation, preventing engineers from having to re-discover this information.

## Keeping External Repositories Updated

While our reference docs link to the official sources, many engineers will find it useful to have local clones of these external repositories for deeper exploration.

It is the **individual engineer's responsibility** to keep their local clones up-to-date. A simple `git pull` in the repository's directory on a regular basis is recommended.

**Do not** commit external repository code directly to this project. The `dev-docs/references` hub is for documentation and links, not for vendoring code.

## Contributing to the Reference Hub

The world of software is constantly changing. A dependency might release a new version, or a documentation link might become outdated. All team members are encouraged to help keep our reference hub current.

**Process for Updating:**

1.  If you notice an outdated link, a missing piece of information, or a new relevant dependency, create a new branch.
2.  Update or create the relevant markdown file in `dev-docs/references/`.
3.  Ensure all new links are public, permanent URLs, not local file paths.
4.  Open a Pull Request with your changes. Briefly explain the reason for the update in the PR description (e.g., "Updating Gemini CLI docs to reflect new v2 API").

By following these simple guidelines, we can maintain a high-quality, reliable set of documentation that accelerates development and reduces confusion.
