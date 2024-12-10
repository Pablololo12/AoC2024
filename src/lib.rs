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
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn out_of_bounds(self, max: T) -> bool
    where
        T: Ord + Copy,
        T: From<u8>,
    {
        !(self.x >= T::from(0) && self.x < max && self.y >= T::from(0) && self.y < max)
    }
}
