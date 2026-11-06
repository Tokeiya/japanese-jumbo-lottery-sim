#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

use super::formatter::ThousandsSeparator;
use rust_decimal::Decimal;
use std::fmt::Formatter;

impl ThousandsSeparator for Decimal {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut iter = self.to_string().into_bytes().into_iter().rev();
		let mut buff = std::collections::VecDeque::new();
		if !self.is_integer() {
			for b in iter.by_ref() {
				buff.push_front(b);

				if b == b'.' {
					break;
				}
			}
		}

		let mut cnt = 0usize;
		for b in iter {
			if cnt == 3 && b != b'-' {
				buff.push_front(b',');
				cnt = 0;
			}

			buff.push_front(b);
			cnt += 1;
		}

		let str = String::from_utf8(buff.into()).map_err(|x| std::fmt::Error)?;
		write!(f, "{str}")
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn positive_integer() {
		let fixture: Decimal = 114_514.into();
		let actual = format!("{}", fixture.separated());

		assert_eq!(actual, "114,514");
	}

	#[test]
	fn negative_integer() {
		let fixture: Decimal = (-114_514i32).try_into().unwrap();
		let actual = format!("{}", fixture.separated());

		assert_eq!(actual, "-114,514");
	}

	#[test]
	fn positive_float() {
		let fixture: Decimal = (114_514.1919).try_into().unwrap();
		let actual = format!("{}", fixture.separated());

		assert_eq!(actual, "114,514.1919");
	}

	#[test]
	fn negative_float() {
		let fixture: Decimal = (-114_514.1919).try_into().unwrap();
		let actual = format!("{}", fixture.separated());

		assert_eq!(actual, "-114,514.1919");
	}
}
