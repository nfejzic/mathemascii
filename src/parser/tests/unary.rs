use super::Snapshot;

super::test_snap!(accents, "hat x vec x tilde x");
super::test_snap!(others, "sqrt x text(some text)");
super::test_snap!(groupings, "abs(x) floor(x) norm x");
super::test_snap!(font_commands, "fr text(other text)");
