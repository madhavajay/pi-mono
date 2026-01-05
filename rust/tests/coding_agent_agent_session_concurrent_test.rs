use pi::test_port::fail;

// Source: packages/coding-agent/test/agent-session-concurrent.test.ts

#[test]
fn should_throw_when_prompt_called_while_streaming() {
    fail(
        "packages/coding-agent/test/agent-session-concurrent.test.ts",
        "should throw when prompt() called while streaming",
    );
}

#[test]
fn should_allow_steer_while_streaming() {
    fail(
        "packages/coding-agent/test/agent-session-concurrent.test.ts",
        "should allow steer() while streaming",
    );
}

#[test]
fn should_allow_followup_while_streaming() {
    fail(
        "packages/coding-agent/test/agent-session-concurrent.test.ts",
        "should allow followUp() while streaming",
    );
}

#[test]
fn should_allow_prompt_after_previous_completes() {
    fail(
        "packages/coding-agent/test/agent-session-concurrent.test.ts",
        "should allow prompt() after previous completes",
    );
}
