use super::Snapshot;

use crate::lexer::Tokenize;

super::test_snap!(plus_minus, "+ -");
super::test_snap!(stars, "*** ** *");
super::test_snap!(ltimes_rtimes_bowtie, "|>< |><| ><|");
super::test_snap!(multiple, "+ - @ox^^^^^");
super::test_snap!(precedence, "nnnnn");
