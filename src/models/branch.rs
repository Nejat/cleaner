use std::fmt::{Display, Formatter};

use git2::Oid;

pub enum BranchName<'a> {
    Branch(&'a str),
    Head(Oid),
}

impl<'a> Display for BranchName<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Branch(name) =>
                fmt.write_fmt(format_args!("branch: {name}")),
            Self::Head(id) =>
                fmt.write_fmt(format_args!("head: {id}")),
        }
    }
}