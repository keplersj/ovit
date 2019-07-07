use std::convert::TryInto;

use std::ops::RangeInclusive;

pub fn get_string_from_bytes_range(
    bytes: &[u8],
    range: RangeInclusive<usize>,
) -> Result<String, String> {
    match String::from_utf8(match bytes.get(range) {
        Some(vec) => vec.to_vec(),
        _ => {
            return Err("Could not get bytes".to_string());
        }
    }) {
        Ok(string) => Ok(string.to_string()),
        Err(err) => Err(format!("Could not convert bytes to string: {:#X?}", err)),
    }
}

pub fn get_u16_from_bytes_range(bytes: &[u8], range: RangeInclusive<usize>) -> Result<u16, String> {
    Ok(u16::from_be_bytes(
        match bytes.get(range) {
            Some(bytes) => bytes,
            _ => return Err("Could not get bytes from range".to_string()),
        }
        .try_into()
        .unwrap(),
    ))
}

pub fn get_u32_from_bytes_range(bytes: &[u8], range: RangeInclusive<usize>) -> Result<u32, String> {
    Ok(u32::from_be_bytes(
        match bytes.get(range) {
            Some(bytes) => bytes,
            _ => return Err("Could not get bytes from range".to_string()),
        }
        .try_into()
        .unwrap(),
    ))
}

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
    fn test_get_string_from_bytes_range() {
        let test_string = b"Hello world!";
        let string_from_bytes = get_string_from_bytes_range(test_string, 0..=4).unwrap();

        assert_eq!(string_from_bytes, "Hello");
    }

    #[test]
    fn test_get_u16_from_bytes_range() {
        let bytes: [u8; 6] = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45];
        let u16_from_bytes = get_u16_from_bytes_range(&bytes, 0..=1).unwrap();

        assert_eq!(u16_from_bytes, 0xABCD);
    }

    #[test]
    fn test_get_u32_from_bytes_range() {
        let bytes: [u8; 6] = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45];
        let u32_from_bytes = get_u32_from_bytes_range(&bytes, 0..=3).unwrap();

        assert_eq!(u32_from_bytes, 0xABCD_EF01);
    }

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
