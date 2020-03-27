
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
	[$($tt:tt)*] => {
		$crate::gtree_doc_hidden! {
			$($tt)*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! gtree_doc_hidden {
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $($rname:expr)? ]
		})? // hidden settings
		$(
			$rl:lifetime $(=> $l:lifetime)? $(($($args:tt)*))? // name function
			
			$(: $name:tt $(( $($dop_data:tt)* ))? )? // pre_fn
			
			{$($data:tt)*} // fn
		;)*
	] => {
		$crate::__gtree! {
			[ [$( $($breakb)? )?]		[$( $($rname)? )?] ] // config
			[ [$( $($start_lets)* )?]	[$( $($end_lets)* )?]  ] // include code
			[ root[]	data[] end_data[] tree[]	] // result_data
			@$({
				[$rl $($l)?][$($($args)*)?][$(:$name $(( $($dop_data)* ))? )? {
					$($data)*
				}]
			})*
		}
	};
	
	
	// LET
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $($rname:expr)? ]
		})?
		
		#let $ident:ident $(: $ty:ty)? $(= $e:expr)?;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@ {
				inc [
					[
						$($($start_lets)*)?
						let $ident $(: $ty)? $(= $e)?;
					]
					[
						$($($end_lets)*)?
					]
				]
				break [$($($breakb)?)?]
				result [$($($rname)?)?]
			}
			$($all)*
		}
	};
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $($rname:expr)? ]
		})?
		
		#let mut $ident:ident $(: $ty:ty)? $(= $e:expr)?;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@ {
				inc [
					[
						$($($start_lets)*)?
						let mut $ident $(: $ty)? $(= $e)?;
					]
					[
						$($($end_lets)*)?
					]
				]
				break [$($($breakb)?)?]
				result [$($($rname)?)?]
			}
			$($all)*
		}
	};
	// END LET
	
	// BREAK
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ ]
			result	[ ]
		})?
		
		#break $rl:lifetime $re:expr;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@ {
				inc [
					[$($($start_lets)*)?] [$($($end_lets)*)?]
				]
				break [$rl]
				result [$re]
			}
			$($all)*
		}
	};
	[
		@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $breakb:lifetime ]
			result	[ $($rname:expr)? ]
		}
		#break $rl:lifetime;
		$($all:tt)*
	] => {
		compile_error!("Нельзя переопределить break.")
	};
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $($rname:expr)? ]
		})?
		
		#break $rl:lifetime;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@ {
				inc [
					[$($($start_lets)*)?] [$($($end_lets)*)?]
				]
				break [$rl]
				result [$($($rname)?)?]
			}
			$($all)*
		}
	};
	// END BREAK
	
	// RESULT
	[
		@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $rname:expr ]
		}
		#result let mut $rnamee:ident $(: $rty:ty)? $(= $rexpr:expr)?;
		$($all:tt)*
	] => {
		compile_error!("Нельзя переопределить result.")
	};
	[
		$(@ {
			inc		[ [$($start_lets:tt)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[  ]
		})?
		
		#result let mut $rnamee:ident $(: $rty:ty)? $(= $rexpr:expr)?;
		$($all:tt)*
	] => {
		$crate::gtree! {
			@ {
				inc [
					[$($($start_lets)*)?] 
					[
						$($($end_lets)*)?
						let mut $rnamee $(: $rty)? $(= $rexpr)?;
					]
				]
				break [$($($breakb)?)?]
				result [$rnamee]
			}
			$($all)*
		}
	};
	//
	[
		$(@ {
			inc		[ [$($start_lets:block)*][$($end_lets:tt)*] ]
			break	[ $($breakb:lifetime)? ]
			result	[ $($rname:expr)? ]
		})?
	] => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gtree {
	// end set root
	[ [$($config:tt)*][$($inc:tt)*][ root[$($root:tt)+] $($all_d:tt)*] @{[$t:lifetime][$($args:tt)*][$($data:tt)*]} $($all:tt)* ] => { // root_yes // 1
		compile_error!("Рут уже определен");
	};
	[ [$($config:tt)*][$($inc:tt)*][ $($all_d:tt)*] @{[$t:lifetime][$($args:tt)+][$($data:tt)*]} $($all:tt)* ] => { // set root // 1
		compile_error!("Рут не может иметь входных данных");
	};
	//
	
	
	// SET BREAK LIFETIME AND SET ROOT +:loop
	[ [[$break:lifetime] $($all_c:tt)*][$($inc:tt)*][ root[] $($all_d:tt)*] @{[$rl:lifetime][][:loop {$($data:tt)*}]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[[$break] $($all_c)*][$($inc)*]
			[ root[$rl loop{loop{$($data)*}}] $($all_d)*] @$($all)*
		}
	};
	[ [[] $($all_c:tt)*][$($inc:tt)*][ root[] $($all_d:tt)*] @{[$rl:lifetime][][:loop {$($data:tt)*}]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[[$rl] $($all_c)*][$($inc)*]
			[ root[$rl loop{loop{$($data)*}}] $($all_d)*] @$($all)*
		}
	};
	//
	// SET BREAK LIFETIME AND SET ROOT
	[ [[$break:lifetime] $($all_c:tt)*][$($inc:tt)*][ root[] $($all_d:tt)*] @{[$rl:lifetime][][$($data:tt)*]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[[$break] $($all_c)*][$($inc)*]
			[ root[$rl $($data)*] $($all_d)*] @$($all)*
		}
	};
	[ [[] $($all_c:tt)*][$($inc:tt)*][ root[] $($all_d:tt)*] @{[$rl:lifetime][][$($data:tt)*]} $($all:tt)* ] => { // set root // 1
		$crate::__gtree! {
			[[$rl] $($all_c)*][$($inc)*]
			[ root[$rl $($data)*] $($all_d)*] @$($all)*
		}
	};
	
	
	// end set root
	
	// added data
	// :tree {'bbb}
	[ 
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($tree:tt)*]
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: tree {$($block:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*]
			[$($inc)*]
			[
				root[$($root)*]
				data[$($data)*]
				end_data[$($end_data)*]
				tree[{[$rl $l][$($args)*][$($block)*]} $($tree)*]
			] @$($all)*
		}
	};
	
	// :clone_anewrun($l: lifetime -> $r: lifetime (ttt))
	[
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($tree:tt)*]
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: clone_anewrun ($l_trans:lifetime -> $r_trans:lifetime) {$($block:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*]
			[$($inc)*]
			[
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($block)*]}  $($data)* ]
				end_data[{[$rl $l_trans][$($args)*][$($block)* continue $r_trans;]}  $($end_data)*	 ] 
				tree[$($tree)*]
			] @$($all)*
		}
	};
	
	// :loop
	[
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($tree:tt)*]
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: loop {$($block:tt)*}]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*]
			[$($inc)*]
			[
				root[$($root)*]
				data[
					{[$rl $l][$($args)*][
						loop {
							loop {$($block)*}
						} 
					]} $($data)* 
				]
				end_data[$($end_data)*] 
				tree[$($tree)*]
			] @$($all)*
		}
	};
	
	// :clone_run($l: lifetime -> $r: lifetime (ttt))
	[ 
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($tree:tt)*]
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: clone_run ($l_trans:lifetime -> $r_trans:lifetime $(( $($r_args:tt)* ))?) {$($block:tt)*}]} 
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*]
			[$($inc)*]
			[
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($b)*]} $($data)* ]
				end_data[{[$rl $l_trans][$($args)*][$($block)* break $r_trans $(( $($r_args)* ))?;]} $($end_data)* ]
				tree[$($tree)*]
			] @$($all)*
		}
	};
		
	// : unknown, WARNING!
	[ 	
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($data2:tt)*] 
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][: $uuu:tt $(($($uut:tt)*))? {$($block:tt)*}]} 
	$($all:tt)* ] => { //1
		compile_error!(concat!("Неопределенный тип \"", stringify!($uuu), "\""))
	};
	// {block}
	[
		[$($config:tt)*]
		[$($inc:tt)*]
		[
			root[$($root:tt)*]
			data[$($data:tt)*]
			end_data[$($end_data:tt)*]
			tree[$($tree:tt)*] 
		]
		@{[$rl:lifetime $l:lifetime][$($args:tt)*][$($block: tt)*]}
	$($all:tt)* ] => { //1
		$crate::__gtree! {
			[$($config)*]
			[$($inc)*]
			[
				root[$($root)*]
				data[{[$rl $l][$($args)*][$($block)*]} $($data)*]
				end_data[$($end_data)*]
				tree[$($tree)*]
			] @$($all)*
		}
	};
	
	// checker
	[ [$($config:tt)*][$($inc:tt)*][ root[] $($data:tt)*] @] => { //end, root
		compile_error!("Требуется определить рут");
	};

	// end, runtime
	[
		[ [$break:lifetime]  [$( $r_name:expr )?] ] // config
		[ [$($left_inc:tt)*] [$($right_inc:tt)*] ] // include code
		[ 
			root[	$rl:lifetime $($rb:tt)*	] 
			data[	$({[$t1:lifetime $t2:lifetime][$($args:tt)*][$($b:tt)*]})*	]
			end_data[	$({[$end_t1:lifetime $end_t2:lifetime][$($end_args:tt)*][$($end_b:tt)*]})*	]
			tree[	$({[$nt1:lifetime $nt2:lifetime][$($nargs:tt)*][$($tree_b:tt)*]})*	] 
		]
	@] => {{ //end
		
		// check 'root
		$(
			$crate::tt_equals! {
				if $rl != $t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($t1), "\""));
				}
			};
		)*
		$(
			$crate::tt_equals! {
				if $rl != $end_t1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($end_t1), "\""));
				}
			};
		)*
		$(
			$crate::tt_equals! {
				if $rl != $nt1 {
					compile_error!(concat!("Неопределенный корень \"", stringify!($nt1), "\""));
				}
			};
		)*
		//
		#[allow(unreachable_code)] {
			$($left_inc)*
			$($right_inc)*
			
			$rl: loop {
				$crate::create_looper! { // no dop
					[$break $($r_name)?][$rl $($rb)*] @
					
					start[	$({ [$t2][$($args)*][$($b)*] })*			]
					end[		$({ [$end_t2][$($end_args)*][$($end_b)*] })*	]
					tree[	$({ [$nt2][$($nargs)*][$($tree_b)*] })* 	]
				}
				
				break $break $($r_name)?;
				
				//
				$crate::tt_equals! {
					if $rl != $break {
						break $rl $($r_name)?; // ignore warning unused_labels
						//$($result_name)?
					}
				};
			}
		}
		//}
	}};
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
	[ [$break:lifetime $($break_i:expr)?][$r:lifetime $($root_block:tt)*] // end
		@ start[] end[] tree[]
	] => {
		#[deny(unreachable_code)] {
			$($root_block)*
		}
	};
	
	// next line
	[ [$break:lifetime $($break_i:expr)?][$($root:tt)*]
		@ start[] end[$($end:tt)+] tree[$($tree:tt)*]
	] => {
		$crate::create_looper! {
			[$break $($break_i)?][$($root)*] @ start[$($end)+] end[] tree[$($tree)*]
		}
	};
	
	
	// decode tree
	[ [$break:lifetime $($break_i:expr)?][$($root:tt)*]
		@ start[] end[] 
		tree[ { [$tname:lifetime][$($args:tt)*][$($tblock:tt)*] } $($tree:tt)*]
		// $({[$nt1:lifetime $nt2:lifetime][$($nargs:tt)*][$($tree_b:tt)*]})*
	] => {
		$tname: loop {
			$crate::create_looper! {
				[$break $($break_i)?][$($root)*] @ start[]
					end[] tree[$($tree)*]
			}
			break $break $($break_i)?;
		}
		#[deny(unreachable_code)] {
			$crate::gtree! {
				#break $break $($break_i)?;
				$($tblock)*
			};
		}
	};
	
	// def decode, args
	[ [$break:lifetime $($break_i:expr)?][$r:lifetime $($root_block:tt)*]
		@ start[{ [$tname:lifetime][$($args:ident $(:$aty:ty)? ),+][$($tblock:tt)*] } $($start:tt)*] 
		end[$($end:tt)*] tree[$($tree:tt)*]
	] => {{
		let ($($args),*) : ( $($crate::create_looper_type![ $($aty)? ]),* ) = $tname: loop {
			$crate::create_looper! {
				[$break $($break_i)?][$r $($root_block)*]@ start[$($start)*]
					end[$($end)*] tree[$($tree)*]
			}

			break $break $($break_i)?;
		};
		#[deny(unreachable_code)] {
			$($tblock)*
		}
	}};
	//
	
	// def decode, no args
	[ [$break:lifetime $($break_i:expr)?][$r:lifetime $($root_block:tt)*]
		@ start[{ [$tname:lifetime][][$($tblock:tt)*] } $($start:tt)*] 
		end[$($end:tt)*] tree[$($tree:tt)*]
	] => {
		$tname: loop {
			$crate::create_looper! {
				[$break $($break_i)?][$r $($root_block)*]@ start[$($start)*] 
					end[$($end)*] tree[$($tree)*]
			}

			break $break $($break_i)?;
		}
		#[deny(unreachable_code)] {
			$($tblock)*
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
			[$d1 $unk2:tt: $t:block $(else $f:block)?] => {$($f)?}; // a != b
		}
		__hidden1! ($d1 $d2: $tr $(else $fa)?);
	}};
	// end ==
	
	// !=
	[ if $d1:tt != $d2:tt $tr: block $(else $fa:block)?] => {{
		//$crate::tt_equals! { $d1 == $d2 {} else $tr }
		//$crate::tt_equals! { $d1 == $d2 $fa else $tr }
		macro_rules! __hidden2 {
			[$d1 $d1: $t:block] => {}; // a == b
			[$d1 $unk2:tt: $t:block] => {$t}; // a != b
			
			[$d1 $d1: $t:block else $f:block] => {$f};// a == b else
			[$d1 $unk2:tt: $t:block else $f:block] => {$t}; // a != b
		}
		__hidden2! ($d1 $d2: $tr $(else $fa)? );
	}};
	// end !=
}