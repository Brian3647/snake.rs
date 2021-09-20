use std::process::exit;

use structopt::StructOpt;

pub fn get_args() -> Args {
	Args::from_optargs(OptArgs::from_args())
}

#[derive(Debug, StructOpt)]
pub struct OptArgs {
	#[structopt(short = "h", long = "height")]
	pub height: Option<u16>,

	#[structopt(short = "w", long = "width")]
	pub width: Option<u16>
}

pub struct Args {
	pub height: u16,
	pub width: u16
}

impl Args {
	pub fn from_optargs(args: OptArgs) -> Self {
		Self {
			height: args.height.unwrap_or(Self::default_height()),
			width: args.width.unwrap_or(Self::default_width())
		}
	}

	pub fn default_height() -> u16 {
		if let Some((_, h)) = term_size::dimensions() {
			(h - 2) as u16
		} else {
			println!("Can't get terminal dimensions :(");
			exit(1)
		}
	}

	pub fn default_width() -> u16 {
		if let Some((w, _)) = term_size::dimensions() {
			(w - 2) as u16
		} else {
			println!("Can't get terminal dimensions :(");
			exit(1)
		}
	}
}
