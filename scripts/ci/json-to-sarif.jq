#!/usr/bin/env jq -f
# Convert ast-grep JSON output to SARIF format
# Usage: ast-grep scan --json | jq -f scripts/ci/json-to-sarif.jq > results.sarif

# First, collect unique rules and create a mapping
. as $input |
($input | [.[] | .rule_id] | unique) as $rule_ids |

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
            # Create rules array with proper indices
            $rule_ids | to_entries | map({
              id: .value,
              name: .value,
              shortDescription: {
                text: "ast-grep rule: \(.value)"
              },
              fullDescription: {
                text: "ast-grep rule: \(.value)"
              },
              help: {
                text: "ast-grep rule: \(.value)",
                markdown: "**ast-grep rule**: `\(.value)`"
              },
              properties: {
                tags: ["ast-grep", "warning"]
              }
            })
          )
        }
      },
      "results": [
        $input[] | . as $item | {
          ruleId: .rule_id,
          ruleIndex: (
            # Find the index of this rule_id in the $rule_ids array
            $rule_ids | to_entries | map(select(.value == $item.rule_id)) | .[0].key
          ),
          level: (
            if .severity == "error" then "error"
            elif .severity == "warning" then "warning"
            else "note"
            end
          ),
          message: {
            text: (.message // "ast-grep violation")
          },
          locations: [
            {
              physicalLocation: {
                artifactLocation: {
                  uri: .file,
                  uriBaseId: "%SRCROOT%"
                },
                region: {
                  startLine: (.start_line // .line // 1),
                  startColumn: (.start_column // .column // 1),
                  endLine: (.end_line // .start_line // .line // 1),
                  endColumn: (.end_column // .start_column // .column // 1)
                }
              }
            }
          ],
          partialFingerprints: {
            primaryLocationLineHash: "\(.file):\(.start_line // .line // 1):\(.rule_id)"
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