
#[macro_use]
extern crate goto;

fn main() {
	let data = "BOOT_IMAGE=/boot/vmlinuz-linux-zen root=UUID=xxxx rw quiet resume=xxxx nowatchdog clocksource=acpi_pm acpi_enforce_resources=lax audit=0 nopti nospectre_v2 l1tf=off no_stf_barrier nospec_store_bypass_disable noibrs noibpb";
	let mut iter = data.as_bytes().iter();
	let mut buffer = Vec::with_capacity(50);
	
	gblock!['root:
		let (name, value) = gblock!['combo_value (unimplemented!()):
			let name = gblock!['empty_value (unimplemented!()):
				'decode_name: loop {
					match iter.next() {
						Some(b' ') => to_end_gblock!('empty_value: match buffer.is_empty() {
							true => to_end_gblock!('root),
							_ => {
								let name = unsafe { String::from_utf8_unchecked(buffer.to_owned()) };
								buffer.clear();
								name
							}
						}),
						Some(b'=') => break 'decode_name,
						Some(a) => buffer.push(*a),
						_ => to_end_gblock!('empty_value: match buffer.is_empty() {
							true => to_end_gblock!('root),
							_ => {
								let name = unsafe { String::from_utf8_unchecked(buffer.to_owned()) };
								buffer.clear();
								name
							}
						}),
					}
				}
				let name = unsafe { String::from_utf8_unchecked(buffer.to_owned()) };
				buffer.clear();
				//no empty value
				'decode_value: loop {
					match iter.next() {
						Some(b' ') => to_end_gblock!('combo_value: {
							let value = unsafe { String::from_utf8_unchecked(buffer.to_owned()) };
							buffer.clear();
							(name, value)
						}),
						Some(a) => buffer.push(*a),
						_ => match buffer.is_empty() {
							true => to_end_gblock!('empty_value: name),
							false => to_end_gblock!('combo_value: {
								let value = unsafe { String::from_utf8_unchecked(buffer.to_owned()) };
								buffer.clear();
								(name, value)
							}),
						},
					}
				}
			];
			// empty value
			println!("{}", name);
			to_start_gblock!('root);
		];
		// combo value
		println!("{}={}", name, value);
		to_start_gblock!('root);
	];
}


