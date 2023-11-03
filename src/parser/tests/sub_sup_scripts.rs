use super::Snapshot;

super::test_snap!(n_to_infty, "n->oo");
super::test_snap!(n_to_infty_grouping, "(n->oo)");
super::test_snap!(complex_subscripts, "lim_(N->oo) sum_(i=0)^N");
