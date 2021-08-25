use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::io::Result;
use std::env;
use std::str;


fn main() -> Result<()> {

	let args : Vec<String> = env::args().collect();
	let (filename, query) = args_analysis(&args);

	let file = File::open(filename)?;
	let	mut buf = BufReader::new(file);

	search_buf(buf, "mut");

	Ok(())

}

fn args_analysis(args : &[String]) -> (String, String) {
	let filename = &args[1];
	let query = &args[2];

	(filename.clone(), query.clone())
}

fn search_buf(buf : BufReader<File>, pattern : &str) {
	for line in buf.lines() {
		match line {
			Ok(stri) => {
				if stri.contains(pattern) {
					println!("{}", stri);
				}
			}
			Err(error) => {
				return;
			}
		}
	}
}