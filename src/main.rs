extern crate wham;
#[macro_use]
extern crate clap;

use clap::App;
use wham::Config;
use std::error::Error;
use std::result::Result;
use std::process;
use std::env;

// Parse command line arguments into a Config struct
fn cli() -> Result<Config, Box<Error>> {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let metadata_file = matches.value_of("metadata").unwrap().to_string();
	let hist_min: f32 = matches.value_of("min_hist").unwrap().parse()?;
	let hist_max: f32 = matches.value_of("max_hist").unwrap().parse()?;
	let num_bins: usize = matches.value_of("bins").unwrap().parse()?;
	let verbose: bool = matches.is_present("verbose");
	let temperature: f32 = matches.value_of("temperature").unwrap().parse()?;

	let tolerance: f32;
	if matches.is_present("tolerance") {
		tolerance = matches.value_of("tolerance").unwrap().parse()?;
	} else {
		tolerance = std::f32::MIN_POSITIVE;
	}
	let max_iterations: usize;
	if matches.is_present("iterations") {
		max_iterations = matches.value_of("iterations").unwrap().parse()?;
	} else {
		max_iterations = std::usize::MAX;
	}
	Ok(wham::Config{metadata_file, hist_min, hist_max, num_bins,
		verbose, tolerance, max_iterations, temperature})
}

fn main() {
	let cfg = cli().expect("Failed to parse CLI.");
	match wham::run(&cfg) {
		Err(_) => process::exit(1),
		_ => {}
	}
}
