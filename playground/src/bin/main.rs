pub enum Either {
	String(String),
	Integer(i64),
}

impl From<&str> for Either {
	fn from(value: &str) -> Self {
		Self::String(value.into())
	}
}

impl From<i64> for Either {
	fn from(value: i64) -> Self {
		Self::Integer(value)
	}
}

fn main() {
	let either = Either::from(10);

	match either {
		Either::String(s) => println!("String:{s}"),
		Either::Integer(i) => println!("Integer:{i}"),
	}

	let either = Either::from("hello world");
	match either {
		Either::String(s) => println!("String:{s}"),
		Either::Integer(i) => println!("Integer:{i}"),
	}
}
