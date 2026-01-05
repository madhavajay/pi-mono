use pi::test_port::fail;

// Source: packages/coding-agent/test/fuzzy.test.ts

#[test]
fn empty_query_matches_everything_with_score_0() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "empty query matches everything with score 0",
    );
}

#[test]
fn query_longer_than_text_does_not_match() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "query longer than text does not match",
    );
}

#[test]
fn exact_match_has_good_score() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "exact match has good score",
    );
}

#[test]
fn characters_must_appear_in_order() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "characters must appear in order",
    );
}

#[test]
fn case_insensitive_matching() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "case insensitive matching",
    );
}

#[test]
fn consecutive_matches_score_better_than_scattered_matches() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "consecutive matches score better than scattered matches",
    );
}

#[test]
fn word_boundary_matches_score_better() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "word boundary matches score better",
    );
}

#[test]
fn empty_query_returns_all_items_unchanged() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "empty query returns all items unchanged",
    );
}

#[test]
fn filters_out_non_matching_items() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "filters out non-matching items",
    );
}

#[test]
fn sorts_results_by_match_quality() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "sorts results by match quality",
    );
}

#[test]
fn works_with_custom_gettext_function() {
    fail(
        "packages/coding-agent/test/fuzzy.test.ts",
        "works with custom getText function",
    );
}
