use crate::utils::list_output;

#[test]
fn given_a_list_with_multiple_items_list_output_should_produce_all_items_as_output_with_comma_and_ampersand_separators() {
    const EXPECTED: &str = "One, Two, Three, Four & Five";

    let actual = list_output(&["One", "Two", "Three", "Four", "Five"]);

    assert_eq!(EXPECTED, actual);
}

#[test]
fn given_a_list_with_one_item_list_output_should_produce_only_that_item_as_output() {
    const EXPECTED: &str = "Only One Item";

    let actual = list_output(&["Only One Item"]);

    assert_eq!(EXPECTED, actual);
}

#[test]
fn given_a_list_with_three_items_list_output_should_produce_all_items_as_output_with_comma_and_ampersand_separators() {
    const EXPECTED: &str = "One, Two & Three";

    let actual = list_output(&["One", "Two", "Three"]);

    assert_eq!(EXPECTED, actual);
}

#[test]
fn given_a_list_with_two_items_list_output_should_produce_both_items_as_output_with_an_ampersand_separator() {
    const EXPECTED: &str = "One & Two";

    let actual = list_output(&["One", "Two"]);

    assert_eq!(EXPECTED, actual);
}

#[test]
fn given_an_empty_list_list_output_should_produce_an_empty_output() {
    const EXPECTED: &str = "";

    let actual = list_output::<&str>(&[]);

    assert_eq!(EXPECTED, actual);
}
