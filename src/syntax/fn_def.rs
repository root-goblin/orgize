use nom::{
    bytes::complete::{tag, take_while1},
    combinator::map,
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        blank_lines, colon_token, l_bracket_token, node, r_bracket_token, trim_line_end,
        GreenElement, NodeBuilder,
    },
    input::Input,
    keyword::affiliated_keyword_nodes,
    object::standard_object_nodes,
    SyntaxKind::*,
};

#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn fn_def_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(fn_def_node_base, input)
}

fn fn_def_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            affiliated_keyword_nodes,
            l_bracket_token,
            tag("fn"),
            colon_token,
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            r_bracket_token,
            trim_line_end,
            blank_lines,
        )),
        |(
            affiliated_keywords,
            l_bracket,
            fn_,
            colon,
            label,
            r_bracket,
            (content, ws_, nl),
            post_blank,
        )| {
            let mut b = NodeBuilder::new();

            b.children.extend(affiliated_keywords);
            b.push(l_bracket);
            b.push(fn_.token(KEYWORD));
            b.push(colon);
            b.push(label.token(FN_LABEL));
            b.push(r_bracket);

            let content_node = node(FN_CONTENT, standard_object_nodes(content));
            b.push(content_node);

            b.ws(ws_);
            b.nl(nl);
            b.children.extend(post_blank);
            b.finish(FN_DEF)
        },
    );
    let (i, fn_def) = parser(input)?;
    Ok((i, fn_def))
}

#[test]
fn parse() {
    use crate::ParseConfig;
    use crate::{ast::FnDef, tests::to_ast};

    let to_fn_def = to_ast::<FnDef>(fn_def_node);

    insta::assert_debug_snapshot!(
         to_fn_def("[fn:1] *bold* -  https://orgmode.org").syntax,
         @r#"
    FN_DEF@0..36
      L_BRACKET@0..1 "["
      KEYWORD@1..3 "fn"
      COLON@3..4 ":"
      FN_LABEL@4..5 "1"
      R_BRACKET@5..6 "]"
      FN_CONTENT@6..36
        TEXT@6..7 " "
        BOLD@7..13
          STAR@7..8 "*"
          TEXT@8..12 "bold"
          STAR@12..13 "*"
        TEXT@13..36 " -  https://orgmode.org"
    "#
    );

    insta::assert_debug_snapshot!(
         to_fn_def("[fn:word_1] https://orgmode.org").syntax,
         @r#"
    FN_DEF@0..31
      L_BRACKET@0..1 "["
      KEYWORD@1..3 "fn"
      COLON@3..4 ":"
      FN_LABEL@4..10 "word_1"
      R_BRACKET@10..11 "]"
      FN_CONTENT@11..31
        TEXT@11..31 " https://orgmode.org"
    "#
    );

    insta::assert_debug_snapshot!(
         to_fn_def("[fn:WORD-1] https://orgmode.org").syntax,
         @r#"
    FN_DEF@0..31
      L_BRACKET@0..1 "["
      KEYWORD@1..3 "fn"
      COLON@3..4 ":"
      FN_LABEL@4..10 "WORD-1"
      R_BRACKET@10..11 "]"
      FN_CONTENT@11..31
        TEXT@11..31 " https://orgmode.org"
    "#
    );

    insta::assert_debug_snapshot!(
         to_fn_def("[fn:WORD]").syntax,
         @r#"
    FN_DEF@0..9
      L_BRACKET@0..1 "["
      KEYWORD@1..3 "fn"
      COLON@3..4 ":"
      FN_LABEL@4..8 "WORD"
      R_BRACKET@8..9 "]"
      FN_CONTENT@9..9
    "#
    );

    insta::assert_debug_snapshot!(
         to_fn_def("[fn:1] In particular, the parser requires stars at column 0 to be\n").syntax,
         @r#"
    FN_DEF@0..66
      L_BRACKET@0..1 "["
      KEYWORD@1..3 "fn"
      COLON@3..4 ":"
      FN_LABEL@4..5 "1"
      R_BRACKET@5..6 "]"
      FN_CONTENT@6..65
        TEXT@6..65 " In particular, the p ..."
      NEW_LINE@65..66 "\n"
    "#
    );

    let config = &ParseConfig::default();

    assert!(fn_def_node(("[fn:] https://orgmode.org", config).into()).is_err());
    assert!(fn_def_node(("[fn:wor d] https://orgmode.org", config).into()).is_err());
    assert!(fn_def_node(("[fn:WORD https://orgmode.org", config).into()).is_err());

    insta::assert_debug_snapshot!(
         to_fn_def("#+ATTR_poi: 1\n[fn:WORD-1] https://orgmode.org").syntax,
         @r##"
    FN_DEF@0..45
      AFFILIATED_KEYWORD@0..14
        HASH_PLUS@0..2 "#+"
        TEXT@2..10 "ATTR_poi"
        COLON@10..11 ":"
        TEXT@11..13 " 1"
        NEW_LINE@13..14 "\n"
      L_BRACKET@14..15 "["
      KEYWORD@15..17 "fn"
      COLON@17..18 ":"
      FN_LABEL@18..24 "WORD-1"
      R_BRACKET@24..25 "]"
      FN_CONTENT@25..45
        TEXT@25..45 " https://orgmode.org"
    "##
    );
}
