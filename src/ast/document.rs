use rowan::ast::AstNode;

use crate::Org;

use super::{Document, Keyword, SyntaxKind};

impl Document {
    /// ```rust
    /// use orgize::{Org, ast::Document};
    ///
    /// let org = Org::parse(r#"
    /// #+TITLE: hello
    /// #+TITLE: world
    /// #+DATE: tody
    /// #+AUTHOR: poi"#);
    /// let doc = org.first_node::<Document>().unwrap();
    /// assert_eq!(doc.keywords().count(), 4);
    /// ```
    pub fn keywords(&self) -> impl Iterator<Item = Keyword> {
        self.syntax
            .first_child()
            .filter(|c| c.kind() == SyntaxKind::SECTION)
            .into_iter()
            .flat_map(|section| section.children().filter_map(Keyword::cast))
    }

    /// Returns the value in `#+TITLE`
    ///
    /// ```rust
    /// use orgize::{Org, ast::Document};
    ///
    /// let org = Org::parse("#+TITLE: hello\n#+TITLE: world");
    /// let doc = org.first_node::<Document>().unwrap();
    /// assert_eq!(doc.title().unwrap(), "hello world");
    ///
    /// let org = Org::parse("");
    /// let doc = org.first_node::<Document>().unwrap();
    /// assert!(doc.title().is_none());
    /// ```
    pub fn title(&self) -> Option<String> {
        self.keywords()
            .filter(|kw| kw.key().eq_ignore_ascii_case("TITLE"))
            .fold(Option::<String>::None, |acc, cur| {
                let mut s = acc.unwrap_or_default();
                if !s.is_empty() {
                    s.push(' ');
                }
                s.push_str(cur.value().trim());
                Some(s)
            })
    }
}

impl Org {
    /// Equals to `self.document().title()`, see [Document::title]
    pub fn title(&self) -> Option<String> {
        self.document().title()
    }

    /// Equals to `self.document().keywords()`, see [Document::keywords]
    pub fn keywords(&self) -> impl Iterator<Item = Keyword> {
        self.document().keywords()
    }
}
