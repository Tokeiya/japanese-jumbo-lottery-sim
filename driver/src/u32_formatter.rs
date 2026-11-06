
use super::formatter::ThousandsSeparator;
use rust_decimal::Decimal;
use std::fmt::Formatter;


impl ThousandsSeparator for u32{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let iter=self.to_string().into_bytes().into_iter().rev();
		let mut buff=std::collections::VecDeque::new();
		let mut cnt=0usize;
		
		for b in iter{
			if cnt == 3 {
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
mod test{
	use crate::formatter::ThousandsSeparator;
	
	#[test]
	fn format() {
		let actual=format!("{}",114514u32.separated());
		assert_eq!("114,514",actual);
	}
}