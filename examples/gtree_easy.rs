
#[macro_use]
extern crate goto;


fn main() {
	let data = "test=test2 test test3=test";
	let mut buffer = Vec::with_capacity(50);
	let mut iter = data.as_bytes().iter();
	gtree! {
		'decode_symbols {
			//write name
			loop {
				match iter.next() {
					Some(b'=') => break, // WRITE VALUE
					Some(b' ') => match buffer.is_empty() {
						false => run_gtree!('one_value: {
							let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_vec()) };
							buffer.clear();
							data
						}),
						true => continue,
					},
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() {
						false => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
						_ => break 'decode_symbols, // end
					}
				}
			}
			//write value
			let name = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
			buffer.clear();
			loop {
				match iter.next() {
					Some(b' ') => run_gtree!('two_value: name, {
						let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
						buffer.clear();
						data
					}),
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() { // name
						true => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
						false => run_gtree!('end_two_value: name, unsafe { std::string::String::from_utf8_unchecked(buffer) }),
					}
				}
			}
		};
		
		// ONE_VALUE
		'decode_symbols => 'end_one_value(name: String) {
			println!("#1 {}", name);
		};
		'decode_symbols => 'one_value(name: String) {
			println!("#1 {}", name);
			continue 'decode_symbols;
		};
		
		
		// TWO_VALUE
		'decode_symbols => 'end_two_value(name: String, value: String) {
			println!("#2 {} {}", name, value);
		};
		'decode_symbols => 'two_value(name: String, value: String) {
			println!("#2 {} {}", name, value);
			continue 'decode_symbols;
		};
	}
	
	/*let mut number = 0;
	gtree! {
		'root {
			println!("Number {:?}", number);
			match number {
				0 => run_gtree!('two_value),
				_ => run_gtree!('one_value), // to ove_value
			}
		};
		'root => 'one_value {
			println!("One_value");
		};
		'root => 'two_value :{
			'root2 {
				println!("Я тут:)");
				run_gtree!('is_ok); // to is_ok
			};
			'root2 => 'is_ok {
				println!("Теперь тут");
				number = 1;
				anew_gtree!('root); // в самое начало
			};
		};
	}*/
	
	
	/*
	let file_path = Path::new("./gblock_logic");
	let file_data: Cow<str>;
	gtree! {
		'exists_file {
			let mut file = match std::fs::File::open(&file_path) {
				Ok(a) => a,
				Err(_e) => run_gtree!('file_no_exists),
			};
			
			let mut buff = String::with_capacity(25);
			if let Err(_e) = file.read_to_string(&mut buff) {
				run_gtree!('file_no_exists);
			}
			
			if buff.is_empty() { // no empty data...
				run_gtree!('file_no_exists);
			}
			file_data = buff.into(); // String -> Cow<str>
			run_gtree!('file_exists); //OK
		};
		
		'exists_file => 'file_no_exists() {
			let mut file = match std::fs::File::create(&file_path) {
				Ok(a) => a,
				Err(e) => panic!("{:?}", e),
			};
			let data = "FALSE";
			if let Err(e) = file.write(data.as_bytes()) {
				panic!("{:?}", e);
			}
			file_data = data.into(); // str -> Cow<str>
		};
		
		'exists_file => 'file_exists {
			
		};
	}
	println!("{:?}", file_data);*/
}