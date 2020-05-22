const N: usize = 100_000;
const M: usize = 1_000;
const BIG: [[i32; M]; N] = [[33; M]; N];
// static BIG: [[i32; M]; N] = [[33; M]; N];
fn main() {
println!("{:?}", BIG[0][0]);
}
