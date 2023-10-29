use super::Snapshot;

use crate::lexer::Tokenize;

super::test_snap!(eq_neq, "= != =!=");
super::test_snap!(compare, "< <= > >= mlt gg");
super::test_snap!(prec_suc, "-< -<= preceq prec succ >-=");
super::test_snap!(sets, "!in in sub sube supe");
super::test_snap!(other, "_= ~= ~~ prop approx cong");
