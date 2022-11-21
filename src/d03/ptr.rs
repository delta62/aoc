#[derive(Debug, Default)]
pub struct Tree {
    seen: usize,
    one: Option<Box<Tree>>,
    zero: Option<Box<Tree>>,
}

impl Tree {
    pub fn add_num(&mut self, num: u32, bit_width: usize) {
        self.seen += 1;
        let bit_set = num & (1 << bit_width) != 0;

        if bit_set {
            if self.one.is_none() {
                self.one = Some(Box::new(Default::default()));
            }

            if bit_width == 0 {
                return;
            }
            self.one.as_mut().unwrap().add_num(num, bit_width - 1);
        } else {
            if self.zero.is_none() {
                self.zero = Some(Box::new(Default::default()));
            }

            if bit_width == 0 {
                return;
            }
            self.zero.as_mut().unwrap().add_num(num, bit_width - 1);
        }
    }

    pub fn common_bits(&self, bit_width: usize) -> u32 {
        let mut result = 0;
        let mut ptr = self;

        for bit in (0..bit_width).rev() {
            let ones = ptr.one.as_ref().map(|t| t.seen).unwrap_or_default();
            let zeroes = ptr.zero.as_ref().map(|t| t.seen).unwrap_or_default();

            if ones >= zeroes {
                result |= 1 << bit;
                ptr = ptr.one.as_ref().unwrap();
            } else {
                ptr = ptr.zero.as_ref().unwrap();
            }
        }

        result
    }

    pub fn uncommon_bits(&self, bit_width: usize) -> u32 {
        let mut result = 0;
        let mut ptr = self;

        for bit in (0..bit_width).rev() {
            let ones = ptr.one.as_ref().map(|t| t.seen).unwrap_or(usize::MAX);
            let zeroes = ptr.zero.as_ref().map(|t| t.seen).unwrap_or(usize::MAX);

            if ones < zeroes {
                result |= 1 << bit;
                ptr = ptr.one.as_ref().unwrap();
            } else {
                ptr = ptr.zero.as_ref().unwrap();
            }
        }

        result
    }
}
