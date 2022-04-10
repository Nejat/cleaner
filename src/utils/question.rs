use once_cell::sync::Lazy;
use std::io::{Read, stdin, stdout, Write};
use std::process::exit;
use std::fmt::{Display, Formatter};

/// Matches new line input
static NEW_LINE: Lazy<u8> = Lazy::new(|| u8::try_from('\n').unwrap());
/// Matches 'y' input
static YES: Lazy<u8> = Lazy::new(|| u8::try_from('y').unwrap());
/// Matches 'n' input
static NO: Lazy<u8> = Lazy::new(|| u8::try_from('n').unwrap());

/// Default answer for a yes/no question
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum DefaultAnswer {
    /// Default answer to Yes
    Yes,

    /// Default answer to No
    No,
}

impl Display for DefaultAnswer {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => fmt.write_str("Y/n"),
            Self::No => fmt.write_str("y/N")
        }
    }
}

/// Response of a yes/no user input question
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Response {
    /// User responded Yes
    Yes {
        /// User response was selected default, by pressing the enter key
        defaulted: bool
    },
    /// User responded No
    No {
        /// User response was selected default, by pressing the enter key
        defaulted: bool
    },
}

/// Queries user with a yes/no question prompt
pub fn yes_no_question(prompt: &str, default_answer: DefaultAnswer) -> Response {
    print!("{prompt} [{default_answer}] ");

    if let Err(err) = stdout().flush() {
        eprintln!("Exception flushing stdout: {}", err);
        eprintln!();
        exit(-1);
    }

    let mut character = [0];

    match stdin().read(&mut character) {
        Ok(_) => {
            match character[0].to_ascii_lowercase() {
                input if input == *YES => Response::Yes { defaulted: false },
                input if input == *NO => Response::No { defaulted: false },
                input =>
                    match default_answer {
                        DefaultAnswer::Yes => Response::Yes { defaulted: input == *NEW_LINE },
                        DefaultAnswer::No => Response::No { defaulted: input == *NEW_LINE }
                    }
            }
        }
        Err(err) => {
            eprintln!("Exception reading user input: {}", err);
            eprintln!();
            exit(-1);
        }
    }
}
