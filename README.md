#	Goto
A safe but not complete implementation of the goto operator.

[![Build Status](https://travis-ci.org/clucompany/Goto.svg?branch=master)](https://travis-ci.org/clucompany/Goto)
[![Apache licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/goto)](https://crates.io/crates/goto)
[![Documentation](https://docs.rs/goto/badge.svg)](https://docs.rs/goto)

# Attention!!
1. At the moment, this is not a complete implementation of the goto operator.
2. We do not plan to violate Rust's safety standards. We provide useful and interesting macros that partially (or fully) implement the goto operator.


# gpoint:
"GOTO point", allows you to return to this line later.

```rust
#[macro_use]
extern crate goto;

fn main() {
	let data = b"1234567890";
	let mut iter = data.iter();
	let mut a;

	gpoint!['begin:
		a = iter.next();
		match a {
			a @ Some(b'0') if a == Some(&b'9') => {
				println!("#a 0!");
				
				gpoint!['add:
					
				];
			},
			Some(a) => {
				println!("#a {}", unsafe { std::char::from_u32_unchecked(*a as u32) });
				continue 'begin;
			},
			_ => break 'begin,
		}
		
	];
}

```

# License

Copyright 2019 #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License
