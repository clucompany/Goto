
#[macro_use]
extern crate goto;

#[test]
fn gpoint() {
	let mut num = 0;
	
	gpoint!['add_one:
		num += 1;
		
		gpoint!['add_two:
			num += 2;
			
			gpoint!['add_three:
				num += 3;
				
				if num != 54 {
					continue 'add_one;
				}
				break 'add_one;
			];
		];
	];
	assert_eq!(num, 54);
}