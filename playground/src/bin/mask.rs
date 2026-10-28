const NUMBER_MASK: u32 = 0x00_03_FF_FF;
const GROUP_MASK: u32 = 0x03_FC_00_00;

const GROUP_OFFSET: usize = 18;
const UNIT_OFFSET: usize = 26;

fn main() {
	const UNIT: u8 = 60;
	const GROUP: u8 = 152;
	const NUMBER: u32 = 195_436;

	let mut data = UNIT as u32;
	data <<= UNIT_OFFSET;
	data |= (GROUP as u32) << GROUP_OFFSET;
	data |= NUMBER;

	println!("0x{data:x}:{data}");

	println!("NUM:{}", data & NUMBER_MASK);
	println!("GRP:{}", (data & GROUP_MASK) >> GROUP_OFFSET);
	println!("UNIT:{}", (data >> UNIT_OFFSET))
}
