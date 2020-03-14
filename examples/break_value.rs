
#[macro_use]
extern crate goto;

fn main() {
	let mut default_result = 1023;
	let e = gblock!['value, 'value2 -> (new_value(default_result)):
		default_result += 1;
	];
	
	assert_eq!(e, new_value(1024));
}

fn new_value(u: usize) -> usize {
	u * 2	
}
