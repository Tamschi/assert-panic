use version_sync::*;

#[test]
fn readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn cargo_versions() {
    assert_contains_regex!("Cargo.toml", "^documentation = \"https://docs\\.rs/assert-panic/{version}/assert-panic/macro\\.assert_panic\\.html\"$");
}

#[test]
fn readme_versions() {
    assert_contains_regex!("README.md", "\\[!\\[docs.rs]\\(https://docs.rs/assert-panic/badge.svg\\?version={version}\\)\\]\\(https://docs.rs/assert-panic/{version}/assert_panic/macro.assert_panic.html\\)");
}

#[test]
fn changelog_contains_version() {
    assert_contains_regex!("CHANGELOG.md", "`{version}`");
}
