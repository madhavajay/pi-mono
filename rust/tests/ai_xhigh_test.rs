use pi::test_port::fail;

// Source: packages/ai/test/xhigh.test.ts

#[test]
fn should_work_with_openai_responses() {
    fail(
        "packages/ai/test/xhigh.test.ts",
        "should work with openai-responses",
    );
}

#[test]
fn should_error_with_openai_responses_when_using_xhigh() {
    fail(
        "packages/ai/test/xhigh.test.ts",
        "should error with openai-responses when using xhigh",
    );
}

#[test]
fn should_error_with_openai_completions_when_using_xhigh() {
    fail(
        "packages/ai/test/xhigh.test.ts",
        "should error with openai-completions when using xhigh",
    );
}
