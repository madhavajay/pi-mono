use pi::test_port::fail;

// Source: packages/coding-agent/test/compaction-hooks-example.test.ts

#[test]
fn custom_compaction_example_should_type_check_correctly() {
    fail(
        "packages/coding-agent/test/compaction-hooks-example.test.ts",
        "custom compaction example should type-check correctly",
    );
}

#[test]
fn compact_event_should_have_correct_fields() {
    fail(
        "packages/coding-agent/test/compaction-hooks-example.test.ts",
        "compact event should have correct fields",
    );
}
