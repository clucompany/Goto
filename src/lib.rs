//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

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

// #Ulin Project 1819

/*!
A safe but not complete implementation of the goto operator.

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

Licensed under the Apache License, Version 2.0

*/

#![no_std]

#[macro_use]
mod gpoint;
