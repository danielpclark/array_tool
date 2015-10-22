#![feature(test)]

extern crate test;
use test::Bencher;
extern crate array_tool;
use array_tool::*;

#[bench]
fn uniques1(b: &mut Bencher){
  b.iter(|| { 
    let a: Vec<i32> = vec![1,43,6,26,62,7,27,2,3,62,246,2346,6,7,2,35,62,6,325,56,63,25,4,8,8727,7,74,452];
    let d: Vec<i32> = vec![36,63,74,872,2,772,7,2,54,265,3245,45,754,235,4567,67,23,2,542,352];
    uniques(a, d) 
  })
}
