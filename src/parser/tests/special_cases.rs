use super::Snapshot;

super::test_snap!(matrix, "[[a,b],[c,d]]");
super::test_snap!(column_vector, "((a),(b))");
super::test_snap!(augmented_matrix, "[[a,b,|,c],[d,e,|,f]]");
super::test_snap!(matrix_layout, "{(2x,+,17y,=,23),(x,-,y,=,5):}");

super::test_snap!(derivatives, "f'(x) = dy/dx");
