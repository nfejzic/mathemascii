use super::Snapshot;

use crate::lexer::Tokenize;

super::test_snap!(roots, "sqrtroot");
super::test_snap!(calculus, "int oint del grad oo |...|");
super::test_snap!(shapes, "square diamond triangle /_\\ frown /_");
super::test_snap!(number_sets, "CCNNQQRRZZ");
super::test_snap!(floor_ceil_power, "^|__ __| |~ rceiling +-");
