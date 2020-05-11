// https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/15
use std::io;
use std::str::FromStr;
use std::error::Error;

fn read_line<T: FromStr>() -> Result<T, Box<Error>>
where <T as FromStr>::Err: Error + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn main() -> Result<(), Box<Error>> {
    let u: u32 = read_line()?;
    let v: String = read_line()?;
    println!("{}", u);
    Ok(())
}
