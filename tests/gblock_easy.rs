
#[macro_use]
extern crate goto;

#[test]
fn gblock_easy() {
	let mut num = 0;
	
	gblock!['add_one:
		num += 1;
		
		gblock!['add_two:
			num += 2;
			
			gblock!['add_three:
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