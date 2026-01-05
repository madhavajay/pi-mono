use pi::test_port::fail;

// Source: packages/coding-agent/test/skills.test.ts

#[test]
fn should_load_a_valid_skill() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should load a valid skill",
    );
}

#[test]
fn should_warn_when_name_doesn_t_match_parent_directory() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn when name doesn't match parent directory",
    );
}

#[test]
fn should_warn_when_name_contains_invalid_characters() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn when name contains invalid characters",
    );
}

#[test]
fn should_warn_when_name_exceeds_64_characters() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn when name exceeds 64 characters",
    );
}

#[test]
fn should_warn_and_skip_skill_when_description_is_missing() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn and skip skill when description is missing",
    );
}

#[test]
fn should_warn_when_unknown_frontmatter_fields_are_present() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn when unknown frontmatter fields are present",
    );
}

#[test]
fn should_load_nested_skills_recursively() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should load nested skills recursively",
    );
}

#[test]
fn should_skip_files_without_frontmatter() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should skip files without frontmatter",
    );
}

#[test]
fn should_warn_when_name_contains_consecutive_hyphens() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should warn when name contains consecutive hyphens",
    );
}

#[test]
fn should_load_all_skills_from_fixture_directory() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should load all skills from fixture directory",
    );
}

#[test]
fn should_return_empty_for_non_existent_directory() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should return empty for non-existent directory",
    );
}

#[test]
fn should_use_parent_directory_name_when_name_not_in_frontmatter() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should use parent directory name when name not in frontmatter",
    );
}

#[test]
fn should_return_empty_string_for_no_skills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should return empty string for no skills",
    );
}

#[test]
fn should_format_skills_as_xml() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should format skills as XML",
    );
}

#[test]
fn should_include_intro_text_before_xml() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should include intro text before XML",
    );
}

#[test]
fn should_escape_xml_special_characters() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should escape XML special characters",
    );
}

#[test]
fn should_format_multiple_skills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should format multiple skills",
    );
}

#[test]
fn should_load_from_customdirectories_only_when_built_ins_disabled() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should load from customDirectories only when built-ins disabled",
    );
}

#[test]
fn should_filter_out_ignoredskills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should filter out ignoredSkills",
    );
}

#[test]
fn should_support_glob_patterns_in_ignoredskills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should support glob patterns in ignoredSkills",
    );
}

#[test]
fn should_have_ignoredskills_take_precedence_over_includeskills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should have ignoredSkills take precedence over includeSkills",
    );
}

#[test]
fn should_expand_in_customdirectories() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should expand ~ in customDirectories",
    );
}

#[test]
fn should_return_empty_when_all_sources_disabled_and_no_custom_dirs() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should return empty when all sources disabled and no custom dirs",
    );
}

#[test]
fn should_filter_skills_with_includeskills_glob_patterns() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should filter skills with includeSkills glob patterns",
    );
}

#[test]
fn should_support_glob_patterns_in_includeskills() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should support glob patterns in includeSkills",
    );
}

#[test]
fn should_return_all_skills_when_includeskills_is_empty() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should return all skills when includeSkills is empty",
    );
}

#[test]
fn should_detect_name_collisions_and_keep_first_skill() {
    fail(
        "packages/coding-agent/test/skills.test.ts",
        "should detect name collisions and keep first skill",
    );
}
