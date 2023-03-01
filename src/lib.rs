use std::{
    ops::{Index,IndexMut},
};


#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum OptVec<T> {
    None,
    One(T),
    Two([T; 2]),
    Vec(Vec<T>),
}
impl<T> From<T> for OptVec<T> {
    fn from(t: T) -> OptVec<T> {
        OptVec::One(t)
    }
}
impl<T> From<Vec<T>> for OptVec<T> {
    fn from(mut ts: Vec<T>) -> OptVec<T> {
        match ts.len() {
            0 => OptVec::None,
            1 => OptVec::One(ts.pop().unwrap()), // safe
            2 => {
                let v2 = ts.pop().unwrap(); //safe
                let v1 = ts.pop().unwrap(); // safe
                OptVec::Two([v1, v2])
            },
            _ => OptVec::Vec(ts),
        }
    }
}

impl<T> Extend<T> for OptVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        loop {
            match self {
                OptVec::Vec(v) => {
                    v.extend(iter);
                    break;
                },
                _ => match iter.next() {
                    None => break,
                    Some(t) => self.push(t),
                },
            }
        }
    }
}

impl<T> FromIterator<T> for OptVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OptVec<T> {
        let mut slf = OptVec::None;
        slf.extend(iter);
        slf
    }
}

impl<T> Index<usize> for OptVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).expect("OptVec index is out of range")
    }
}
impl<T> IndexMut<usize> for OptVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("OptVec index is out of range")
    }
}


impl<T> OptVec<T> {
    pub fn new() -> OptVec<T> {
        OptVec::None
    }
    pub fn len(&self) -> usize {
        match self {
            OptVec::None => 0,
            OptVec::One(_) => 1,
            OptVec::Two(_) => 2,
            OptVec::Vec(v) => v.len(),
        }
    }
    pub fn push(&mut self, el: T) {
        match self {
            OptVec::None => *self = OptVec::One(el),
            OptVec::Vec(v) => v.push(el),
            _ => {
                let mut tmp = OptVec::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptVec::None |
                    OptVec::Vec(_) => unreachable!(),
                    OptVec::One(t) => *self = OptVec::Two([t,el]),
                    OptVec::Two([t1,t2]) => *self = OptVec::Vec(vec![t1,t2,el]),
                }
            },
            
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self {
            OptVec::None => None,
            OptVec::Vec(v) => {
                v.pop()
            },
            _ => {
                let mut tmp = OptVec::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptVec::None |
                    OptVec::Vec(_) => unreachable!(),
                    OptVec::One(t) => Some(t),
                    OptVec::Two([t1,t2]) => {
                        *self = OptVec::One(t1);
                        Some(t2)
                    },
                }
            },
            
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            OptVec::None => None,
            OptVec::One(t) => match i == 0 {
                true => Some(t),
                false => None,
            },
            OptVec::Two(s) => s.get(i),
            OptVec::Vec(v) => v.get(i),
        }
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        match self {
            OptVec::None => None,
            OptVec::One(t) => match i == 0 {
                true => Some(t),
                false => None,
            },
            OptVec::Two(s) => s.get_mut(i),
            OptVec::Vec(v) => v.get_mut(i),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
    }

    
    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        match self {
            OptVec::None => {},
            OptVec::One(t) => f(t),
            OptVec::Two(s) => for t in s { f(t); },
            OptVec::Vec(v) => for t in v { f(t); },
        }
    }
    pub fn consume<F: FnMut(T)>(&mut self, mut f: F)  {
        let mut tmp = OptVec::None;
        std::mem::swap(&mut tmp, self);
        match tmp {
            OptVec::None => {},
            OptVec::One(t) => f(t),
            OptVec::Two(s) => for t in s { f(t); },
            OptVec::Vec(v) => for t in v { f(t); },
        }
    }
}




pub enum IntoIter<T> {
    None,
    One(T),
    Two([T;2]),
    Vec(std::vec::IntoIter<T>),
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            IntoIter::None => None,
            IntoIter::Vec(v) => v.next(),
            _ => {
                let mut tmp = IntoIter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IntoIter::None |
                    IntoIter::Vec(_) => unreachable!(),
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

impl<T> IntoIterator for OptVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptVec::None => IntoIter::None,
            OptVec::One(t) => IntoIter::One(t),
            OptVec::Two(s) => IntoIter::Two(s),
            OptVec::Vec(v) => IntoIter::Vec(v.into_iter()),
        }
    }
}


pub enum Iter<'t,T> {
    None,
    One(&'t T),
    Slice(std::slice::Iter<'t,T>),
}
impl<'t,T> Iterator for Iter<'t,T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::None => None,
            Iter::Slice(s) => s.next(),
            _ => {
                let mut tmp = Iter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    Iter::None |
                    Iter::Slice(_) => unreachable!(),
                    Iter::One(t) => Some(t),
                }
            },
        }
    }
}

impl<'t, T> IntoIterator for &'t OptVec<T> {
    type Item = &'t T;
    type IntoIter = Iter<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptVec::None => Iter::None,
            OptVec::One(t) => Iter::One(t),
            OptVec::Two(s) => Iter::Slice(s.iter()),
            OptVec::Vec(v) => Iter::Slice(v.iter()),
        }
    }
}

pub enum IterMut<'t,T> {
    None,
    One(&'t mut T),
    Slice(std::slice::IterMut<'t,T>),
}
impl<'t,T> Iterator for IterMut<'t,T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMut::None => None,
            IterMut::Slice(s) => s.next(),
            _ => {
                let mut tmp = IterMut::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IterMut::None |
                    IterMut::Slice(_) => unreachable!(),
                    IterMut::One(t) => Some(t),
                }
            },
        }
    }
}

impl<'t, T> IntoIterator for &'t mut OptVec<T> {
    type Item = &'t mut T;
    type IntoIter = IterMut<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptVec::None => IterMut::None,
            OptVec::One(t) => IterMut::One(t),
            OptVec::Two(s) => IterMut::Slice(s.iter_mut()),
            OptVec::Vec(v) => IterMut::Slice(v.iter_mut()),
        }
    }
}
