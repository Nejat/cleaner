use inquire::Confirm;

pub use display::{display_error_and_exit, list_output};
pub use platforms::{load_supported_platforms, path_of_supported_platforms_configuration};
pub use validation::{
    validate_path, validate_platform, validate_platforms, validate_platforms_filter,
    validate_unique_values,
};

mod display;
mod platforms;
mod validation;

pub fn get_confirmation(msg: &&str) -> bool {
    let confirmation = Confirm::new(&format!("remove {msg}"))
        .with_default(false)
        .with_placeholder("N")
        .prompt();

    match confirmation {
        Ok(answer) => answer,
        Err(err) => {
            display_error_and_exit(&format!("Exception processing input: {err}"));
        }
    }
}
