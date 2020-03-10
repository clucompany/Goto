

#[macro_export]
macro_rules! anew_gtree {
	[ $n:tt: $($tt:tt)* ] => {
		continue $n {$($tt)+};
	};
	[ $n:tt ] => {
		continue $n;
	};
}


#[macro_export]
macro_rules! run_gtree {
	[ $n:tt: $($ident:expr),*] => {
		break $n ( $($ident),* );
	};
	[ $n:tt: $($tt:tt)+ ] => {
		break $n {$($tt)+};
	};
	[ $n:tt ] => {
		break $n;
	};
}

#[macro_export]
macro_rules! gtree {
	//$(#let $($data:tt)* ;)* 
	[ $($t:lifetime $(=> $t2:lifetime)? $(($($args:tt)*))? $(: {$($b:tt)*})? $($b2:block)? ;)* ] => {
		$crate::__gtree! {
			[][	[][]	]
			@$({[$t $($t2)?][$($($args)*)?][$(: {$($b)*})? $($b2)?]})*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gtree {
	// end set root
	[ [$($root:tt)+][$($data:tt)*] @{[$t:lifetime][$($args:tt)*][$b:block]} $($all:tt)* ] => { // root_yes // 1
		compile_error!("Рут уже определен");
	};
	[ [][$($data:tt)*] @{[$t:lifetime][$($args:tt)+][$b:block]} $($all:tt)* ] => { // set root // 1
		compile_error!("Рут не имеет входных данных");
	};
	[ [][$($data:tt)*] @{[$t:lifetime][][$b:block]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[$t $b][$($data)*] @$($all)*
		}
	};
	// end set root
	
	// added data
	// : {'bbb}
	[ [$($root:tt)*][ [$($data:tt)*][$($data2:tt)*] ] @{[$t:lifetime $t2:lifetime][$($args:tt)*][: {$($b:tt)*}]} $($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($root)*][ [$($data)*] [$($data2)* {[$t $t2][$($args)*][$($b)*]}] ] @$($all)*
		}
	};
	// {block}
	[ [$($root:tt)*][ [$($data:tt)*][$($data2:tt)*] ] @{[$t:lifetime $t2:lifetime][$($args:tt)*][$b: block]} $($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($root)*][ [$($data)* {[$t $t2][$($args)*][$b]}] [$($data2)*] ] @$($all)*
		}
	};
	
	// checker
	[ [][$($data:tt)+] @] => { //end, root
		compile_error!("Требуется определить рут");
	};

	// end, runtime
	[ [$r:tt $rb:block][ [$({[$t1:lifetime $t2:lifetime][$($args:tt)*][$b:block]})*] [$({[$nt1:lifetime $nt2:lifetime][$($nargs:tt)*][$($nb:tt)*]})*] ] @] => { //end
		// check 'root
		$(
			$crate::tt_equals! {
				if $r != $t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($t1), "\""));
				}
			}
		)*
		$(
			$crate::tt_equals! {
				if $r != $nt1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($nt1), "\""));
				}
			}
		)*
		//
		
		#[allow(unreachable_code)] {
			$r: loop {
				$crate::create_looper! { // no dop
					[$r $rb] @ [$({ [$t2][$($args)*][$b] })*] [$({ [$nt2][$($nargs)*][$($nb)*] })*]
				}
				
				break $r;
			}
		}

	};
}



#[doc(hidden)]
#[macro_export]
macro_rules! create_looper_type {
	[ $ty:ty ] => {$ty};
	[] => {_};
}
	
#[doc(hidden)]
#[macro_export]
macro_rules! create_looper {
	[ [$r:tt $rb:block]@ [] [] ] => {
		$rb
	};
	
	
	// args
	[ [$r:tt $rb:block]@ [{ [$tname:lifetime][$($args:ident $(:$aty:ty)? ),+][$tblock:block] } $($all_tt:tt)*] [$($all2_tt:tt)*] ] => {
		let ($($args),*) : ( $($crate::create_looper_type![ $($aty)? ]),* ) = $tname: loop {
			$crate::create_looper! {
				[$r $rb]@ [$($all_tt)*] [$($all2_tt)*]
			}

			break $r;
		};
		$tblock
	};

	[ [$r:tt $rb:block]@ [] [{ [$tname:lifetime][$($args:ident $(:$aty:ty)?),+][$($ttblock:tt)*] } $($all2_tt:tt)*] ] => {
		let ($($args),*) : ( $($crate::create_looper_type![ $($aty)? ]),* ) = $tname: loop {
			$crate::create_looper! {
				[$r $rb]@ [] [$($all2_tt)*]
			}
			
			break $r;
		};
		// decode_new_table
		$crate::gtree! {
			$($ttblock)*
		}
	};
	//
	
	// no args
	[ [$r:tt $rb:block]@ [{ [$tname:lifetime][][$tblock:block] } $($all_tt:tt)*] [$($all2_tt:tt)*] ] => {
		$tname: loop {
			$crate::create_looper! {
				[$r $rb]@ [$($all_tt)*] [$($all2_tt)*]
			}

			break $r;
		}
		$tblock
	};

	[ [$r:tt $rb:block]@ [] [{ [$tname:lifetime][][$($ttblock:tt)*] } $($all2_tt:tt)*] ] => {
		$tname: loop {
			$crate::create_looper! {
				[$r $rb]@ [] [$($all2_tt)*]
			}
			
			break $r;
		}
		// decode_new_table
		$crate::gtree! {
			$($ttblock)*
		}
	};
	//
}

	

#[doc(hidden)]
#[macro_export]
macro_rules! tt_equals {
	// ==
	[ if $d1:tt == $d2:tt $tr: block $(else $fa: block)? ] => {{
		macro_rules! __hidden1 {
			[$d1 $d1: $t:block $(else $f:block)?] => {$t}; // a == b
			[$d1 $unk2:tt: $t:block $(else $f:block)?] => {$($f)?} // a != b
		}
		__hidden1! ($d1 $d2: $tr $(else $fa)?)
	}};
	// end ==
	
	// !=
	[ if $d1:tt != $d2:tt $tr: block $(else $fa:block)?] => {
		//$crate::tt_equals! { $d1 == $d2 {} else $tr }
		//$crate::tt_equals! { $d1 == $d2 $fa else $tr }
		macro_rules! __hidden2 {
			[$d1 $d1: $t:block] => {}; // a == b
			[$d1 $unk2:tt: $t:block] => {$t}; // a != b
			
			[$d1 $d1: $t:block else $f:block] => {$f};// a == b else
			[$d1 $unk2:tt: $t:block else $f:block] => {$t} // a != b
		}
		__hidden2! ($d1 $d2: $tr $(else $fa)? )
	};
	// end !=
}