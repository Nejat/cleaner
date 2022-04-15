#[test]
fn html_root_url() {
    version_sync::assert_html_root_url_updated!("src/main.rs");
}

#[test]
fn readme_usage_version() {
    version_sync::assert_contains_regex!("README.md", "cleaner {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-builds {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-builds-list {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-builds-remove {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-empties {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-empties-list {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-empties-remove {version}");
    version_sync::assert_contains_regex!("README.md", "cleaner-supported {version}");
}