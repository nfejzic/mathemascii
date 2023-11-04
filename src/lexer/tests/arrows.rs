use crate::lexer::tests::Snapshot;

super::test_snap!(right, "rarr rightarrow");
super::test_snap!(big_left, "lArrLeftarrow");
super::test_snap!(too_long, "abcdefghijklmnopqrstuvwxy");
super::test_snap!(multiple, "uarr->|->twoheadrightarrowtailto");
