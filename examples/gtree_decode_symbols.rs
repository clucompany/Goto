
#[macro_use]
extern crate goto;
use std::collections::HashMap;

fn main() {
	let data = "BOOT_IMAGE=/boot/vmlinuz-linux-zen root=UUID=xxxx rw quiet";
	let mut hash = HashMap::new();
	
	gtree! {
		#let mut buffer = Vec::with_capacity(50); // local variables
		#let mut iter = data.as_bytes().iter();
		#let mut a;
		
		'decode_symbols {  // root tree
			//write name
			loop {
				a = iter.next();
				match a {
					Some(b'=') => {
						let name = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
						buffer.clear();
						
						// write value
						loop {
							a = iter.next();
							match a {
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
						
					},
					Some(b' ') => match buffer.is_empty() {
						false => run_gtree!('one_value: {
							let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
							buffer.clear();
							data
						}),
						true => continue,
					},
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() {
						true => break 'decode_symbols, // end
						false => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
					}
				}
			}
		};

		// ONE_VALUE tree end_one_value
		'decode_symbols => 'end_one_value(name: String) :clone_anewrun('one_value -> 'decode_symbols) {
			hash.insert(name, None);
		};

		// TWO_VALUE tree end_one_value
		'decode_symbols => 'end_two_value(name: String, value: String) :clone_anewrun('two_value -> 'decode_symbols) {
			hash.insert(name, Some(value));
		};
	}
	
	assert_eq!(
		hash,		
		{
			let mut check_hash = HashMap::new();
			check_hash.insert("root".to_string(), Some("UUID=xxxx".to_string()));
			check_hash.insert("BOOT_IMAGE".to_string(), Some("/boot/vmlinuz-linux-zen".to_string()));
			
			check_hash.insert("rw".to_string(), None);
			check_hash.insert("quiet".to_string(), None);
			check_hash
		},
	);
}