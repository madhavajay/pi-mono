use pi::test_port::fail;

// Source: packages/ai/test/context-overflow.test.ts

#[test]
fn claude_3_5_haiku_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "claude-3-5-haiku - should detect overflow via isContextOverflow",
    );
}

#[test]
fn claude_sonnet_4_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "claude-sonnet-4 - should detect overflow via isContextOverflow",
    );
}

#[test]
fn gpt_4o_mini_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "gpt-4o-mini - should detect overflow via isContextOverflow",
    );
}

#[test]
fn gpt_4o_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "gpt-4o - should detect overflow via isContextOverflow",
    );
}

#[test]
fn gemini_2_0_flash_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "gemini-2.0-flash - should detect overflow via isContextOverflow",
    );
}

#[test]
fn grok_3_fast_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "grok-3-fast - should detect overflow via isContextOverflow",
    );
}

#[test]
fn llama_3_3_70b_versatile_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "llama-3.3-70b-versatile - should detect overflow via isContextOverflow",
    );
}

#[test]
fn qwen_3_235b_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "qwen-3-235b - should detect overflow via isContextOverflow",
    );
}

#[test]
fn glm_4_5_flash_should_detect_overflow_via_iscontextoverflow_silent_overflow_or_rate_limit() {
    fail("packages/ai/test/context-overflow.test.ts", "glm-4.5-flash - should detect overflow via isContextOverflow (silent overflow or rate limit)");
}

#[test]
fn devstral_medium_latest_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "devstral-medium-latest - should detect overflow via isContextOverflow",
    );
}

#[test]
fn anthropic_claude_sonnet_4_via_openrouter_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "anthropic/claude-sonnet-4 via OpenRouter - should detect overflow via isContextOverflow",
    );
}

#[test]
fn deepseek_deepseek_v3_2_via_openrouter_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "deepseek/deepseek-v3.2 via OpenRouter - should detect overflow via isContextOverflow",
    );
}

#[test]
fn mistralai_mistral_large_2512_via_openrouter_should_detect_overflow_via_iscontextoverflow() {
    fail("packages/ai/test/context-overflow.test.ts", "mistralai/mistral-large-2512 via OpenRouter - should detect overflow via isContextOverflow");
}

#[test]
fn google_gemini_2_5_flash_via_openrouter_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "google/gemini-2.5-flash via OpenRouter - should detect overflow via isContextOverflow",
    );
}

#[test]
fn meta_llama_llama_4_maverick_via_openrouter_should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "meta-llama/llama-4-maverick via OpenRouter - should detect overflow via isContextOverflow",
    );
}

#[test]
fn gpt_oss_20b_should_detect_overflow_via_iscontextoverflow_ollama_silently_truncates() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "gpt-oss:20b - should detect overflow via isContextOverflow (ollama silently truncates)",
    );
}

#[test]
fn should_detect_overflow_via_iscontextoverflow() {
    fail(
        "packages/ai/test/context-overflow.test.ts",
        "should detect overflow via isContextOverflow",
    );
}
