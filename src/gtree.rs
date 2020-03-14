
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
	
	[
		$(@hidden_inc [ $($data_args:tt)* ])?
		$(@hidden_break $break:lifetime)?

		$($t:lifetime $(=> $t2:lifetime)? $(($($args:tt)*))? // name function
		
		$(: $name:tt $(( $($dop_data:tt)* ))? {$($b:tt)*})? $($b2:block)? ;)* 
	] => {
		$crate::__gtree! {
			[$($break)? ][ root[]	data[] end_data[] tree[]	] [$( $($data_args)* )?]
			@$({[$t $($t2)?][$($($args)*)?][$(:$name $(( $($dop_data)* ))? {$($b)*})? $($b2)?]})*
		}
	};
	
	
	// LET
	[
		$(@hidden_inc [ $($data_args:tt)* ])?
		$(@hidden_break $break:lifetime)?
		
		#let $ident:ident $(: $ty:ty)? $(= $expr:expr)?;
		$($all_tt:tt)*
	] => {
		$crate::gtree! {
			@hidden_inc [
				$($($data_args)*)?
				let $ident $(: $ty)? $(= $expr)?;
			]
			$(@hidden_break $break)?
			
			$($all_tt)*
		}
	};
	[
		$(@hidden_inc [ $($data_args:tt)* ])?
		$(@hidden_break $break:lifetime)?
		
		#let mut $ident:ident $(: $ty:ty)? $(= $expr:expr)?;
		$($all_tt:tt)*
	] => {
		$crate::gtree! {
			@hidden_inc [
				$($($data_args)*)?
				let mut $ident $(: $ty)? $(= $expr)?;
			]
			$(@hidden_break $break)?
			$($all_tt)*
		}
	};
	// END LET
	
	// BREAK
	[
		$(@hidden_inc [ $($data_args:tt)* ])?
		@hidden_break $l:lifetime
		
		#break $rl:lifetime;
		$($all_tt:tt)*
	] => {
		compile_error!("Нельзя переопределить break.")
	};
	[
		$(@hidden_inc [ $($data_args:tt)* ])?
		//$(@hidden_break $l:lifetime)?
		
		#break $rl:lifetime;
		$($all_tt:tt)*
	] => {
		$crate::gtree! {
			$(@hidden_inc [$($data_args)*])?
			@hidden_break $rl
			
			$($all_tt)*
		}
	};
	// END BREAK
	
	[
		$(@hidden_inc [ $($data_args:tt)* ])? 
		$(@hidden_break $l:lifetime)?
	] => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gtree {
	// end set root
	[ [$($c:tt)*][ root[$($root:tt)+] $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][$($args:tt)*][$b:block]} $($all:tt)* ] => { // root_yes // 1
		compile_error!("Рут уже определен");
	};
	[ [$($c:tt)*][ $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][$($args:tt)+][$b:block]} $($all:tt)* ] => { // set root // 1
		compile_error!("Рут не может иметь входных данных");
	};
	//
	
	// SET BREAK LIFETIME AND SET ROOT
	[ [$break:lifetime][ root[] $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][][$b:block]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[$break][ root[$t $b] $($data)*][$($inc)*] @$($all)*
		}
	};
	[ [][ root[] $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][][$b:block]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[$t][ root[$t $b] $($data)*][$($inc)*] @$($all)*
		}
	};
	//
	
	// end set root
	
	// added data
	// :tree {'bbb}
	[ [$($c:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end:tt)*]
		tree[$($data2:tt)*]
	][$($inc:tt)*] 
		@{[$t:lifetime $t2:lifetime][$($args:tt)*][: tree {$($b:tt)*}]} 
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($c)*][
				root[$($root)*]
				data[$($data)*]
				end_data[$($end)*]
				tree[$($data2)* {[$t $t2][$($args)*][$($b)*]}]
			][$($inc)*] @$($all)*
		}
	};
	
	// :clone_anew($l: lifetime -> $r: lifetime (ttt))
	[ [$($c:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end:tt)*]
		tree[$($tree:tt)*] 
	][$($inc:tt)*]
		@{[$t:lifetime $t2:lifetime][$($args:tt)*][: clone_anew ($l_trans:lifetime -> $r_trans:lifetime) {$($b:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($c)*][ 
				root[$($root)*] 
				data[$($data)*		{[$t $t2][$($args)*][$($b)*]} ] 
				end_data[$($end)*	{[$t $l_trans][$($args)*][$($b)* continue $r_trans;]} ] 
				tree[$($tree)*] 
			][$($inc)*] @$($all)*
		}
	};
	
	// :clone_run($l: lifetime -> $r: lifetime (ttt))
	[ [$($c:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end:tt)*]
		tree[$($tree:tt)*] 
	][$($inc:tt)*]
		@{[$t:lifetime $t2:lifetime][$($args:tt)*][: clone_run ($l_trans:lifetime -> $r_trans:lifetime $(( $($r_args:tt)* ))?) {$($b:tt)*}]} 
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($c)*][ 
				root[$($root)*] 
				data[$($data)*		{[$t $t2][$($args)*][$($b)*]} ] 
				end_data[$($end)*	{[$t $l_trans][$($args)*][$($b)* break $r_trans $(( $($r_args)* ))?;]} ] 
				tree[$($tree)*] 
			][$($inc)*] @$($all)*
		}
	};
		
	// : unknown, WARNING!
	[ [$($c:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end:tt)*]
		tree[$($data2:tt)*] ]
	[$($inc:tt)*] 
		@{[$t:lifetime $t2:lifetime][$($args:tt)*][: $uuu:tt $(($($uut:tt)*))? {$($b:tt)*}]} 
	$($all:tt)* ] => { //1
		compile_error!(concat!("Неопределенный тип \"", stringify!($uuu), "\""))
	};
	// {block}
	[ [$($c:tt)*][ root[$($root:tt)*] data[$($data:tt)*] end_data[$($end:tt)*] tree[$($data2:tt)*] ][$($inc:tt)*] @{[$t:lifetime $t2:lifetime][$($args:tt)*][$($b: tt)*]} $($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($c)*][
				root[$($root)*] 
				data[$($data)*		{[$t $t2][$($args)*][$($b)*]}] 
				end_data[$($end)*] 
				tree[$($data2)*] 
			][$($inc)*] @$($all)*
		}
	};
	
	// checker
	[ [$($c:tt)*][ root[] $($data:tt)+][$($inc:tt)*] @] => { //end, root
		compile_error!("Требуется определить рут");
	};

	// end, runtime
	[
		[$break:lifetime]
		[ 
			root[	$r:lifetime $rb:block	] 
			data[	$({[$t1:lifetime $t2:lifetime][$($args:tt)*][$($b:tt)*]})*	]
			end_data[	$({[$end_t1:lifetime $end_t2:lifetime][$($end_args:tt)*][$($end_b:tt)*]})*	]
			tree[	$({[$nt1:lifetime $nt2:lifetime][$($nargs:tt)*][$($nb:tt)*]})*	] 
		]
		[$($inc:tt)*] 
	@] => { //end
		
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
				if $r != $end_t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($end_t1), "\""));
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
			$($inc)*
			$r: loop {
				$crate::create_looper! { // no dop
					[$break][$r $rb] @ 
					
					start[$({ [$t2][$($args)*][$($b)*] })*]
					
					end[
						$({ [$end_t2][$($end_args)*] // end
							[
								$($end_b)*
							]
						})*
						$({ [$nt2][$($nargs)*] // tree
							[
								$crate::gtree! {
									#break $break;
									$($nb)*
								}
							] 
						})*
					]
				}
				
				break $break;
				break $r; // ignore warning unused_labels
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
	[ [$break:lifetime][$r:lifetime $rb:block]@ start[] end[] ] => {{
		$rb
	}};
	
	// next line
	[ [$break:lifetime][$($root:tt)*]@ start[] end[$($all:tt)+] ] => {
		$crate::create_looper! {
			[$break][$($root)*] @ start[$($all)+] end[]
		}
	};
	
	// args
	[ [$break:lifetime][$r:lifetime $rb:block]@ start[{ [$tname:lifetime][$($args:ident $(:$aty:ty)? ),+][$($tblock:tt)*] } $($start_tt:tt)*] end[$($end_tt:tt)*] ] => {{
		let ($($args),*) : ( $($crate::create_looper_type![ $($aty)? ]),* ) = $tname: loop {
			$crate::create_looper! {
				[$break][$r $rb]@ start[$($start_tt)*] end[$($end_tt)*]
			}

			break $break;
		};
		$($tblock)*
		//$(	drop( $args );	)*
	}};
	//
	
	// no args
	[ [$break:lifetime][$r:lifetime $rb:block]@ start[{ [$tname:lifetime][][$($tblock:tt)*] } $($start_tt:tt)*] end[$($end_tt:tt)*] ] => {{
		$tname: loop {
			$crate::create_looper! {
				[$break][$r $rb]@ start[$($start_tt)*] end[$($end_tt)*]
			}

			break $break;
		}
		$($tblock)*
	}};
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