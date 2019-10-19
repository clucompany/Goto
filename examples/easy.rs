
#[macro_use]
extern crate goto;

fn main() {
	
	let data = b"1234567890";
	let mut iter = data.iter();
	let mut a;

	gpoint!['begin:
		a = iter.next();
		match a {
			Some(b'0') => {
				println!("#0");
				
				gpoint!['add:
					
				];
			},
			Some(a) => {
				println!("#a {:?}", a);
				continue 'begin;
			},
			_ => {},
		}
		
	];
}
