#![doc = include_str!("../README.md")]

pub mod ast;
pub mod config;
mod entities;
pub mod export;
mod org;
mod replace;
mod syntax;
#[cfg(test)]
mod tests;

// Re-export of the rowan crate.
pub use rowan;

pub use config::ParseConfig;
pub use org::Org;
pub use rowan::{TextRange, TextSize};
pub use syntax::{
    SyntaxElement, SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub(crate) use syntax::combinator::lossless_parser;
