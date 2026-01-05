use pi::test_port::fail;

// Source: packages/coding-agent/test/interactive-mode-status.test.ts

#[test]
fn coalesces_immediately_sequential_status_messages() {
    fail(
        "packages/coding-agent/test/interactive-mode-status.test.ts",
        "coalesces immediately-sequential status messages",
    );
}

#[test]
fn appends_a_new_status_line_if_something_else_was_added_in_between() {
    fail(
        "packages/coding-agent/test/interactive-mode-status.test.ts",
        "appends a new status line if something else was added in between",
    );
}
