pub fn calculate_encoder(current_encoder_value: i64, motor_encoder: Vec<u32>) -> i64 {
    let bits: [u8; 8] = get_bits(motor_encoder[1] as u8);
    
    let delta: i64 = motor_encoder[0] as i32 as i64;
    let mut sum: i64 = current_encoder_value + delta;

    let underflow: bool = bits[0] != 0;
    let overflow: bool = bits[2] != 0;

    let range: i64 = u32::MAX as i64 + 1;
    sum += overflow as i64 * range;
    sum -= underflow as i64 * range;

    sum
}

pub fn get_bits(byte: u8) -> [u8; 8] {
    let mut bits: [u8; 8] = [0; 8];
    for i in 0..8 {
        bits[7 - i] = (byte >> i) & 1;
    }
    bits
}