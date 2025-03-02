//! Parse `.gitattribute` files and provide utilities to match against them.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

use byteyarn::{Yarn, YarnRef};
pub use gix_glob as glob;

mod assignment;
///
pub mod name;
///
pub mod state;

///
pub mod search;

///
pub mod parse;

/// Parse attribute assignments line by line from `bytes`, and fail the operation on error.
///
/// For leniency, ignore errors using `filter_map(Result::ok)` for example.
pub fn parse(bytes: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(bytes)
}

/// The state an attribute can be in, referencing the value.
///
/// Note that this doesn't contain the name.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum StateRef<'a> {
    /// The attribute is listed, or has the special value 'true'
    Set,
    /// The attribute has the special value 'false', or was prefixed with a `-` sign.
    Unset,
    /// The attribute is set to the given value, which followed the `=` sign.
    /// Note that values can be empty.
    Value(state::ValueRef<'a>),
    /// The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign.
    Unspecified,
}

/// The state an attribute can be in, owning the value.
///
/// Note that this doesn't contain the name.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum State {
    /// The attribute is listed, or has the special value 'true'
    Set,
    /// The attribute has the special value 'false', or was prefixed with a `-` sign.
    Unset,
    /// The attribute is set to the given value, which followed the `=` sign.
    /// Note that values can be empty.
    Value(state::Value),
    /// The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign.
    Unspecified,
}

/// Represents a validated attribute name
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Name(pub(crate) Yarn);

/// Holds a validated attribute name as a reference
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct NameRef<'a>(YarnRef<'a, str>);

/// Name an attribute and describe it's assigned state.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Assignment {
    /// The validated name of the attribute.
    pub name: Name,
    /// The state of the attribute.
    pub state: State,
}

/// Holds validated attribute data as a reference
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct AssignmentRef<'a> {
    /// The name of the attribute.
    pub name: NameRef<'a>,
    /// The state of the attribute.
    pub state: StateRef<'a>,
}

/// A grouping of lists of patterns while possibly keeping associated to their base path in order to find matches.
///
/// Pattern lists with base path are queryable relative to that base, otherwise they are relative to the repository root.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct Search {
    /// A list of pattern lists, each representing a patterns from a file or specified by hand, in the order they were
    /// specified in.
    ///
    /// When matching, this order is reversed.
    patterns: Vec<gix_glob::search::pattern::List<search::Attributes>>,
}

/// A list of known global sources for git attribute files in order of ascending precedence.
///
/// This means that values from the first variant will be returned first.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Source {
    /// The attribute file that the installation itself ships with.
    GitInstallation,
    /// System-wide attributes file. This is typically defined as
    /// `$(prefix)/etc/gitattributes` (where prefix is the git-installation directory).
    System,
    /// This is `<xdg-config-home>/git/attributes` and is git application configuration per user.
    ///
    /// Note that there is no `~/.gitattributes` file.
    Git,
    /// The configuration of the repository itself, located in `$GIT_DIR/info/attributes`.
    Local,
}

mod source;
