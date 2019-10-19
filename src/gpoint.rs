

/// "GOTO point", allows you to return to this line later.
#[macro_export]
macro_rules! gpoint {
	//three
	[ $n:tt + $n2:tt + $n3:tt $e:tt: $($tt: tt)* ] => {{
		$n: loop {
			$n2: loop {
				$n3: loop {
					$($tt)*
				
					#[allow(dead_code)]
					#[allow(unreachable_code)]
					break $n $e
				}
				
				#[allow(dead_code)]
				#[allow(unreachable_code)]
				break $n $e
			}
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n $e
		}
	}};
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
	
	//two
	[ $n:tt + $n2:tt $e:tt: $($tt: tt)* ] => {{
		$n: loop {
			$n2: loop {
				$($tt)*
				
				#[allow(dead_code)]
				#[allow(unreachable_code)]
				break $n $e
			}
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n $e
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
	
	//one
	[ $n:tt $e:tt: $($tt:tt)* ] => {{
		$n: loop {
			$($tt)*
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n $e
		}
	}};
	
	[ $n:tt : $($tt:tt)* ] => {{
		$n: loop {
			$($tt)*
			
			#[allow(dead_code)]
			#[allow(unreachable_code)]
			break $n
		}
	}};
}
