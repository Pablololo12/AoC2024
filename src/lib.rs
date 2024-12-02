pub struct PairTerator<'a, T> {
    front: std::slice::Iter<'a, T>,
    back: std::slice::Iter<'a, T>,
}

impl<'a, T> PairTerator<'a, T> {
    pub fn new(vec: &'a [T]) -> Self {
        let mut front = vec.iter();
        let back = vec.iter();
        front.next();
        PairTerator { front, back }
    }

    pub fn next(&mut self) -> Option<(&'a T, &'a T)> {
        match (self.front.next(), self.back.next()) {
            (Some(f), Some(ff)) => Some((f, ff)),
            _ => None,
        }
    }
}

impl<'a, T> Iterator for PairTerator<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

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
