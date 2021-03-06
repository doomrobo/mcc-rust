use std::u8;

pub fn pkcs7_pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let block_max = u8::MAX as usize;
    if block_size > block_max {
        panic!("PKCS7 padding only works for block sizes <= {}", block_max);
    }

    let mut out: Vec<u8> = bytes.to_vec();
    let rem = block_size - (bytes.len() % block_size);
    for _ in 0..rem {
        out.push(rem as u8);
    }
    // If the input is already aligned on a block boundary, make a block of
    // block_size as u8
    if rem == 0 {
        out.push(block_size as u8);
    }

    out
}

// For when we don't care how it's padded; equivalent to pkcs7 except
// it won't add an extra block of padding if the input is alread aligned
pub fn minimal_pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    if bytes.len() % block_size == 0 {
        bytes.to_vec()
    } else {
        pkcs7_pad(bytes, block_size)
    }
}

#[test]
fn tst09() {
    let bytes = b"YELLOW SUBMARINE";
    assert_eq!(&pkcs7_pad(bytes, 20), b"YELLOW SUBMARINE\x04\x04\x04\x04");
}
