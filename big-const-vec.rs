const N: usize = 100_000;
const M: usize = 1_000;

// error[E0015]: calls in constants are limited to
// constant functions, tuple structs and tuple variants
// --> big-const-vec.rs:3:33
//  |
//3 | const BIG: Vec<Vec<i32>> = vec![vec![33; M]; N];
//  |                                 ^^^^^^^^^^^
const BIG: Vec<Vec<i32>> = vec![vec![33; M]; N];
// static BIG: [[i32; M]; N] = [[33; M]; N];
fn main() {
println!("{:?}", BIG[0][0]);
}
