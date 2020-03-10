
#[macro_use]
extern crate goto;

use std::borrow::Cow;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() {
	let file_path = Path::new("./gblock_logic");
	let file_data: Cow<str>;
	gblock!['is_create_file:
		gblock!['decode_file:
			// decode file
			let mut file = match std::fs::File::open(&file_path) {
				Ok(a) => a,
				Err(_e) => to_end_gblock!('decode_file),
			};
			
			let mut buff = String::with_capacity(25);
			if let Err(_e) = file.read_to_string(&mut buff) {
				to_end_gblock!('decode_file);
			}
			
			if buff.is_empty() { // no empty data...
				to_end_gblock!('decode_file);
			}
			file_data = buff.into(); // String -> Cow<str>
			to_end_gblock!('is_create_file); //OK
		];
		
		// create new file and default value
		let mut file = match std::fs::File::create(&file_path) {
			Ok(a) => a,
			Err(e) => panic!("{:?}", e),
		};
		let data = "FALSE";
		if let Err(e) = file.write(data.as_bytes()) {
			panic!("{:?}", e);
		}
		file_data = data.into(); // str -> Cow<str>
	];
	println!("{:?}", file_data);
	
}
