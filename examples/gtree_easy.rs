
#[macro_use]
extern crate goto;


fn main() {
	let data = "BOOT_IMAGE=/boot/vmlinuz-linux-zen root=UUID=xxxx rw quiet resume=/dev/sda2 nowatchdog clocksource=acpi_pm acpi_enforce_resources=lax audit=0 nopti nospectre_v2 l1tf=off no_stf_barrier nospec_store_bypass_disable noibrs noibpb";
	
	gtree! {
		#let mut buffer = Vec::with_capacity(50);
		#let mut iter = data.as_bytes().iter();
		#let mut a;
		
		'decode_symbols {
			//write name
			loop {
				a = iter.next();
				match a {
					Some(b'=') => {
						// write name
						let name = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
						buffer.clear();
						loop {
							a = iter.next();
							match a {
								Some(b' ') => run_gtree!('two_value: name, {
									let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
									buffer.clear();
									data
								}),
								Some(a) => buffer.push(*a),
								None => match buffer.is_empty() { // name
									true => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
									false => run_gtree!('end_two_value: name, unsafe { std::string::String::from_utf8_unchecked(buffer) }),
								}
							}
						}
						
					}, // WRITE VALUE
					Some(b' ') => match buffer.is_empty() {
						false => run_gtree!('one_value: {
							let data = unsafe { std::string::String::from_utf8_unchecked(buffer.to_owned()) };
							buffer.clear();
							data
						}),
						true => continue,
					},
					Some(a) => buffer.push(*a),
					None => match buffer.is_empty() {
						true => break 'decode_symbols, // end
						false => run_gtree!('end_one_value: unsafe { std::string::String::from_utf8_unchecked(buffer) }),
					}
				}
			}
		};

		// ONE_VALUE
		'decode_symbols => 'end_one_value(name: String) :clone_anew('one_value -> 'decode_symbols) {
			println!("#1 {}", name);
		};

		// TWO_VALUE
		'decode_symbols => 'end_two_value(name: String, value: String) :clone_anew('two_value -> 'decode_symbols) {
			println!("#2 {}={}", name, value);
		};
	}
}