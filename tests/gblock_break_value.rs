
#[macro_use]
extern crate goto;

#[test]
fn gblock_break_value() {
	let mut default_num = 1023;
	let e = gblock!['value, 'value2 (new_value(default_num)):
		default_num += 1;
	];
	
	assert_eq!(e, new_value(1024));

	#[inline]
	fn new_value(u: usize) -> usize {
		u * 2
	}
}
