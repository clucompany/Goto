
#[macro_use]
extern crate goto;

fn main() {
	let data = b"1234567890";
	let mut iter = data.iter();
	let mut a;

	gblock!['root:
		a = iter.next();
		match a {
			Some(b'0') => {
				println!("#0");
				
				gblock!['add:
					
				];
			},
			Some(a) => {
				println!("#a {:?}", a);
				continue 'root;
			},
			_ => {},
		}
		
	];
}
