use std::convert::TryInto;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

const APM_BLOCK_SIZE: usize = 512;

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

pub fn get_block_from_drive_and_correct_order(
    file: &mut File,
    location: u64,
    is_byte_swapped: bool,
) -> Result<Vec<u8>, String> {
    Ok(correct_byte_order(
        &get_block_from_drive(file, location)?,
        is_byte_swapped,
    ))
}

pub fn get_block_from_drive(file: &mut File, location: u64) -> Result<Vec<u8>, String> {
    get_blocks_from_drive(file, location, 1)
}

pub fn get_blocks_from_drive_and_correct_order(
    file: &mut File,
    location: u64,
    count: usize,
    is_byte_swapped: bool,
) -> Result<Vec<u8>, String> {
    Ok(correct_byte_order(
        &get_blocks_from_drive(file, location, count)?,
        is_byte_swapped,
    ))
}

pub fn get_blocks_from_drive(
    file: &mut File,
    location: u64,
    count: usize,
) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0; APM_BLOCK_SIZE * count];

    match file.seek(SeekFrom::Start(location * APM_BLOCK_SIZE as u64)) {
        Ok(_) => {}
        Err(_) => {
            return Err(format!(
                "Could not set file cursor to location {}",
                location
            ));
        }
    };

    match file.read(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(format!(
            "Could not read block from file at location {}",
            location
        )),
    }
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
