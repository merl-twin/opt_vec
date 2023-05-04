use std::{
    hash::Hash,
    collections::HashSet,
};


#[derive(Debug,Clone,PartialEq,Eq)]
pub enum OptHashSet<T: Hash + Eq> {
    None,
    One(T),
    Two([T; 2]),
    Set(HashSet<T>),
}
impl<T: Hash + Eq> From<T> for OptHashSet<T> {
    fn from(t: T) -> OptHashSet<T> {
        OptHashSet::One(t)
    }
}
impl<T: Hash + Eq> From<Vec<T>> for OptHashSet<T> {
    fn from(ts: Vec<T>) -> OptHashSet<T> {
        let ts = ts.into_iter().collect::<HashSet<_>>();
        OptHashSet::from(ts)
    }
}
impl<T: Hash + Eq> From<HashSet<T>> for OptHashSet<T> {
    fn from(ts: HashSet<T>) -> OptHashSet<T> {        
        match ts.len() {
            0 => OptHashSet::None,
            1 => OptHashSet::One(ts.into_iter().next().unwrap()), // safe
            2 => {
                let mut it = ts.into_iter();
                let v1 = it.next().unwrap(); //safe
                let v2 = it.next().unwrap(); // safe
                OptHashSet::Two([v1, v2])
            },
            _ => OptHashSet::Set(ts),
        }
    }
}

impl<T: Hash + Eq> Extend<T> for OptHashSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        loop {
            match self {
                OptHashSet::Set(v) => {
                    v.extend(iter);
                    break;
                },
                _ => match iter.next() {
                    None => break,
                    Some(t) => { self.insert(t); },
                },
            }
        }
    }
}

impl<T: Hash + Eq> FromIterator<T> for OptHashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OptHashSet<T> {
        let mut slf = OptHashSet::None;
        slf.extend(iter);
        slf
    }
}

impl<T: Hash + Eq> OptHashSet<T> {
    pub fn new() -> OptHashSet<T> {
        OptHashSet::None
    }
    pub fn len(&self) -> usize {
        match self {
            OptHashSet::None => 0,
            OptHashSet::One(_) => 1,
            OptHashSet::Two(_) => 2,
            OptHashSet::Set(v) => v.len(),
        }
    }
    pub fn insert(&mut self, el: T) -> bool { // true if new
        match self {
            OptHashSet::None => {
                *self = OptHashSet::One(el);
                true
            },
            OptHashSet::Set(v) => v.insert(el),
            _ => {
                let mut tmp = OptHashSet::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptHashSet::None |
                    OptHashSet::Set(_) => unreachable!(),
                    OptHashSet::One(t) => match t != el {
                        true => {
                            *self = OptHashSet::Two([t,el]);
                            true
                        },
                        false => {
                            *self = OptHashSet::One(t);
                            false
                        }
                    },
                    OptHashSet::Two([t1,t2]) => match (t1 != el)&&(t2 != el) {
                        true => {
                            *self = OptHashSet::from(vec![t1,t2,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Two([t1,t2]);
                            false
                        }
                    },
                }
            },
            
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self {
            OptHashSet::None => false,
            OptHashSet::One(t) => t == value,
            OptHashSet::Two([t1,t2]) => (t1 == value)||(t2 == value),
            OptHashSet::Set(v) => v.contains(value),
        }
    }
    
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
}




pub enum IntoIter<T> {
    None,
    One(T),
    Two([T;2]),
    Set(std::collections::hash_set::IntoIter<T>),
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            IntoIter::None => None,
            IntoIter::Set(v) => v.next(),
            _ => {
                let mut tmp = IntoIter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IntoIter::None |
                    IntoIter::Set(_) => unreachable!(),
                    IntoIter::One(t) => Some(t),
                    IntoIter::Two([t1, t2]) => {
                        *self = IntoIter::One(t2);
                        Some(t1)
                    },
                }
            },
            
        }
    }
}

impl<T: Hash + Eq> IntoIterator for OptHashSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptHashSet::None => IntoIter::None,
            OptHashSet::One(t) => IntoIter::One(t),
            OptHashSet::Two(s) => IntoIter::Two(s),
            OptHashSet::Set(v) => IntoIter::Set(v.into_iter()),
        }
    }
}


pub enum Iter<'t,T> {
    None,
    One(&'t T),
    Slice(std::slice::Iter<'t,T>),
    Set(std::collections::hash_set::Iter<'t,T>),
}
impl<'t,T> Iterator for Iter<'t,T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::None => None,
            Iter::Slice(s) => s.next(),
            Iter::Set(s) => s.next(),
            _ => {
                let mut tmp = Iter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    Iter::None |
                    Iter::Slice(_) |
                    Iter::Set(_) => unreachable!(),
                    Iter::One(t) => Some(t),
                }
            },
        }
    }
}

impl<'t, T: Hash + Eq> IntoIterator for &'t OptHashSet<T> {
    type Item = &'t T;
    type IntoIter = Iter<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptHashSet::None => Iter::None,
            OptHashSet::One(t) => Iter::One(t),
            OptHashSet::Two(s) => Iter::Slice(s.iter()),
            OptHashSet::Set(v) => Iter::Set(v.iter()),
        }
    }
}

