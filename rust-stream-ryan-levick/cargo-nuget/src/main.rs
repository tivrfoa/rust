use cargo_toml::Manifest;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");

	let opt = Opt::from_args();
	println!("{:?}", opt.subcommand);

	match opt.subcommand {
		Subcommand::Install(i) => {
			do_install().unwrap();
		},
	}
}

/// A utility for interacting with nuget package
#[derive(StructOpt, Debug)]
#[structopt(name = "nuget")]
struct Opt {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,

}

#[derive(Debug, StructOpt)]
enum Subcommand {
	Install(Install),
}

#[derive(Debug, StructOpt)]
pub struct Install {}

fn do_install() -> std::io::Result<()> {
	let bytes = std::fs::read("Cargo.toml")?;
	let manifest = Manifest::from_slice(&bytes).unwrap();
	println!("{:#?}", manifest);

	Ok(())
}
