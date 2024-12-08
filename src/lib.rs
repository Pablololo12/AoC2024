use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct PairTerror<'a, T> {
    terror: std::slice::Iter<'a, T>,
    follower: std::slice::Iter<'a, T>,
}

impl<'a, T> PairTerror<'a, T> {
    pub fn new(terror: std::slice::Iter<'a, T>) -> Self {
        let mut follower = terror.clone();
        follower.next();
        PairTerror { terror, follower }
    }

    pub fn next(&mut self) -> Option<(&'a T, &'a T)> {
        match (self.terror.next(), self.follower.next()) {
            (Some(f), Some(ff)) => Some((f, ff)),
            _ => None,
        }
    }
}

impl<'a, T> Iterator for PairTerror<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn out_of_bounds(self, max: i32) -> bool {
        !(self.x >= 0 && self.x < max && self.y >= 0 && self.y < max)
    }
}
