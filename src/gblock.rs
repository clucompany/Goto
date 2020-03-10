
/// Move to the beginning of "gblock".
#[macro_export]
macro_rules! to_start_gblock {
	[ $n:tt: $($tt:tt)* ] => {
		continue $n {$($tt)+};
	};
	[ $n:tt ] => {
		continue $n;
	};
}


/// Move to the end of "gblock".
#[macro_export]
macro_rules! to_end_gblock {
	[ $n:tt: $($tt:tt)+ ] => {
		break $n {$($tt)+};
	};
	[ $n:tt ] => {
		break $n;
	};
}


/// A safe version of the "goto" prisoner in the block. Ability to move to the beginning of the block or to the end of the block.
#[macro_export]
macro_rules! gblock {
	[ $n:tt $(, $n2:tt)* $( ($($r:tt)*) )*: ] => {{
		//empty
	}};
	[ $n:tt $(, $n2:tt)* $( ($($r:tt)*) )*: $($tt:tt)* ] => {{
		$crate::__gblock_fn!{
			[$n][$n $($n2)*][ $({$($r)*})* ][$($tt)*]
		}
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __gblock_fn {
	[ [$root_n:tt][$n:tt $($n2:tt)+][ $({$($ret_tt:tt)*})?] [$($tt:tt)*]  ] => {
		#[allow(dead_code)]
		#[allow(unreachable_code)]
		#[allow(unused_labels)]
		$n: loop {
			$crate::__gblock_fn! {
				[$root_n][$($n2)+][$({$($ret_tt)*})?][$($tt)*]
			}
			
			break $root_n $({$($ret_tt)*})?;
		}
	};
	[ [$root_n:tt][$n:tt][$({$($ret_tt:tt)*})?][$($tt:tt)*]  ] => {
		#[allow(unused_labels)]
		#[allow(dead_code)]
		#[allow(unreachable_code)]
		$n: loop {
			$($tt)*
			
			break $root_n $({$($ret_tt)*})?;
		}
	};
}
