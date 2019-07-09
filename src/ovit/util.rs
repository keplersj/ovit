use std::convert::TryInto;

pub fn correct_byte_order(raw_buffer: &[u8], is_byte_swapped: bool) -> Vec<u8> {
    raw_buffer
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes(chunk[0..2].try_into().unwrap()))
        .map(|byte| {
            if is_byte_swapped {
                byte
            } else {
                byte.swap_bytes()
            }
        })
        .flat_map(|byte| -> Vec<u8> { byte.to_ne_bytes().to_vec() })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_byte_order_if_byte_swap_true() {
        let bytes: [u8; 2] = [0x92, 0x14];
        let corrected_bytes = correct_byte_order(&bytes, true);

        assert_eq!(corrected_bytes, [0x14, 0x92]);
    }

    #[test]
    fn test_correct_byte_order_if_byte_swap_false() {
        let bytes: [u8; 2] = [0x14, 0x92];
        let corrected_bytes = correct_byte_order(&bytes, false);

        assert_eq!(corrected_bytes, [0x14, 0x92]);
    }
}
