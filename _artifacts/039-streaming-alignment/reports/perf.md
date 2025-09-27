    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/codex_cli_acp-6dc660b66be2e421)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running unittests src/bin/acplb_notify_forwarder.rs (target/debug/deps/acplb_notify_forwarder-397e8d472f81b840)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codex_cli_acp-03038df5c727d4a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/playback.rs (target/debug/deps/playback-04eaff1f515a53dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/acp_integration_test.rs (target/debug/deps/acp_integration_test-93e565b2615e3228)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/codex_cli_smoke_test.rs (target/debug/deps/codex_cli_smoke_test-1227f91c71d51ca3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/jsonl_regression_test.rs (target/debug/deps/jsonl_regression_test-c5ec5116dcfb6e27)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/notify_test.rs (target/debug/deps/notify_test-8285e7fdbbc64c09)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/playback.rs (target/debug/deps/playback-cbb779074c84c38f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/session_update_format.rs (target/debug/deps/session_update_format-45014093fba68c34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/streaming_snapshots_test.rs (target/debug/deps/streaming_snapshots_test-6cd0b97aeeb80e16)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tool_call_lifecycle_test.rs (target/debug/deps/tool_call_lifecycle_test-66680d25d729e449)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tool_calls_test.rs (target/debug/deps/tool_calls_test-d250b30dba1277dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

        0.17 real         0.09 user         0.05 sys
            44777472  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               14239  page reclaims
                   4  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   1  signals received
                 175  voluntary context switches
                 163  involuntary context switches
           928750414  instructions retired
           292512715  cycles elapsed
            31097856  peak memory footprint
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/codex_cli_acp-6dc660b66be2e421)

running 7 tests
test tool_calls::tests::test_extract_shell_command ... ok
test tool_calls::tests::test_extract_shell_params ... ok
test tool_calls::tests::test_format_tool_output ... ok
test tool_calls::tests::test_map_tool_kind ... ok
test tool_calls::tests::test_truncate_output ... ok
test validation::tests::test_validate_absolute_path ... ok
test validation::tests::test_validate_line_number ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/acplb_notify_forwarder.rs (target/debug/deps/acplb_notify_forwarder-397e8d472f81b840)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codex_cli_acp-03038df5c727d4a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/playback.rs (target/debug/deps/playback-04eaff1f515a53dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/acp_integration_test.rs (target/debug/deps/acp_integration_test-93e565b2615e3228)

running 3 tests
test idle_timeout_without_notify_returns_end_turn ... ok
test notify_signal_causes_early_stop_reason ... ok
test session_lifecycle_returns_end_turn ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/codex_cli_smoke_test.rs (target/debug/deps/codex_cli_smoke_test-1227f91c71d51ca3)

running 1 test
test codex_cli_exec_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/jsonl_regression_test.rs (target/debug/deps/jsonl_regression_test-c5ec5116dcfb6e27)

running 2 tests
test jsonl_regression_playback_remains_compatible ... ok
test jsonl_regression_requires_official_schema ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/notify_test.rs (target/debug/deps/notify_test-8285e7fdbbc64c09)

running 4 tests
test notify_source_tests::test_file_notify_source_reads_new_lines ... ok
test test_notify_forwarder_appends_to_existing_file ... ok
test test_notify_forwarder_fails_without_env ... ok
test test_notify_forwarder_writes_to_file ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/playback.rs (target/debug/deps/playback-cbb779074c84c38f)

running 5 tests
test test_basic_session ... ok
test test_cancel_notification ... ok
test test_handshake ... ok
test test_invalid_params ... ok
test test_unknown_method ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.67s

     Running tests/session_update_format.rs (target/debug/deps/session_update_format-45014093fba68c34)

running 6 tests
test test_agent_message_chunk_format ... ok
test test_initialize_response_spec_compliance ... ok
test test_serialization_format ... ok
test test_tool_call_content_structure ... ok
test test_tool_call_format ... ok
test test_tool_call_update_structure ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/streaming_snapshots_test.rs (target/debug/deps/streaming_snapshots_test-6cd0b97aeeb80e16)

running 5 tests
test agent_message_json_content_blocks ... ok
test deduplicated_agent_chunks_should_parse_with_official_schema ... ok
test harness_emits_updates_for_agent_messages ... ok
test rich_session_updates_cover_user_thought_plan_and_mode ... ok
test session_update_variants_deserialize_into_official_schema ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/tool_call_lifecycle_test.rs (target/debug/deps/tool_call_lifecycle_test-66680d25d729e449)

running 6 tests
test harness_emits_batched_tool_calls ... ok
test harness_emits_tool_call_updates ... ok
test idle_timeout_should_emit_official_stop_reason ... ok
test notify_completion_should_emit_official_stop_reason ... ok
test tool_call_error_maps_to_failed_status_update ... ok
test tool_call_status_transitions_use_official_update_fields ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/tool_calls_test.rs (target/debug/deps/tool_calls_test-d250b30dba1277dd)

running 6 tests
test test_batch_tool_calls ... ok
test test_output_truncation ... ok
test test_shell_command_extraction ... ok
test test_shell_params_extraction ... ok
test test_single_tool_call_progression ... ok
test test_tool_kind_mapping_comprehensive ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

        3.18 real         0.14 user         0.10 sys
            45039616  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               12519  page reclaims
                 955  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   1  signals received
                  60  voluntary context switches
                1030  involuntary context switches
           916295397  instructions retired
           281627793  cycles elapsed
            31360000  peak memory footprint
