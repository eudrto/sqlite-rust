fn is_high_order_bit_set(byte: u8) -> bool {
    (byte as i8) < 0
}

fn clear_high_order_bit(byte: u8) -> u8 {
    let mask = !(1 << 7);
    byte & mask
}

fn add(num: i64, byte: u8) -> i64 {
    let cleared = clear_high_order_bit(byte);
    (num << 7) | cleared as i64
}

pub fn parse_varint(encoded: &mut &[u8]) -> i64 {
    let mut decoded: i64 = 0;

    for _ in 0..9 {
        if encoded.len() == 0 {
            panic!("invalid varint");
        }

        let byte = encoded[0];
        *encoded = &encoded[1..];

        decoded = add(decoded, byte);

        if !is_high_order_bit_set(byte) {
            break;
        }
    }

    return decoded;
}

pub fn parse_varints(window: &[u8]) -> Vec<i64> {
    let mut window = window;
    let window = &mut window;

    let mut decoded_varints = vec![];
    while window.len() > 0 {
        let decoded = parse_varint(window);
        decoded_varints.push(decoded);
    }
    decoded_varints
}
