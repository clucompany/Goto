
#[macro_use]
extern crate goto;

#[test]
fn gblock_array_decode() {
	let data = b"1=2 2= 3=4";
	let mut iter = data.iter();
	
	let mut a;
	let mut result = Vec::with_capacity(6);
	let mut buffer = Vec::with_capacity(10);
	
	#[derive(Debug, PartialEq)]
	struct Element(Vec<u8>, Option<Vec<u8>>);
	
	gblock!['write_name:
		loop {
			a = iter.next();
			match a {
				Some(b'=') => break,
				Some(b' ') => {
					if !buffer.is_empty() {
						result.push(Element({
							let a = buffer.to_owned();
							buffer.clear();
							a
						}, None));
					}
					
					continue 'write_name;
				},
				Some(a) => buffer.push(*a),
				_ => {
					if !buffer.is_empty() {
						result.push(Element(buffer, None));
					}
					
					break 'write_name;
				},
			}
		}
		
		let name = buffer.to_owned();
		buffer.clear();
		
		gblock!['write_value:
			loop {
				a = iter.next();
				match a {
					Some(b' ') => {
						match buffer.is_empty() {
							true => result.push(Element(name, None)),
							_ => result.push(Element(name, {
								let a = buffer.to_owned();
								buffer.clear();
								
								Some(a)
							})),
						}
						continue 'write_name;
					},
					Some(a) => buffer.push(*a),
					_ => {
						match buffer.is_empty() {
							true => result.push(Element(name, None)),
							_ => result.push(Element(name, {
								let a = buffer.to_owned();
								buffer.clear();
								
								Some(a)
							})),
						}
						break 'write_name;
					},
				}
			}
		];
		
		continue 'write_name;
	];
	
	assert_eq!(result, vec![
		Element(vec![b'1'], Some(vec![b'2'])),
		Element(vec![b'2'], None),
		Element(vec![b'3'], Some(vec![b'4'])),
	]);
	//1=2 2= 3=4
}