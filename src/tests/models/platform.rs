use crate::models::Filter;
use crate::Platform;

#[test]
fn given_two_same_platforms_both_should_be_the_same() {
    let web = Platform {
        name: String::from("Web"),
        folders: vec![String::from("node_modules")],
        associated: vec![Filter::new(String::from("package.json"))],
    };

    let nodejs = Platform {
        name: String::from("NodeJS"),
        folders: vec![String::from("node_modules")],
        associated: vec![Filter::new(String::from("package.json"))],
    };

    let the_same = web.same_as(&nodejs);

    assert!(the_same);

    let the_same = nodejs.same_as(&web);

    assert!(the_same);
}

#[test]
fn given_two_different_platforms_neither_should_be_the_same() {
    let web = Platform {
        name: String::from("Web"),
        folders: vec![String::from("node_modules")],
        associated: vec![Filter::new(String::from("package.json"))],
    };

    let rust = Platform {
        name: String::from("Rust"),
        folders: vec![String::from("target")],
        associated: vec![Filter::new(String::from("cargo.toml"))],
    };

    let not_the_same = !web.same_as(&rust);

    assert!(not_the_same);

    let not_the_same = !rust.same_as(&web);

    assert!(not_the_same);
}