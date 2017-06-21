#[derive(Debug, PartialEq)]
pub enum UnpaddingError {
    /// Got zero-length vector
    InputIsEmpty,

    /// `length % block_size != 0`
    InputNotAligned,

    /// At least one byte in the extra null block isn't 0x00
    NullBlockInconsistent,

    /// The implied padding byte count is bigger than `block_size - 1`
    PaddingBytesOutOfBounds,

    /// At least one padding byte isn't equal to padding length
    PaddingBytesIncosistent,
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
                        return Err(UnpaddingError::NullBlockInconsistent);
                    }
                }
            }
        }
        Some(n) => {
            popped_bytes.push(n);
            if n >= block_size as u8 {
                data.push(n);
                return Err(UnpaddingError::PaddingBytesOutOfBounds);
            }
            // See if the remaining (n - 1) bytes are also n
            for _ in 0..(n - 1) {
                if let Some(byte) = data.pop() {
                    popped_bytes.push(byte);
                    if byte != n {
                        // Restore all popped bytes
                        popped_bytes.reverse();
                        data.extend_from_slice(&mut popped_bytes);
                        return Err(UnpaddingError::PaddingBytesIncosistent);
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
    use super::*;
    use UnpaddingError::*;

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
    fn inconsistent_null_block_unpad_test() {
        let block_size = 16;

        for i in 0..(block_size - 1) {
            let mut inconsistent: Vec<u8> = vec![0u8; block_size];
            inconsistent[i] = 1u8;
            let mut inconsistent_cpy = inconsistent.clone();

            if let Err(e) = unpad_pkcs7(&mut inconsistent_cpy, block_size) {
                assert_eq!(e, NullBlockInconsistent);
                assert_eq!(inconsistent_cpy, inconsistent);
            } else {
                panic!("Got Ok() for inconsistent input");
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
    fn inconsistent_normal_block_unpad_test() {
        let block_size = 16;

    }
}
