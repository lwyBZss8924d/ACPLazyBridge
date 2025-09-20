#!/usr/bin/env jq -f
# Convert ast-grep JSON output to SARIF format
# Usage: ast-grep scan --json | jq -f scripts/ci/json-to-sarif.jq > results.sarif

# Process the input array to generate SARIF
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
            # Extract unique rule IDs and create rules (ensure strings)
            [.[] | (.ruleId // "unknown") | tostring]
            | unique
            | map({
              id: .,
              name: .,
              shortDescription: {
                text: "ast-grep rule: \(.)"
              },
              fullDescription: {
                text: "ast-grep rule: \(.)"
              },
              help: {
                text: "Check for \(.)",
                markdown: "**ast-grep rule**: `\(.)`"
              },
              properties: {
                tags: ["ast-grep", "code-quality"]
              }
            })
          )
        }
      },
      "results": [
        # Store the rules array for indexing (ensure strings)
        ([.[] | (.ruleId // "unknown") | tostring] | unique) as $rules |

        # Convert each result
        .[]
        | ((.ruleId // "unknown") | tostring) as $rid
        | {
          ruleId: $rid,
          # Only include ruleIndex if found, otherwise omit the field
          ruleIndex: ($rules | index($rid) // empty),
          level: (
            if .severity == "error" then "error"
            elif .severity == "warning" then "warning"
            else "note"
            end
          ),
          message: {
            text: (.message // "ast-grep rule violation")
          },
          locations: [
            {
              physicalLocation: {
                artifactLocation: {
                  uri: .file,
                  uriBaseId: "%SRCROOT%"
                },
                region: (
                  if .range then {
                    startLine: .range.start.line,
                    # Ensure columns are >= 1 (SARIF requirement)
                    startColumn: (if (.range.start.column // 0) < 1 then 1 else .range.start.column end),
                    endLine: .range.end.line,
                    endColumn: (if (.range.end.column // 0) < 1 then 1 else .range.end.column end)
                  } else {
                    startLine: 1,
                    startColumn: 1
                  } end
                )
              }
            }
          ],
          partialFingerprints: {
            # Include column for better uniqueness
            primaryLocationLineHash: "\(.file):\(.range.start.line // 1):\(.range.start.column // 1):\($rid)"
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