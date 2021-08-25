use std::io::BufReader;
use std::fs::File;
use std::io::Result;
use std::env;


fn main() -> Result<()> {

	let args : Vec<String> = env::args().collect();
	let (filename, query) = args_analysis(args);

	let file = File::open(filename)?;
	let	mut buf = BufReader::new(file);

	

	Ok(())

}

fn args_analysis(args : Vec<String>) -> (String, String) {

	let filename = args[1];
	let query = args[2];

	(filename, query)
}