use std::process::exit;
use std::sync::Once;

/// Creates an easier to read comma separated output from a list
pub fn list_output<T: AsRef<str>>(source: &[T]) -> String {
    /// Separator for creating a list for string values for display
    const SEPARATOR: &str = ", ";

    let mut output = String::default();

    if source.is_empty() {
        return output;
    }

    let mut add_separator = false;
    let skip_first = Once::new();

    for item in source.iter().take(source.len() - 1) {
        if add_separator { output.push_str(SEPARATOR); }

        skip_first.call_once(|| add_separator = true);

        output.push_str(item.as_ref());
    }

    if let Some(last) = source.last() {
        if !output.is_empty() {
            output.push_str(" & ");
        }

        output.push_str(last.as_ref());
    }

    output
}

#[inline]
pub fn display_error_and_exit(message: &str) -> ! {
    eprintln!("\n{}\n", message);

    exit(-1);
}
