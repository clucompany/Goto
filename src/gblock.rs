
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
	[ $name:lifetime $(, $alias:lifetime)* $( -> ($($breaker:tt)*) )*: ] => {
		//empty
	};
	[ $name:lifetime $(, $alias:lifetime)* $( -> ($($breaker:tt)*) )?: $($data:tt)* ] => {{
		$crate::__gblock_fn!{
			[$name][$name $($alias)*][ $({$($breaker)*})* ][$($data)*]
		}
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __gblock_fn {
	[ [$root:lifetime][$name:lifetime $($alias:lifetime)+][ $({$($breaker:tt)*})? ] [$($data:tt)*]  ] => {
		#[allow(dead_code)]
		#[allow(unreachable_code)]
		#[allow(unused_labels)]
		$name: loop {
			$crate::__gblock_fn! {
				[$root][$($alias)+][$({$($breaker)*})?][$($data)*]
			}
			
			break $root $({$($breaker)*})?;
		}
	};
	[ [$root:lifetime][$name:lifetime][ $({$($breaker:tt)*})? ][$($data:tt)*]  ] => {
		#[allow(unused_labels)]
		#[allow(dead_code)]
		#[allow(unreachable_code)]
		$name: loop {
			$($data)*
			
			break $root $({$($breaker)*})?;
		}
	};
}
