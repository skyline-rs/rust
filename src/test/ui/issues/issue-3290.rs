// run-pass
#![feature(box_syntax)]
#![allow(dead_code)]

pub fn main() {
   let mut x: Box<_> = box 3;
   x = x;
   assert_eq!(*x, 3);
}
