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
fn it_can_return_its_own_sorted_unique() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1, 2, 2, 3, 4, 8, 8, 9].unique(),
        vec![1, 2, 3, 4, 8, 9]
    );
    assert_eq!(
        vec![1, 2, 2, 3, 3, 3, 6, 7, 8, 8, 8, 8, 9, 11, 11, 11, 11, 11, 11].unique(),
        vec![1, 2, 3, 6, 7, 8, 9, 11]
    );
}

#[test]
fn it_answers_about_sorted_uniqueness() {
    use array_tool::vec::Uniq;
    assert_eq!(vec![1, 2, 3, 4].is_unique(), true);
    assert_eq!(vec![1, 2, 2, 4].is_unique(), false);
}

#[test]
fn it_implements_individual_uniq_on_sorted_vec_via() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1.1, 2.6, 3.7, 4.7, 5.4, 5.2, 6.6].uniq_via(
            vec![1.5, 2.7, 5.1, 7.1, 9.4, 10.2],
            |l: &f64, r: &f64| l.floor() == r.floor(),
            |l, r| l.floor() < r.floor()
        ),
        vec![3.7, 4.7, 6.6]
    );
    assert_eq!(
        vec![5, 4, 3, 2, 1].uniq_via(vec![8, 6, 4, 2], |l, r| l == r, |l, r| l > r),
        vec![5, 3, 1]
    );
}

#[test]
fn it_can_return_its_own_sorted_unique_via() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1.2, 2.5, 2.9, 2.9, 4.1, 4.4].unique_via(|l: &f64, r: &f64| l.floor() == r.floor()),
        vec![1.2, 2.5, 4.1]
    );
}

#[test]
fn it_answers_about_sorted_uniqueness_via() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![1.2, 2.5, 3.3, 4.4].is_unique_via(|l: &f64, r: &f64| l.floor() == r.floor()),
        true
    );
    assert_eq!(
        vec![1.1, 2.2, 2.7, 3.3].is_unique_via(|l: &f64, r: &f64| l.floor() == r.floor()),
        false
    );
}
