/// First in source of the functions that share the slot `read` returns in.
pub fn lead(data: &[u8]) -> Result<&[u8], u8> {
	data.get(1..).ok_or(1)
}

pub fn read(data: &[u8]) -> Result<&[u8], u8> {
	data.get(..1).ok_or(0)
}
