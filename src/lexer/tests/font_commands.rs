use crate::lexer::tests::Snapshot;

use super::super::*;

super::test_snap!(font_bold, "bb bbb mathbf mathbb");
super::test_snap!(various_commands, "cc mathtt mathcal fr sf mathfrak");
