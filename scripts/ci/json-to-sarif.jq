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
            # Extract unique rule IDs and create rules
            [.[] | .ruleId // "unknown"]
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
        # Store the rules array for indexing
        ([.[] | .ruleId // "unknown"] | unique) as $rules |

        # Convert each result
        .[] | {
          ruleId: (.ruleId // "unknown"),
          ruleIndex: (
            # Find index of this rule in the rules array
            .ruleId as $rid |
            $rules | map(. == $rid) | index(true)
          ),
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
                    startColumn: .range.start.column,
                    endLine: .range.end.line,
                    endColumn: .range.end.column
                  } else {
                    startLine: 1,
                    startColumn: 1
                  } end
                )
              }
            }
          ],
          partialFingerprints: {
            primaryLocationLineHash: "\(.file):\(.range.start.line // 1):\(.ruleId // "unknown")"
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