use rust_decimal::Decimal;

fn main() {
	let fixture:Decimal=(114514.1919).try_into().unwrap();

	
	
	println!("{}",format(&fixture));
}

fn format(value:&Decimal)->String{
	let mut buff=std::collections::VecDeque::<u8>::new();
	let mut iter=value.to_string().into_bytes().into_iter().rev();
	
	if !value.is_integer(){
		for b in iter.by_ref(){
			buff.push_front(b);
			if b==b'.'{
				break
			}
		}
	}
	
	let mut cnt=0usize;
	
	for b in iter{
		if cnt==3{
			buff.push_front(b',');
			cnt=0;
		}
		
		buff.push_front(b);
		cnt+=1;
	}
	
	
	
	for b in value.to_string().bytes().rev(){
		buff.push_front(b);
	}
	
	String::from_utf8(buff.into()).unwrap()
}

