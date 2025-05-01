#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            None
        } else {
            let current = self.v;
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            Some(current)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return 0;
    }
    let mut steps = 0;
    let mut current = n;
    while current != 1 {
        current = if current % 2 == 0 {
            current / 2
        } else {
            3 * current + 1
        };
        steps += 1;
    }
    steps
}
