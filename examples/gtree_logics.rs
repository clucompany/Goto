
#[macro_use]
extern crate goto;

use std::borrow::Cow;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
	let file_path = Path::new("./gblock_logic");
	let file_data: Cow<str>;
	gtree! {
		'checker {
			// decode file
			let mut file = match std::fs::File::open(&file_path) {
				Ok(a) => a,
				Err(e) => run_gtree!('err_load_file: e),
			};
			
			let mut buff = String::with_capacity(25);
			if let Err(e) = file.read_to_string(&mut buff) {
				run_gtree!('err_load_file: e);
			}
			
			if buff.is_empty() { // no empty data...
				run_gtree!('err_load_file: Error::new(ErrorKind::Other, "empty file!"));
			}
			file_data = buff.into(); // String -> Cow<str>
		};
		
		'checker => 'full_unk_error(error) {
			panic!("{:?}", error);
		};
		
		'checker => 'err_load_file(error) {
			println!("Err, read file, {:?}", error);
			
			let mut file = match std::fs::File::create(&file_path) {
				Ok(a) => a,
				Err(e) => run_gtree!('full_unk_error: e),
			};
			let data = "FALSE";
			if let Err(e) = file.write(data.as_bytes()) {
				run_gtree!('full_unk_error: e);
			}
			file_data = data.into(); // str -> Cow<str>
		};
	}
	
	println!("{:?}", file_data);
}
