extern crate libc;

pub mod jit;

use Implicit::*;

#[derive(Clone, PartialEq)]
pub enum Implicit {
    Infinite,
    Point(f32, f32),
    Shift(f32, Box<Implicit>),
    Or(Box<Implicit>, Box<Implicit>),
    And(Box<Implicit>, Box<Implicit>),
    Not(Box<Implicit>)
}


impl Implicit {
    pub fn point(x: f32, y: f32) -> Implicit {
        Point(x, y)
    }

    pub fn shift(self, by: f32) -> Implicit {
        Shift(by, Box::new(self))
    }

    pub fn or(self, other: Implicit) -> Implicit {
        Or(Box::new(self), Box::new(other))
    }

    pub fn and(self, other: Implicit) -> Implicit {
        And(Box::new(self), Box::new(other))
    }

    pub fn xor(self, other: Implicit) -> Implicit {
        Implicit::or(
            self.clone().and(other.clone().not()),
            self.not().and(other)
        )
    }

    pub fn not(self) -> Implicit {
        Not(Box::new(self))
    }

    pub fn or_these(mut all: Vec<Implicit>) -> Implicit {
        match all.pop() {
            Some(last) => {
                all.into_iter().fold(last, |a, b| a.or(b))
            }
            None => Infinite
        }
    }

    pub fn and_these(mut all: Vec<Implicit>) -> Implicit {
        match all.pop() {
            Some(last) => {
                all.into_iter().fold(last, |a, b| a.and(b))
            }
            None => Infinite
        }
    }
}

pub fn evaluate(implicit: &Implicit, x: f32, y: f32) -> f32 {
    match implicit {
        &Point(px, py) => {
            let dx = px - x;
            let dy = py - y;
            (dx * dx + dy * dy).sqrt()
        }
        &Shift(by, ref other) => evaluate(other, x, y) + by,
        &Or(ref a, ref b) => evaluate(a, x, y).max(evaluate(b, x, y)),
        &And(ref a, ref b) => evaluate(a, x, y).min(evaluate(b, x, y)),
        &Not(ref t) => -evaluate(t, x, y),
        &Infinite => ::std::f32::INFINITY,
    }
}
