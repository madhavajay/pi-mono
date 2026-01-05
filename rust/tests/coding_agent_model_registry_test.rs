use pi::test_port::fail;

// Source: packages/coding-agent/test/model-registry.test.ts

#[test]
fn overriding_baseurl_keeps_all_built_in_models() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "overriding baseUrl keeps all built-in models",
    );
}

#[test]
fn overriding_baseurl_changes_url_on_all_built_in_models() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "overriding baseUrl changes URL on all built-in models",
    );
}

#[test]
fn overriding_headers_merges_with_model_headers() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "overriding headers merges with model headers",
    );
}

#[test]
fn baseurl_only_override_does_not_affect_other_providers() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "baseUrl-only override does not affect other providers",
    );
}

#[test]
fn can_mix_baseurl_override_and_full_replacement() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "can mix baseUrl override and full replacement",
    );
}

#[test]
fn refresh_picks_up_baseurl_override_changes() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "refresh() picks up baseUrl override changes",
    );
}

#[test]
fn custom_provider_with_same_name_as_built_in_replaces_built_in_models() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "custom provider with same name as built-in replaces built-in models",
    );
}

#[test]
fn custom_provider_with_same_name_as_built_in_does_not_affect_other_built_in_providers() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "custom provider with same name as built-in does not affect other built-in providers",
    );
}

#[test]
fn multiple_built_in_providers_can_be_overridden() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "multiple built-in providers can be overridden",
    );
}

#[test]
fn refresh_reloads_overrides_from_disk() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "refresh() reloads overrides from disk",
    );
}

#[test]
fn removing_override_from_models_json_restores_built_in_provider() {
    fail(
        "packages/coding-agent/test/model-registry.test.ts",
        "removing override from models.json restores built-in provider",
    );
}
