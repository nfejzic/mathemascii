use super::Snapshot;

use super::super::*;

super::test_snap!(sin_cos, "sincos");
super::test_snap!(f_g, "fg");
super::test_snap!(ln_log, "ln logln");
super::test_snap!(too_long, "abcdefghijklmnopqrstuvwxyf");
super::test_snap!(multiple, "sincoshfg");
super::test_snap!(precedence, "gammag");
