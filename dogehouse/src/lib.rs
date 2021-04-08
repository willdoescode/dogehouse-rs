#[macro_use]
extern crate dogehouse_macros;

#[cfg(test)]
mod tests {
	use super::*;

	#[show_streams]
	fn yo() {
		println!("Hello")
	}
}
