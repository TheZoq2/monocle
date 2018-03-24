pub fn encode_u32(val: u32, buf: &mut [u8]) {
    buf[0] = (val >> 24) as u8;
    buf[1] = (val >> 16) as u8;
    buf[2] = (val >> 8) as u8;
    buf[3] = (val) as u8;
}
