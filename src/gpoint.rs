

/// "GOTO point", allows you to return to this line later.
#[macro_export]
#[deprecated = "No action is required from you, just rename “gpoint” to “gblock”, this is done for greater correctness."]
macro_rules! gpoint {
	[ $($all:tt)* ] => {
		$crate::gblock! {
			$($all)*
		}
	};
}

