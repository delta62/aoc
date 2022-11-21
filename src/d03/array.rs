#[derive(Debug)]
pub struct ArrayTree {
    width: usize,
    elements: Vec<u32>,
}

impl ArrayTree {
    pub fn new(width: usize) -> Self {
        assert!(width > 0, "Width must be positive");

        let element_count = 2usize.pow(width as u32 + 1);
        let elements = vec![0; element_count];

        Self { width, elements }
    }

    pub fn add_num(&mut self, n: u32) {
        let mut bit = self.width - 1;
        let mut dest = 1;

        loop {
            // SAFETY: We always initialize `self.elements` to have length of
            // 2^(width + 1), and that is the maximum number `dest` can be set
            // to in this loop.
            unsafe { *self.elements.get_unchecked_mut(dest) += 1 };

            let bit_set = n & (1 << bit) != 0;
            if bit_set {
                dest *= 2;
            } else {
                dest = dest * 2 + 1;
            }

            if bit == 0 {
                break;
            }
            bit -= 1;
        }
    }

    pub fn common_bits(&self) -> u32 {
        let mut result = 0;
        let mut i = 1;
        let mut bit = self.width - 1;

        loop {
            let ones = self.elements[i * 2];
            let zeroes = self.elements[i * 2 + 1];

            if ones >= zeroes {
                result |= 1 << bit;
                i *= 2;
            } else {
                i = i * 2 + 1;
            }

            if bit == 0 {
                break;
            }
            bit -= 1;
        }

        result
    }

    pub fn uncommon_bits(&self) -> u32 {
        let mut result = 0;
        let mut i = 1;
        let mut bit = self.width - 1;

        loop {
            let ones = self.elements[i * 2];
            let zeroes = self.elements[i * 2 + 1];

            if ones < zeroes || ones > 0 && zeroes == 0 {
                result |= 1 << bit;
                i *= 2;
            } else {
                i = i * 2 + 1;
            }

            if bit == 0 {
                break;
            }
            bit -= 1;
        }

        result
    }
}
