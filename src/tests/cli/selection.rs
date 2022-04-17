use std::str::FromStr;

use crate::Selection;

#[test]
fn given_a_str_all_then_it_should_be_all() {
    let expected = Selection::All;
    let actual = Selection::from_str("all ").unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn given_a_str_of_select_comma_delimited_values_it_should_be_cleansed_select_values() {
    let expected = Selection::Select {
        values: vec![
            String::from('a'), String::from('b'),
            String::from('c'), String::from('d'),
        ]
    };

    let actual = Selection::from_str("a, b,c ,, d ").unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn given_a_str_of_select_comma_delimited_values_it_should_be_select_values() {
    let expected = Selection::Select {
        values: vec![
            String::from('a'), String::from('b'),
            String::from('c'), String::from('d'),
        ]
    };

    let actual = Selection::from_str("a,b,c,d").unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn given_all_checking_a_value_should_match() {
    let expected = true;
    let actual = Selection::All.matches("value");

    assert_eq!(expected, actual);
}

#[test]
fn given_all_it_should_display_all() {
    let actual = format!("{}", Selection::All);
    let expected = "all";

    assert_eq!(expected, actual);
}

#[test]
fn given_all_it_should_pluralize() {
    let actual = Selection::All.pluralize("s");
    let expected = "s";

    assert_eq!(expected, actual);
}

#[test]
fn given_all_when_choosing_it_should_choose_all_value() {
    let expected = "all";
    let actual = Selection::All.choose("select", "all");

    assert_eq!(expected, actual);
}

#[test]
fn given_no_selected_it_should_pluralize() {
    let sut = Selection::Select { values: vec![] };
    let actual = sut.pluralize("s");
    let expected = "";

    assert_eq!(expected, actual);
}

#[test]
fn given_one_selected_it_should_not_pluralize() {
    let sut = Selection::Select {
        values: vec![
            String::from("one")
        ]
    };
    let actual = sut.pluralize("s");
    let expected = "";

    assert_eq!(expected, actual);
}

#[test]
fn given_select_when_choosing_it_should_choose_select_value() {
    let expected = "select";
    let sut = Selection::Select { values: vec![] };
    let actual = sut.choose("select", "all");

    assert_eq!(expected, actual);
}

#[test]
fn given_selected_checking_a_value_should_match_if_exists() {
    let expected = true;
    let sut = Selection::Select { values: vec![String::from("value")] };
    let actual = sut.matches("value");

    assert_eq!(expected, actual);
}

#[test]
fn given_selected_checking_a_value_should_not_match_if_does_not_exists() {
    let expected = false;
    let sut = Selection::Select { values: vec![String::from("value")] };
    let actual = sut.matches("other value");

    assert_eq!(expected, actual);
}

#[test]
fn given_selected_it_should_display_selected() {
    let sut = Selection::Select {
        values: vec![
            String::from("one"), String::from("two"),
        ]
    };

    let actual = format!("{sut}");
    let expected = "one & two";

    assert_eq!(expected, actual);
}

#[test]
fn given_selected_it_should_pluralize() {
    let sut = Selection::Select {
        values: vec![
            String::from("one"), String::from("two"),
        ]
    };

    let actual = sut.pluralize("s");
    let expected = "s";

    assert_eq!(expected, actual);
}
