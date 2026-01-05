use pi::test_port::fail;

// Source: packages/coding-agent/test/model-resolver.test.ts

#[test]
fn exact_match_returns_model_with_off_thinking_level() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "exact match returns model with off thinking level",
    );
}

#[test]
fn partial_match_returns_best_model() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "partial match returns best model",
    );
}

#[test]
fn no_match_returns_null_model() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "no match returns null model",
    );
}

#[test]
fn sonnet_high_returns_sonnet_with_high_thinking_level() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "sonnet:high returns sonnet with high thinking level",
    );
}

#[test]
fn gpt_4o_medium_returns_gpt_4o_with_medium_thinking_level() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "gpt-4o:medium returns gpt-4o with medium thinking level",
    );
}

#[test]
fn all_valid_thinking_levels_work() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "all valid thinking levels work",
    );
}

#[test]
fn sonnet_random_returns_sonnet_with_off_and_warning() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "sonnet:random returns sonnet with off and warning",
    );
}

#[test]
fn gpt_4o_invalid_returns_gpt_4o_with_off_and_warning() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "gpt-4o:invalid returns gpt-4o with off and warning",
    );
}

#[test]
fn qwen3_coder_exacto_matches_the_model_with_off() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "qwen3-coder:exacto matches the model with off",
    );
}

#[test]
fn openrouter_qwen_qwen3_coder_exacto_matches_with_provider_prefix() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "openrouter/qwen/qwen3-coder:exacto matches with provider prefix",
    );
}

#[test]
fn qwen3_coder_exacto_high_matches_model_with_high_thinking_level() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "qwen3-coder:exacto:high matches model with high thinking level",
    );
}

#[test]
fn openrouter_qwen_qwen3_coder_exacto_high_matches_with_provider_and_thinking_level() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "openrouter/qwen/qwen3-coder:exacto:high matches with provider and thinking level",
    );
}

#[test]
fn gpt_4o_extended_matches_the_extended_model() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "gpt-4o:extended matches the extended model",
    );
}

#[test]
fn qwen3_coder_exacto_random_returns_model_with_off_and_warning() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "qwen3-coder:exacto:random returns model with off and warning",
    );
}

#[test]
fn qwen3_coder_exacto_high_random_returns_model_with_off_and_warning() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "qwen3-coder:exacto:high:random returns model with off and warning",
    );
}

#[test]
fn empty_pattern_matches_via_partial_matching() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "empty pattern matches via partial matching",
    );
}

#[test]
fn pattern_ending_with_colon_treats_empty_suffix_as_invalid() {
    fail(
        "packages/coding-agent/test/model-resolver.test.ts",
        "pattern ending with colon treats empty suffix as invalid",
    );
}
