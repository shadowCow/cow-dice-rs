use rand::prelude::*;

pub enum D6 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six
}

impl D6 {
    pub fn roll() -> D6 {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..7);

        match value {
            1 => D6::One,
            2 => D6::Two,
            3 => D6::Three,
            4 => D6::Four,
            5 => D6::Five,
            6 => D6::Six,
            _ => D6::One, // should not happen, but satisfy the type checker.
        }
    }

    pub fn pips(&self) -> u8 {
        match self {
            D6::One => 1,
            D6::Two => 2,
            D6::Three => 3,
            D6::Four => 4,
            D6::Five => 5,
            D6::Six => 6,
        }
    }
}

pub struct TwoD6 {
    d1: D6,
    d2: D6,
}

impl TwoD6 {
    pub fn roll() -> TwoD6 {
        TwoD6 { d1: D6::roll(), d2: D6::roll() }
    }

    pub fn pips(&self) -> u8 {
        self.d1.pips() + self.d2.pips()
    }

    pub fn probability(v: &u8) -> f64 {
        match v {
            2 => 1.0/36.0,
            3 => 2.0/36.0,
            4 => 3.0/36.0,
            5 => 4.0/36.0,
            6 => 5.0/36.0,
            7 => 6.0/36.0,
            8 => 5.0/36.0,
            9 => 4.0/36.0,
            10 => 3.0/36.0,
            11 => 2.0/36.0,
            12 => 1.0/36.0,
            _ => 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d6_roll() {
        let num_samples = 1000;

        for i in [0..num_samples] {
            let die = D6::roll();
        }
    }

    #[test]
    fn test_d6_pips() {
        assert_eq!(1, D6::One.pips());
        assert_eq!(2, D6::Two.pips());
        assert_eq!(3, D6::Three.pips());
        assert_eq!(4, D6::Four.pips());
        assert_eq!(5, D6::Five.pips());
        assert_eq!(6, D6::Six.pips());
    }

    #[test]
    fn test_twod6_roll() {
        let num_samples = 1000;

        for i in [0..num_samples] {
            let pair = TwoD6::roll();
        }
    }
}