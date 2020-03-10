#	Goto
A safe but not complete implementation of the goto operator.

[![Build Status](https://travis-ci.org/clucompany/Goto.svg?branch=master)](https://travis-ci.org/clucompany/Goto)
[![Mit/Apache licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/goto)](https://crates.io/crates/goto)
[![Documentation](https://docs.rs/goto/badge.svg)](https://docs.rs/goto)

# Attention!!
1. At the moment, this is not a complete implementation of the goto operator.
2. We do not plan to violate Rust's safety standards. We provide useful and interesting macros that partially (or fully) implement the goto operator.

# gblock:
A safe version of the "goto" prisoner in the block. Ability to move to the beginning of the block or to the end of the block.

```rust
let file_path = Path::new("./gblock_logic");
let file_data: Cow<str>;
gblock!['is_create_file:
	gblock!['decode_file:
		// decode file
		let mut file = match std::fs::File::open(&file_path) {
			Ok(a) => a,
			Err(_e) => to_end_gblock!('decode_file),
		};
		
		let mut buff = String::with_capacity(25);
		if let Err(_e) = file.read_to_string(&mut buff) {
			to_end_gblock!('decode_file);
		}
		
		if buff.is_empty() { // no empty data...
			to_end_gblock!('decode_file);
		}
		file_data = buff.into(); // String -> Cow<str>
		to_end_gblock!('is_create_file); //OK
	];
	
	// create new file and default value
	let mut file = match std::fs::File::create(&file_path) {
		Ok(a) => a,
		Err(e) => panic!("{:?}", e),
	};
	let data = "FALSE";
	if let Err(e) = file.write(data.as_bytes()) {
		panic!("{:?}", e);
	}
	file_data = data.into(); // str -> Cow<str>
];
println!("{:?}", file_data);
```

# License

Copyright 2019 #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License

Licensed under the Apache License, Version 2.0

