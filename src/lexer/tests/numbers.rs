use super::Snapshot;

super::test_snap!(only_number, "24.42");
super::test_snap!(start_with_dot, ".42 .42a");
super::test_snap!(start_and_stop_dot, ".42.3");
super::test_snap!(stop_at_non_digit, "24.42a");
super::test_snap!(stop_at_dot, "24.42.");
