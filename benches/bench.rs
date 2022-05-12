#![feature(test)]

// NOTES
//
// string slices are faster than character iteration with method nth.

extern crate test;
use test::Bencher;
extern crate array_tool;
use array_tool::*;

#[bench]
fn uniques1(b: &mut Bencher) {
    let a: Vec<i32> = vec![
        1, 43, 6, 26, 62, 7, 27, 2, 3, 62, 246, 2346, 6, 7, 2, 35, 62, 6, 325, 56, 63, 25, 4, 8,
        8727, 7, 74, 452,
    ];
    let d: Vec<i32> = vec![
        36, 63, 74, 872, 2, 772, 7, 2, 54, 265, 3245, 45, 754, 235, 4567, 67, 23, 2, 542, 352,
    ];
    b.iter(|| vec::Uniq::uniq(&a, d.clone()))
}

#[bench]
fn sorted_uniques(b: &mut Bencher) {
    let a: Vec<i32> = vec![
        1, 2, 4, 7, 11, 12, 15, 15, 19, 22, 39, 50, 51, 52, 102, 104, 150, 230, 280, 400, 401, 402,
        8231, 49823, 109482,
    ];
    let d: Vec<i32> = vec![
        1, 2, 3, 6, 8, 11, 15, 19, 50, 102, 103, 108, 120, 160, 199, 220, 230, 280, 500, 509, 8231,
        29391, 20413,
    ];
    b.iter(|| sorted_vec::SortedUniq::uniq(&a, d.clone()))
}

#[bench]
fn intersects(b: &mut Bencher) {
    let a: Vec<i32> = vec![
        1, 2, 4, 7, 11, 12, 15, 15, 19, 22, 39, 50, 51, 52, 102, 104, 150, 230, 280, 400, 401, 402,
        8231, 49823, 109482,
    ];
    let d: Vec<i32> = vec![
        1, 2, 3, 6, 8, 11, 15, 19, 50, 102, 103, 108, 120, 160, 199, 220, 230, 280, 500, 509, 8231,
        29391, 20413,
    ];

    b.iter(|| vec::Intersect::intersect(&a, d.clone()))
}

#[bench]
fn sorted_intersects(b: &mut Bencher) {
    let a: Vec<i32> = vec![
        1, 2, 4, 7, 11, 12, 15, 15, 19, 22, 39, 50, 51, 52, 102, 104, 150, 230, 280, 400, 401, 402,
        8231, 49823, 109482,
    ];
    let d: Vec<i32> = vec![
        1, 2, 3, 6, 8, 11, 15, 19, 50, 102, 103, 108, 120, 160, 199, 220, 230, 280, 500, 509, 8231,
        29391, 20413,
    ];

    b.iter(|| sorted_vec::SortedIntersect::intersect(&a, d.clone()))
}

#[bench]
fn times(b: &mut Bencher) {
    b.iter(|| {
        use array_tool::vec::Times;
        vec![1, 2, 3, 4, 5, 6].times(150);
    });
}

#[bench]
fn subst_marks(b: &mut Bencher) {
    b.iter(|| {
        use array_tool::string::SubstMarks;
        "dfgklerfgseawrfgawergq35g4w6uw4372472q4762q47yq35uw4567u32qy7q3yuq3"
            .subst_marks(vec![0, 3, 6, 9, 12, 24, 34, 40], "Z");
    });
}

#[bench]
fn word_wrap(b: &mut Bencher) {
    b.iter(|| {
        use array_tool::string::WordWrap;
        "asdf sdf s df d sd\n sf  sfg  sg   g\n      sfdgsg\n gfdga a\n     ".word_wrap(3);
    });
}
