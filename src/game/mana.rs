use std::cmp::{max, min};
use std::ops;

#[derive(Debug)]
pub struct Mana {
    current: i8,
    maximum: i8
}

impl Mana {
    pub fn new(current: i8, maximum: i8) -> Mana {
        Mana {
            current: current,
            maximum: maximum
        }
    }

    pub fn refresh_mana(&mut self) {
        self.current = self.maximum;
    }

    pub fn increment_max_mana(&mut self) {
        if self.maximum < 10 {
            self.maximum += 1
        }
    }

    pub fn increment_and_refresh_mana(&mut self) {
        self.increment_max_mana();
        self.refresh_mana();
    }
}

/// Allow + operations on the Mana structure
impl ops::Add<i8> for Mana {
    type Output = Mana;

    fn add(self, other: i8) -> Mana {
        Mana {
            current: max(0, min(self.maximum, self.current + other)),
            maximum: self.maximum
        }
    }
}


/// Allow += binary assignment
impl ops::AddAssign<i8> for Mana {
    fn add_assign(&mut self, other: i8) {
        *self = Mana {
            current: max(0, min(self.maximum, self.current + other)),
            maximum: self.maximum
        }
    }
}


/// Allow + operations on the Mana structure
impl ops::Sub<i8> for Mana {
    type Output = Mana;

    fn sub(self, other: i8) -> Mana {
        Mana {
            current: max(0, min(self.maximum, self.current - other)),
            maximum: self.maximum
        }
    }
}

/// Allow += binary assignment
impl ops::SubAssign<i8> for Mana {
    fn sub_assign(&mut self, other: i8) {
        *self = Mana {
            current: max(0, min(self.maximum, self.current - other)),
            maximum: self.maximum
        }
    }
}