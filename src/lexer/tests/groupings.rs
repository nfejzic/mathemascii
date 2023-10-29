use super::Snapshot;

use crate::lexer::Tokenize;

super::test_snap!(parens, "()(()))");
super::test_snap!(brackets, "[][[]]][");
super::test_snap!(angled, "langle(::)<<>>");
super::test_snap!(ignored, "( :}{:)");
