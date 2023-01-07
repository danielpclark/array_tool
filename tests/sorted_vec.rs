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
fn it_implements_individual_uniq_on_descending_sorted_vec() {
    use array_tool::sorted_vec::SortedUniq;
    assert_eq!(
        vec![6, 5, 4, 3, 2, 1].uniq_desc(vec![9, 7, 5, 3, 1]),
        vec![6, 4, 2]
    );
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
    assert!(vec![1, 2, 3, 4].is_unique());
    assert!(!vec![1, 2, 2, 4].is_unique());
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
    assert!(vec![1.2, 2.5, 3.3, 4.4].is_unique_via(|l: &f64, r: &f64| l.floor() == r.floor()));
    assert!(!vec![1.1, 2.2, 2.7, 3.3].is_unique_via(|l: &f64, r: &f64| l.floor() == r.floor()));
}

#[test]
fn it_intersects() {
    use array_tool::sorted_vec::SortedIntersect;
    assert_eq!(vec![1, 1, 3, 5].intersect(vec![1, 2, 3]), vec![1, 3]);
}

#[test]
fn it_intersects_descending_arrays() {
    use array_tool::sorted_vec::SortedIntersect;
    assert_eq!(
        vec![6, 5, 4, 3, 2, 1].intersect_desc(vec![4, 2, 0]),
        vec![4, 2]
    );
}

#[test]
fn it_intersects_if() {
    use array_tool::sorted_vec::SortedIntersect;
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 7, 8].intersect_if(
            vec![2, 4, 6, 8, 10, 12],
            |l, r| l == r,
            |l, r| l < r
        ),
        vec![2, 4, 6, 8]
    );
    assert_eq!(
        vec!['a', 'a', 'c', 'e'].intersect_if(
            vec!['A', 'B', 'C', 'E'],
            |l, r| l.eq_ignore_ascii_case(r),
            |l, r| l < &r.to_ascii_lowercase()
        ),
        vec!['a', 'c', 'e']
    );
}
