#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    /// Init a new Rc4 stream cipher instance
    pub fn new(key: &[u8]) -> Self {
        // Verify valid key length (40 to 2048 bits)
        assert!(5 <= key.len() && key.len() <= 256);

        // Zero-init a new instance
        let mut rc4 = Rc4 {
            s: [0; 256],
            i: 0,
            j: 0,
        };

        // Cipher state identity permutation
        for (i, b) in rc4.s.iter_mut().enumerate() {
            // s[i] = i
            *b = i as u8;
        }

        let mut j = 0u8;

        // Process for 256 iterations, starting from initial state
        for i in 0..256 {
            j = j.wrapping_add(rc4.s[i]).wrapping_add(key[i % key.len()]);
            rc4.s.swap(i, j as usize);
        }

        rc4
    }
}
