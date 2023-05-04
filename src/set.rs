use std::{
    collections::BTreeSet,
};


#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum OptSet<T> {
    None,
    One(T),
    Two([T; 2]),
    Set(BTreeSet<T>),
}
impl<T> From<T> for OptSet<T> {
    fn from(t: T) -> OptSet<T> {
        OptSet::One(t)
    }
}
impl<T: Ord> From<Vec<T>> for OptSet<T> {
    fn from(ts: Vec<T>) -> OptSet<T> {
        let ts = ts.into_iter().collect::<BTreeSet<_>>();
        OptSet::from(ts)
    }
}
impl<T: Ord> From<BTreeSet<T>> for OptSet<T> {
    fn from(ts: BTreeSet<T>) -> OptSet<T> {        
        match ts.len() {
            0 => OptSet::None,
            1 => OptSet::One(ts.into_iter().next().unwrap()), // safe
            2 => {
                let mut it = ts.into_iter();
                let v1 = it.next().unwrap(); //safe
                let v2 = it.next().unwrap(); // safe
                OptSet::Two([v1, v2])
            },
            _ => OptSet::Set(ts),
        }
    }
}

impl<T: Ord> Extend<T> for OptSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        loop {
            match self {
                OptSet::Set(v) => {
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

impl<T: Ord> FromIterator<T> for OptSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OptSet<T> {
        let mut slf = OptSet::None;
        slf.extend(iter);
        slf
    }
}

impl<T: Ord> OptSet<T> {
    pub fn new() -> OptSet<T> {
        OptSet::None
    }
    pub fn len(&self) -> usize {
        match self {
            OptSet::None => 0,
            OptSet::One(_) => 1,
            OptSet::Two(_) => 2,
            OptSet::Set(v) => v.len(),
        }
    }
    pub fn insert(&mut self, el: T) -> bool { // true if new
        match self {
            OptSet::None => {
                *self = OptSet::One(el);
                true
            },
            OptSet::Set(v) => v.insert(el),
            _ => {
                let mut tmp = OptSet::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptSet::None |
                    OptSet::Set(_) => unreachable!(),
                    OptSet::One(t) => match t != el {
                        true => {
                            *self = match t <= el {
                                true => OptSet::Two([t,el]),
                                false => OptSet::Two([el,t]),
                            };
                            true
                        },
                        false => {
                            *self = OptSet::One(t);
                            false
                        }
                    },
                    OptSet::Two([t1,t2]) => match (t1 != el)&&(t2 != el) {
                        true => {
                            *self = OptSet::from(vec![t1,t2,el]);
                            true
                        },
                        false => {
                            *self =  OptSet::Two([t1,t2]);
                            false
                        }
                    },
                }
            },
            
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self {
            OptSet::None => false,
            OptSet::One(t) => t == value,
            OptSet::Two([t1,t2]) => (t1 == value)||(t2 == value),
            OptSet::Set(v) => v.contains(value),
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
    Set(std::collections::btree_set::IntoIter<T>),
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

impl<T> IntoIterator for OptSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptSet::None => IntoIter::None,
            OptSet::One(t) => IntoIter::One(t),
            OptSet::Two(s) => IntoIter::Two(s),
            OptSet::Set(v) => IntoIter::Set(v.into_iter()),
        }
    }
}


pub enum Iter<'t,T> {
    None,
    One(&'t T),
    Slice(std::slice::Iter<'t,T>),
    Set(std::collections::btree_set::Iter<'t,T>),
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

impl<'t, T> IntoIterator for &'t OptSet<T> {
    type Item = &'t T;
    type IntoIter = Iter<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptSet::None => Iter::None,
            OptSet::One(t) => Iter::One(t),
            OptSet::Two(s) => Iter::Slice(s.iter()),
            OptSet::Set(v) => Iter::Set(v.iter()),
        }
    }
}

