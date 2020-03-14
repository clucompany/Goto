#![feature(test)]
//Copyright (c) 2020 #UlinProject Denis Kotlyarov (Денис Котляров)

//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

// #Ulin Project 20

/*!
Safe and convenient zero-cost 'goto' implementations for RustLang.

# Attention!!
1. We do not plan to violate Rust's safety standards. We provide useful and interesting macros that partially (or fully) implement the goto operator.

# gtree:
Safe "goto" tree with branches of executable code.

The main branch is a branch with a single root name, it is from the main branch that code execution begins.

It is allowed to descend to the desired branch or climb the branch.

// Gtree does not use functions and is strictly on the same stack
// Technologies used are also used in gblock.

It is assumed that "gtree" will be used in parsers or other methods of logical actions.

If there is no transition to another branch in the code of the branch, it is assumed that "gtree" should complete its work.

```rust
#[macro_use]
extern crate goto;
use std::collections::HashMap;

fn main() {
	let data = "BOOT_IMAGE=/boot/vmlinuz-linux-zen root=UUID=xxxx rw quiet";
	let mut hash = HashMap::new();
	
	gtree! {
		#let mut buffer = Vec::with_capacity(50); // local variables
		#let mut iter = data.as_bytes().iter();
		#let mut a;
		
		'decode_symbols {  // root tree
			//write name
			loop {
				a = iter.next();
				match a {
					Some(b'=') => {
						let name = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
						buffer.clear();
						
						// write value
						loop {
							a = iter.next();
							match a {
								Some(b' ') => run_gtree!('two_value: name, {
									let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
									buffer.clear();
									data
								}),
								Some(a) => buffer.push(*a),
								None => match buffer.is_empty() { // name
									true => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
									false => run_gtree!('end_two_value: name, unsafe { std::string::String::from_utf8_unchecked(buffer) }),
								}
							}
						}
						
					},
					Some(b' ') => match buffer.is_empty() {
						false => run_gtree!('one_value: {
							let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
							buffer.clear();
							data
						}),
						true => continue,
					},
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() {
						true => break 'decode_symbols, // end
						false => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
					}
				}
			}
		};

		// ONE_VALUE tree end_one_value
		'decode_symbols => 'end_one_value(name: String) :clone_anewrun('one_value -> 'decode_symbols) {
			hash.insert(name, None);
		};

		// TWO_VALUE tree end_one_value
		'decode_symbols => 'end_two_value(name: String, value: String) :clone_anewrun('two_value -> 'decode_symbols) {
			hash.insert(name, Some(value));
		};
	}
	
	assert_eq!(
		hash,		
		{
			let mut check_hash = HashMap::new();
			check_hash.insert("root".to_string(), Some("UUID=xxxx".to_string()));
			check_hash.insert("BOOT_IMAGE".to_string(), Some("/boot/vmlinuz-linux-zen".to_string()));
			
			check_hash.insert("rw".to_string(), None);
			check_hash.insert("quiet".to_string(), None);
			check_hash
		},
	);
}
```

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

Copyright 2020 #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License

Licensed under the Apache License, Version 2.0

*/

//#![no_std]

#[macro_use]
mod gpoint;

#[macro_use]
mod gblock;

#[macro_use]
mod gtree;
