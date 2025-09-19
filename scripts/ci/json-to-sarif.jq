#!/usr/bin/env jq -f
# Convert ast-grep JSON output to SARIF format
# Usage: ast-grep scan --json | jq -f scripts/ci/json-to-sarif.jq > results.sarif

{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "ast-grep",
          "version": "0.x.x",
          "informationUri": "https://ast-grep.github.io/",
          "rules": (
            # Extract unique rules from all results
            [.[] | {
              id: .rule_id,
              name: .rule_id,
              shortDescription: {
                text: .message
              },
              fullDescription: {
                text: .message
              },
              help: {
                text: "ast-grep rule: \(.rule_id)",
                markdown: "**ast-grep rule**: `\(.rule_id)`"
              },
              properties: {
                tags: ["ast-grep", .severity // "warning"]
              }
            }] | unique_by(.id)
          )
        }
      },
      "results": [
        .[] | {
          ruleId: .rule_id,
          ruleIndex: 0,
          level: (
            if .severity == "error" then "error"
            elif .severity == "warning" then "warning"
            else "note"
            end
          ),
          message: {
            text: .message
          },
          locations: [
            {
              physicalLocation: {
                artifactLocation: {
                  uri: .file,
                  uriBaseId: "%SRCROOT%"
                },
                region: {
                  startLine: .start_line,
                  startColumn: .start_column,
                  endLine: .end_line,
                  endColumn: .end_column
                }
              }
            }
          ],
          partialFingerprints: {
            primaryLocationLineHash: "\(.file):\(.start_line):\(.rule_id)"
          }
        }
      ],
      "invocations": [
        {
          "executionSuccessful": true,
          "toolConfigurationNotifications": [],
          "toolExecutionNotifications": [],
          "exitCode": 0
        }
      ]
    }
  ]
}