//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

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

*/

#![no_std]


#[macro_export]
macro_rules! gpoint {
	[ $n:tt + $n2:tt + $n3:tt : $($tt: tt)* ] => {{
		$n: loop {
			$n2: loop {
				$n3: loop {
					$($tt)*
					
					#[allow(dead_code)]
					#[allow(unreachable_code)]
					break $n
				}
				
				
				#[allow(dead_code)]
				#[allow(unreachable_code)]
				break $n
			}
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n
		}
	}};
	
	[ $n:tt + $n2:tt : $($tt: tt)* ] => {{
		$n: loop {
			$n2: loop {
				$($tt)*
				
				
				#[allow(dead_code)]
				#[allow(unreachable_code)]
				break $n
			}
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n
		}
	}};
	
	[ $name:tt : $($tt:tt)* ] => {{
		$name: loop {
			$($tt)*
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $name
		}
	}};
}

#[macro_export]
macro_rules! goto {
	[ $name:tt ] => {{
		continue $name
	}};
}
