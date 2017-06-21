#[derive(Debug, PartialEq)]
pub enum UnpaddingError {
    /// Got zero-length vector
    InputIsEmpty,

    /// `length % block_size != 0`
    InputNotAligned,

    /// At least one padding byte isn't equal to padding length
    InvalidPadding,
}

/// PKCS#7 padding
pub fn pad_pkcs7(data: &mut Vec<u8>, block_size: usize) {
    let len = data.len();
    let bytes_occupied = len % block_size;
    if bytes_occupied != 0 {
        let pad_bytes_count = block_size - bytes_occupied;
        data.resize(len + pad_bytes_count, pad_bytes_count as u8);
    } else {
        data.append(&mut vec![0; 16]);
    }
}

/// PKCS#7 unpadding (leaves data intact on error)
pub fn unpad_pkcs7(data: &mut Vec<u8>, block_size: usize) -> Result<(), UnpaddingError> {
    let len = data.len();

    if len % block_size != 0 {
        return Err(UnpaddingError::InputNotAligned);
    }

    let mut popped_bytes: Vec<u8> = vec![0; 0];
    match data.pop() {
        // The extra block case
        Some(0) => {
            popped_bytes.push(0);
            // See if the remaining (block_size - 1) bytes are also 0
            for _ in 0..(block_size - 1) {
                if let Some(byte) = data.pop() {
                    popped_bytes.push(byte);
                    if byte != 0 {
                        // Restore all popped bytes
                        popped_bytes.reverse();
                        data.extend_from_slice(&mut popped_bytes);
                        return Err(UnpaddingError::InvalidPadding);
                    }
                }
            }
        }
        Some(n) => {
            popped_bytes.push(n);
            // Err if last byte is not sane
            if n >= block_size as u8 {
                data.push(n);
                return Err(UnpaddingError::InvalidPadding);
            }
            // See if the remaining (n - 1) bytes are also n
            for _ in 0..(n - 1) {
                if let Some(byte) = data.pop() {
                    popped_bytes.push(byte);
                    if byte != n {
                        // Restore all popped bytes
                        popped_bytes.reverse();
                        data.extend_from_slice(&mut popped_bytes);
                        return Err(UnpaddingError::InvalidPadding);
                    }
                }
            }
        }
        None => return Err(UnpaddingError::InputIsEmpty),
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    extern crate rand;

    use rand::Rng;
    use super::*;
    use UnpaddingError::*;
    use std::u8;

    #[test]
    fn input_not_aligned_unpad_test() {
        let block_size = 16;

        for i in 1..block_size {
            let misaligned = vec![0u8; i];
            let mut misaligned_cpy = misaligned.clone();

            if let Err(e) = unpad_pkcs7(&mut misaligned_cpy, block_size) {
                assert_eq!(e, InputNotAligned);
                assert_eq!(misaligned_cpy, misaligned);
            } else {
                panic!("Got Ok() for misaligned input");
            }
        }
    }

    #[test]
    fn null_block_unpad_test() {
        let block_size = 16;
        let mut block = vec![0u8; block_size];

        if let Err(e) = unpad_pkcs7(&mut block, block_size) {
            panic!("Got an error for valid input! {:#?}", e)
        } else {
            assert_eq!(block, vec![0u8; 0]);
        }
    }

    #[test]
    fn invalid_null_block_unpad_test() {
        let block_size = 16;

        for i in 0..(block_size - 1) {
            let mut invalid: Vec<u8> = vec![0u8; block_size];
            invalid[i] = 1u8;
            let mut invalid_cpy = invalid.clone();

            if let Err(e) = unpad_pkcs7(&mut invalid_cpy, block_size) {
                assert_eq!(e, InvalidPadding);
                assert_eq!(invalid_cpy, invalid);
            } else {
                panic!("Got Ok() for invalid input");
            }
        }
    }

    #[test]
    fn normal_block_unpad_test() {
        let block_size = 16;

        for i in 1..block_size {
            let bytes_occupied = block_size - i;

            let mut block = vec![0u8; bytes_occupied];
            pad_pkcs7(&mut block, block_size);

            let mut block_cpy = block.clone();

            if let Err(e) = unpad_pkcs7(&mut block_cpy, block_size) {
                panic!("Got an error for valid input! {:#?}", e);
            } else {
                assert_eq!(block_cpy.len(), bytes_occupied);
                assert_eq!(block_cpy[..], block[..(bytes_occupied)])
            }
        }
    }

    #[test]
    fn invalid_normal_block_unpad_test() {
        let block_size: usize = 16;

        let mut generator = rand::thread_rng();

        for i in 2..block_size {
            let bytes_occupied = block_size - i;

            // Pick a byte that won't match the padding and fit in bounds
            let mut faulty_byte: u8;
            loop {
                faulty_byte = generator.gen_range(0, (block_size - 1) as u8);
                if faulty_byte != i as u8 {
                    break;
                }
            }

            // Properly pad i vacant bytes
            let mut invalid = vec![0u8; bytes_occupied];
            pad_pkcs7(&mut invalid, block_size);

            let faulty_index = generator.gen_range(bytes_occupied, block_size - 1);

            invalid[faulty_index] = faulty_byte;

            let mut invalid_cpy = invalid.clone();

            if let Err(e) = unpad_pkcs7(&mut invalid_cpy, block_size) {
                assert_eq!(invalid_cpy, invalid);
                assert_eq!(e, InvalidPadding);
            } else {
                panic!("Got an Ok() for invalid input!");
            }

            // test same index with a value out of bounds
            invalid[faulty_index] += block_size as u8;

            invalid_cpy = invalid.clone();

            if let Err(e) = unpad_pkcs7(&mut invalid_cpy, block_size) {
                assert_eq!(invalid_cpy, invalid);
                assert_eq!(e, InvalidPadding);
            } else {
                panic!("Got an Ok() for invalid input!");
            }

        }

    }

    #[test]
    fn empty_unpad_test() {
        let block_size = 16;

        let empty = vec![0u8; 0];
        let mut empty_cpy = empty.clone();

        if let Err(e) = unpad_pkcs7(&mut empty_cpy, block_size) {
            assert_eq!(e, InputIsEmpty);
            assert_eq!(empty_cpy, empty);
        } else {
            panic!("Got Ok() for empty input");
        }
    }
}
