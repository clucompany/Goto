
#[macro_export]
macro_rules! anew_gtree {
	[ $n:tt: $($e:expr),*] => {
		continue $n ( $($e),* );
	};
	[ $n:tt: $($tt:tt)* ] => {
		continue $n {$($tt)+};
	};
	[ $n:tt ] => {
		continue $n;
	};
}


#[macro_export]
macro_rules! run_gtree {
	[ $n:tt: $($e:expr),*] => {
		break $n ( $($e),* );
	};
	[ $n:tt: $($tt:tt)* ] => {
		break $n {$($tt)+};
	};
	[ $n:tt ] => {
		break $n;
	};
}

#[macro_export]
macro_rules! gtree {
	[
		$(@hidden_inc [ $($include_lets:tt)* ])?
		$(@hidden_break $break:lifetime)?

		$(	
			$rl:lifetime $(=> $l:lifetime)? $(($($args:tt)*))? // name function
			
			$(: $name:tt $(( $($dop_data:tt)* ))? )? // pre_fn
			
			{$($b:tt)*} // fn
		;)*
	] => {
		$crate::__gtree! {
			[$($break)? ][ root[]	data[] end_data[] tree[]	] [$( $($include_lets)* )?]
			@$({
				[$rl $($l)?][$($($args)*)?][$(:$name $(( $($dop_data)* ))? )? {$($b)*}]
			})*
		}
	};
	
	
	// LET
	[
		$(@hidden_inc [ $($include_lets:tt)* ])?
		$(@hidden_break $break:lifetime)?
		
		#let $ident:ident $(: $ty:ty)? $(= $expr:expr)?;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@hidden_inc [
				$($($include_lets)*)?
				let $ident $(: $ty)? $(= $expr)?;
			]
			$(@hidden_break $break)?
			
			$($all)*
		}
	};
	[
		$(@hidden_inc [ $($include_lets:tt)* ])?
		$(@hidden_break $break:lifetime)?
		
		#let mut $ident:ident $(: $ty:ty)? $(= $expr:expr)?;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@hidden_inc [
				$($($include_lets)*)?
				let mut $ident $(: $ty)? $(= $expr)?;
			]
			$(@hidden_break $break)?
			$($all)*
		}
	};
	// END LET
	
	// BREAK
	[
		$(@hidden_inc [ $($include_lets:tt)* ])?
		@hidden_break $break:lifetime
		
		#break $rl:lifetime;
		$($all:tt)*
	] => {
		compile_error!("Нельзя переопределить break.")
	};
	[
		$(@hidden_inc [ $($include_lets:tt)* ])?
		//$(@hidden_break $l:lifetime)?
		
		#break $rl:lifetime;
		$($all:tt)*
	] => {
		$crate::gtree! {
			$(@hidden_inc [$($include_lets)*])?
			@hidden_break $rl
			
			$($all)*
		}
	};
	// END BREAK
	
	[
		$(@hidden_inc [ $($include_lets:tt)* ])? 
		$(@hidden_break $break:lifetime)?
	] => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gtree {
	// end set root
	[ [$($config:tt)*][ root[$($root:tt)+] $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][$($args:tt)*][$($block:tt)*]} $($all:tt)* ] => { // root_yes // 1
		compile_error!("Рут уже определен");
	};
	[ [$($config:tt)*][ $($data:tt)*][$($inc:tt)*] @{[$t:lifetime][$($args:tt)+][$($block:tt)*]} $($all:tt)* ] => { // set root // 1
		compile_error!("Рут не может иметь входных данных");
	};
	//
	
	// SET BREAK LIFETIME AND SET ROOT
	[ [$break:lifetime][ root[] $($data:tt)*][$($inc:tt)*] @{[$rl:lifetime][][$($block:tt)*]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[$break]	[ root[$rl $($block)*] $($data)*][$($inc)*] @$($all)*
		}
	};
	[ [][ root[] $($data:tt)*][$($inc:tt)*] @{[$rl:lifetime][][$($block:tt)*]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[$rl]	[ root[$rl $($block)*] $($data)*][$($inc)*] @$($all)*
		}
	};
	//
	
	// end set root
	
	// added data
	// :tree {'bbb}
	[ [$($config:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end_data:tt)*]
		tree[$($tree:tt)*]
	][$($inc:tt)*] 
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: tree {$($block:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*][
				root[$($root)*]
				data[$($data)*]
				end_data[$($end_data)*]
				tree[{[$rl $l][$($args)*][$($block)*]} $($tree)*]
			][$($inc)*] @$($all)*
		}
	};
	
	// :clone_anewrun($l: lifetime -> $r: lifetime (ttt))
	[ [$($config:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end_data:tt)*]
		tree[$($tree:tt)*]
	][$($inc:tt)*]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: clone_anewrun ($l_trans:lifetime -> $r_trans:lifetime) {$($block:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*][
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($block)*]}  $($data)* ]
				end_data[{[$rl $l_trans][$($args)*][$($block)* continue $r_trans;]}  $($end_data)*	 ] 
				tree[$($tree)*]
			][$($inc)*] @$($all)*
		}
	};
	
	// :clone_run($l: lifetime -> $r: lifetime (ttt))
	[ [$($config:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end_data:tt)*]
		tree[$($tree:tt)*]
	][$($inc:tt)*]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: clone_run ($l_trans:lifetime -> $r_trans:lifetime $(( $($r_args:tt)* ))?) {$($block:tt)*}]} 
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*][
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($b)*]} $($data)* ]
				end_data[{[$rl $l_trans][$($args)*][$($block)* break $r_trans $(( $($r_args)* ))?;]} $($end_data)* ]
				tree[$($tree)*]
			][$($inc)*] @$($all)*
		}
	};
		
	// : unknown, WARNING!
	[ [$($config:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end_data:tt)*]
		tree[$($data2:tt)*] ]
	[$($inc:tt)*] 
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: $uuu:tt $(($($uut:tt)*))? {$($block:tt)*}]} 
	$($all:tt)* ] => { //1
		compile_error!(concat!("Неопределенный тип \"", stringify!($uuu), "\""))
	};
	// {block}
	[ [$($config:tt)*][
		root[$($root:tt)*]
		data[$($data:tt)*]
		end_data[$($end_data:tt)*]
		tree[$($tree:tt)*] ]
	[$($inc:tt)*] 
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][$($block: tt)*]} 
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*][
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($block)*]} $($data)*]
				end_data[$($end_data)*]
				tree[$($tree)*]
			][$($inc)*] @$($all)*
		}
	};
	
	// checker
	[ [$($config:tt)*][ root[] $($data:tt)+][$($inc:tt)*] @] => { //end, root
		compile_error!("Требуется определить рут");
	};

	// end, runtime
	[
		[$break:lifetime] // config
		[ 
			root[	$rl:lifetime $($rb:tt)*	] 
			data[	$({[$t1:lifetime $t2:lifetime][$($args:tt)*][$($b:tt)*]})*	]
			end_data[	$({[$end_t1:lifetime $end_t2:lifetime][$($end_args:tt)*][$($end_b:tt)*]})*	]
			tree[	$({[$nt1:lifetime $nt2:lifetime][$($nargs:tt)*][$($nb:tt)*]})*	] 
		]
		[$($inc:tt)*]
	@] => { //end
		
		// check 'root
		$(
			$crate::tt_equals! {
				if $rl != $t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($t1), "\""));
				}
			}
		)*
		$(
			$crate::tt_equals! {
				if $rl != $end_t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($end_t1), "\""));
				}
			}
		)*
		$(
			$crate::tt_equals! {
				if $rl != $nt1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($nt1), "\""));
				}
			}
		)*
		//
		
		#[allow(unreachable_code)] {
			$($inc)*
			$rl: loop {
				$crate::create_looper! { // no dop
					[$break][$rl $($rb)*] @
					
					start[	$({ [$t2][$($args)*][$($b)*] })*	]
					
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
				
				//
				$crate::tt_equals! {
					if $rl != $break {
						break $rl; // ignore warning unused_labels
					}
				}
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
	[ [$break:lifetime][$r:lifetime $($root_block:tt)*] // end
		@ start[] end[]
	] => {{
		$($root_block)*
	}};
	
	// next line
	[ [$break:lifetime][$($root:tt)*]
		@ start[] end[$($end:tt)+]
	] => {
		$crate::create_looper! {
			[$break][$($root)*] @ start[$($end)+] end[]
		}
	};
	
	// args
	[ [$break:lifetime][$r:lifetime $($root_block:tt)*]
		@ start[{ [$tname:lifetime][$($args:ident $(:$aty:ty)? ),+][$($tblock:tt)*] } $($start:tt)*] end[$($end:tt)*] 
	] => {{
		let ($($args),*) : ( $($crate::create_looper_type![ $($aty)? ]),* ) = $tname: loop {
			$crate::create_looper! {
				[$break][$r $($root_block)*]@ start[$($start)*] end[$($end)*]
			}

			break $break;
		};
		$($tblock)*
	}};
	//
	
	// no args
	[ [$break:lifetime][$r:lifetime $($root_block:tt)*]
		@ start[{ [$tname:lifetime][][$($tblock:tt)*] } $($start:tt)*] end[$($end:tt)*] 
	] => {
		$tname: loop {
			$crate::create_looper! {
				[$break][$r $($root_block)*]@ start[$($start)*] end[$($end)*]
			}

			break $break;
		}
		$($tblock)*
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