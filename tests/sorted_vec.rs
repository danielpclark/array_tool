extern crate array_tool;

#[test]
fn it_implements_individual_uniq_on_sorted_vec() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6].uniq(vec![1, 2, 5, 7, 9]),
        vec![3, 4, 6]
    );
    assert_eq!(vec![1, 2, 3, 4, 5, 6].uniq(vec![6, 7]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn it_doesnt_mutate_on_individual_sorted_uniq() {
    use array_tool::sorted_vec::SortedUniq;
    let a = vec![1, 2, 3, 4, 5, 6];
    a.uniq(vec![0, 2, 5, 6, 8]);
    assert_eq!(a, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn it_implements_individual_uniq_on_sorted_vec_via() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1.1, 2.6, 3.7, 4.7, 5.4, 6.6].uniq_via(
            vec![1.5, 2.7, 5.1, 7.1, 9.4, 10.2],
            |l: &f64, r: &f64| l.floor() == r.floor(),
            |l, r| l.floor() < r.floor()
        ),
        vec![3.7, 4.7, 6.6]
    );
}
