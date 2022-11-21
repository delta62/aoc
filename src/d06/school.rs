pub type Fish = u8;

pub struct School {
    fish_ages: [usize; 9],
}

impl School {
    pub fn simulate(&mut self) {
        let ages = &mut self.fish_ages;

        let mut carry = 0;
        for idx in (0..9).rev() {
            if idx == 0 {
                let zero_count = ages[0];
                ages[6] += zero_count;
                ages[8] += zero_count;
                ages[0] = carry;
            } else {
                std::mem::swap(&mut ages[idx], &mut carry);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.fish_ages.iter().sum()
    }
}

impl FromIterator<Fish> for School {
    fn from_iter<T: IntoIterator<Item = Fish>>(iter: T) -> Self {
        let mut fish_ages = [0; 9];

        for fish in iter {
            let age = fish as usize;
            fish_ages[age] += 1;
        }

        Self { fish_ages }
    }
}
