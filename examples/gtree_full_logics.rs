
#[macro_use]
extern crate goto;

fn main() {
	let str = "data[name = A3, index=44] is_real_user = true";
	gtree! {
		#let mut iter = str.as_bytes().iter();
		#let mut buffer = Vec::with_capacity(50);
		#let mut a;
		
		'root {
			loop {
				a = iter.next();
				match a {
					Some(b'=') => run_gtree!('decode_value: {
						let data = buffer.to_owned();
						buffer.clear();
						unsafe { String::from_utf8_unchecked(data) }
					}),
					Some(b'[') => run_gtree!('decode_array: {
						let data = buffer.to_owned();
						buffer.clear();
						unsafe { String::from_utf8_unchecked(data) }
					}),
					Some(b' ') => continue, // ignore space
					Some(a) => buffer.push(*a),
					None => run_gtree!('end_push_empty_value: unsafe {
						String::from_utf8_unchecked(buffer)
					}),
				}
			}
		};
		'root => 'decode_value(name: String) {
			loop {
				a = iter.next();
				match a {
					Some(b',') => anew_gtree!('root),
					Some(b' ') => continue, // ignore space
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() {
						true => run_gtree!('end_push_empty_value: name),
						false => run_gtree!('end_push_value: name, unsafe {
							String::from_utf8_unchecked(buffer)
						}),
					},
				}
			}
		};
		'root => 'decode_array(name: String) {
			
		};
		
		
		'root => 'end_push_value(name: String, value: String) :clone_anewrun('push_value -> 'root) {
			
		};
		'root => 'end_push_empty_value(name: String) :clone_anewrun('push_empty_value -> 'root) {
			
		};
	}
}