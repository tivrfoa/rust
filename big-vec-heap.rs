const N: usize = 100_000;
const M: usize = 1_000;
// const BIG: [[i32; M]; N] = [[33; M]; N];
// static BIG: [[i32; M]; N] = [[33; M]; N];
fn main() {
	let mut big: Vec<Vec<i32>> = vec![vec![33; M]; N];
	println!("{:?}", big[10][20]);
}
