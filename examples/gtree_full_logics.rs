#[macro_use]
extern crate goto;

fn main() {
	//let str = "data[name = A3, index=44] is_real_user = true";

	let data = gtree! {
		#result let mut sn = String::with_capacity(24);
		#let mut num = 0usize;
		
		'root {
			sn.push_str("~");
			run_gtree!('test);
		};
		'root => 'test :tree {
			'root2 {
				num += 1;
				sn.push_str("+");
				sn.push_str(&num.to_string());
				
				run_gtree!('test2);
			};
			'root2 => 'test2 {
				num += 2;
				sn.push_str("-");
				sn.push_str(&num.to_string());
				
				if num < 8 {
					anew_gtree!('root2);
				}
			};
		};
	};
	println!("{:?}", data);
	
	/*let hash_map = gtree! {
		#let mut iter = str.as_bytes().iter();
		#let mut buffer = Vec::with_capacity(50);
		#let mut a;
		
		//#result let mut hash_map: HashMap<(), ()> = HashMap::new();
		#result let mut sn = 1024;
		
		'root :loop {
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
		};
		'root => 'decode_value(name: String) :loop {
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
		};
		'root => 'decode_array(name: String) {
			
		};
		
		
		'root => 'end_push_value(name: String, value: String) :clone_anewrun('push_value -> 'root) {
			
		};
		'root => 'end_push_empty_value(name: String) :clone_anewrun('push_empty_value -> 'root) {
			
		};
	};*/
}