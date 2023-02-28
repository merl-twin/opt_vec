use std::{
    ops::{Index,IndexMut},
};

#[derive(Debug)]
pub enum OptVec<T> {
    None,
    One(T),
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
            _ => OptVec::Vec(ts),
        }
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
            OptVec::Vec(v) => v.len(),
        }
    }
    pub fn push(&mut self, el: T) {
        match self {
            OptVec::None => *self = OptVec::One(el),
            OptVec::One(_) => {
                let mut tmp = OptVec::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptVec::One(t) => *self = OptVec::Vec(vec![t,el]),
                    _ => unreachable!(),
                }
            },
            OptVec::Vec(v) => v.push(el),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            OptVec::None => None,
            OptVec::One(t) => match i == 0 {
                true => Some(t),
                false => None,
            },
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
            OptVec::Vec(v) => v.get_mut(i),
        }
    }

    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        match self {
            OptVec::None => {},
            OptVec::One(t) => f(t),
            OptVec::Vec(v) => for t in v { f(t); },
        }
    }
    pub fn consume<F: FnMut(T)>(&mut self, mut f: F)  {
        let mut tmp = OptVec::None;
        std::mem::swap(&mut tmp, self);
        match tmp {
            OptVec::None => {},
            OptVec::One(t) => f(t),
            OptVec::Vec(v) => for t in v { f(t); },
        }
    }
}




pub enum IntoIter<T> {
    None,
    One(T),
    Vec(std::vec::IntoIter<T>),
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            IntoIter::None => None,
            IntoIter::One(_) => {
                let mut tmp = IntoIter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IntoIter::One(t) => Some(t),
                    _ => unreachable!(),
                }
            },
            IntoIter::Vec(v) => v.next(),
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
            OptVec::Vec(v) => IntoIter::Vec(v.into_iter()),
        }
    }
}


pub enum Iter<'t,T> {
    None,
    One(&'t T),
    Vec(std::slice::Iter<'t,T>),
}
impl<'t,T> Iterator for Iter<'t,T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::None => None,
            Iter::One(_) => {
                let mut tmp = Iter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    Iter::One(t) => Some(t),
                    _ => unreachable!(),
                }
            },
            Iter::Vec(v) => v.next(),
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
            OptVec::Vec(v) => Iter::Vec(v.iter()),
        }
    }
}

pub enum IterMut<'t,T> {
    None,
    One(&'t mut T),
    Vec(std::slice::IterMut<'t,T>),
}
impl<'t,T> Iterator for IterMut<'t,T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMut::None => None,
            IterMut::One(_) => {
                let mut tmp = IterMut::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IterMut::One(t) => Some(t),
                    _ => unreachable!(),
                }
            },
            IterMut::Vec(v) => v.next(),
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
            OptVec::Vec(v) => IterMut::Vec(v.iter_mut()),
        }
    }
}
