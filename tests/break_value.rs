

#[macro_use]
extern crate goto;

#[test]
fn break_value() {
	let mut a = 0;
	let e = gpoint!['value + 'value2 (new_value(1024)):
		a += 1;
	];
	a += 1;
	
	assert_eq!(e, new_value(1024));
	assert_eq!(a, 2);

	fn new_value(u: usize) -> usize {
		u * 2	
	}
}
