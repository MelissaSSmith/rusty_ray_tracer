use rand::RngExt;

#[derive(Clone, Copy, Debug)]
pub struct Sequence {
    base: [f64; 5],
    next_index: usize
}

impl Sequence {
    fn create(base: [f64; 5]) -> Sequence {
        Sequence {
            base,
            next_index: 0
        }
    }

    pub fn new() -> Sequence {
        let mut rng = rand::rng();
        let mut base = [0.0; 5];
        for n in &mut base {
            *n = rng.random_range(0.0..1.0);
        }
        Sequence::create(base)
    }

    pub fn one(one: f64) -> Sequence {
        Sequence::create([one, one, one, one, one])
    }

    pub fn two(one: f64, two: f64) -> Sequence {
        Sequence::create([one, two, one, two, f64::NAN])
    }

    pub fn three(one: f64, two: f64, three: f64) -> Sequence {
        Sequence::create([one, two, three, f64::NAN, f64::NAN])
    }

    pub fn four(one: f64, two: f64, three: f64, four: f64) -> Sequence {
        Sequence::create([one, two, three, four, f64::NAN])
    }

    pub fn five(one: f64, two: f64, three: f64, four: f64, five: f64) -> Sequence {
        Sequence::create([one, two, three, four, five])
    }

    pub fn next(&mut self) -> f64 {
        let next = self.base[self.next_index];
        self.next_index = self.generate_next_index(self.next_index);

        next
    }

    fn generate_next_index(&self, old_index: usize) -> usize {
        if old_index + 1 == self.base.len() {
            return 0;
        }
        let mut new_index = old_index + 1;
        if self.base[new_index].is_nan() {
            new_index = self.generate_next_index(new_index);
        }

        new_index

    }
}

#[cfg(test)]
mod tests {
    use crate::features::sequence::Sequence;

    #[test]
    fn test_number_generator_returns_a_cyclic_sequence_of_numbers() {
        let mut gen = Sequence::three(0.1, 0.5, 1.0);

        assert_eq!(gen.next(), 0.1);
        assert_eq!(gen.next(), 0.5);
        assert_eq!(gen.next(), 1.0);
        assert_eq!(gen.next(), 0.1);
    }

    #[test]
    fn test_new_generates_random_numbers() {
        let gen = Sequence::new();

        for num in gen.base {
            assert!(num > 0.0 && num < 1.0);
        }
    }
}