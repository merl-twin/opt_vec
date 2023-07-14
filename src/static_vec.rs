use std::{
    ops::{Index,IndexMut},
};

macro_rules! self_macro {
    ( $m:ident, $self:ident ) => {
        $m!($self,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten)
    }
}

macro_rules! self_f_macro {
    ( $m:ident, $self:ident, $f:ident ) => {
        $m!($self,$f,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten)
    }
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum OptVec10<T> {
    None,
    One(T),
    Two([T; 2]),
    Three([T; 3]),
    Four([T; 4]),
    Five([T; 5]),
    Six([T; 6]),
    Seven([T; 7]),
    Eight([T; 8]),
    Nine([T; 9]),
    Ten([T; 10]),
    Vec(Vec<T>),
}
impl<T> Into<crate::vec::OptVec<T>> for OptVec10<T> {
    fn into(self) -> crate::vec::OptVec<T> {
        match self {
            OptVec10::None => crate::vec::OptVec::None,
            OptVec10::One(t) => crate::vec::OptVec::One(t),
            OptVec10::Two(s) => crate::vec::OptVec::Two(s),
            OptVec10::Three(s) => crate::vec::OptVec::Three(s),
            OptVec10::Four(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Five(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Six(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Seven(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Eight(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Nine(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Ten(iv) => crate::vec::OptVec::Vec(iv.into()),
            OptVec10::Vec(v) => crate::vec::OptVec::Vec(v),
        }
    }
}
impl<T> From<T> for OptVec10<T> {
    fn from(t: T) -> OptVec10<T> {
        OptVec10::One(t)
    }
}
impl<T> From<Vec<T>> for OptVec10<T> {
    fn from(mut ts: Vec<T>) -> OptVec10<T> {
        match ts.len() {
            0 => OptVec10::None,
            1 => OptVec10::One(ts.pop().unwrap()), // safe
            2 => {
                let mut iter = ts.into_iter();
                OptVec10::Two([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            3 => {
                let mut iter = ts.into_iter();
                OptVec10::Three([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            4 => {
                let mut iter = ts.into_iter();
                OptVec10::Four([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            5 => {
                let mut iter = ts.into_iter();
                OptVec10::Five([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            6 => {
                let mut iter = ts.into_iter();
                OptVec10::Six([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            7 => {
                let mut iter = ts.into_iter();
                OptVec10::Seven([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            8 => {
                let mut iter = ts.into_iter();
                OptVec10::Eight([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            9 => {
                let mut iter = ts.into_iter();
                OptVec10::Nine([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            10 => {
                let mut iter = ts.into_iter();
                OptVec10::Ten([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            _ => OptVec10::Vec(ts),
        }
    }
}

impl<T> Extend<T> for OptVec10<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        loop {
            match self {
                OptVec10::Vec(v) => {
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

impl<T> FromIterator<T> for OptVec10<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OptVec10<T> {
        let mut slf = OptVec10::None;
        slf.extend(iter);
        slf
    }
}

impl<T> Index<usize> for OptVec10<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).expect("OptVec10 index is out of range")
    }
}
impl<T> IndexMut<usize> for OptVec10<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("OptVec10 index is out of range")
    }
}

macro_rules! run_f_macro {
    ( $slf:ident, $f:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => {},
            OptVec10::One(t) => $f(t), 
            $( OptVec10::$value(s) => for t in s { $f(t); }, )*
            OptVec10::Vec(v) => for t in v { $f(t); },
        }
    }   
}

macro_rules! len_macro {
    ( $slf:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => 0,
            OptVec10::One(_) => 1,
            $( OptVec10::$value(s) => s.len(), )*
            OptVec10::Vec(v) => v.len(),
        }
    }   
}

macro_rules! get_macro {
    ( $slf:ident, $i:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => None,
            OptVec10::One(t) => match $i == 0 {
                true => Some(t),
                false => None,
            },
            $( OptVec10::$value(s) => s.get($i), )*
            OptVec10::Vec(v) => v.get($i),
        }
    }   
}

macro_rules! get_mut_macro {
    ( $slf:ident, $i:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => None,
            OptVec10::One(t) => match $i == 0 {
                true => Some(t),
                false => None,
            },
            $( OptVec10::$value(s) => s.get_mut($i), )*
            OptVec10::Vec(v) => v.get_mut($i),
        }
    }   
}

impl<T> OptVec10<T> {
    pub fn new() -> OptVec10<T> {
        OptVec10::None
    }
    pub fn with_capacity(sz: usize) -> OptVec10<T> {
        match sz > 10 {
            true => OptVec10::Vec(Vec::with_capacity(sz)),
            false => OptVec10::None,
        }
    }
    pub fn take(&mut self) -> OptVec10<T> {
        let mut tmp = OptVec10::None;
        std::mem::swap(self, &mut tmp);
        tmp
    }
    pub fn len(&self) -> usize {
        self_macro!(len_macro,self)
    }
    pub fn push(&mut self, el: T) {
        match self {
            OptVec10::None => *self = OptVec10::One(el),
            OptVec10::Vec(v) => v.push(el),
            _ => {
                let mut tmp = OptVec10::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptVec10::None |
                    OptVec10::Vec(_) => unreachable!(),
                    OptVec10::One(t) => *self = OptVec10::Two([t,el]),
                    OptVec10::Two([t1,t2]) => *self = OptVec10::Three([t1,t2,el]),
                    OptVec10::Three([t1,t2,t3]) => *self = OptVec10::Four([t1,t2,t3,el]),
                    OptVec10::Four([t1,t2,t3,t4]) => *self = OptVec10::Five([t1,t2,t3,t4,el]),
                    OptVec10::Five([t1,t2,t3,t4,t5]) => *self = OptVec10::Six([t1,t2,t3,t4,t5,el]),
                    OptVec10::Six([t1,t2,t3,t4,t5,t6]) => *self = OptVec10::Seven([t1,t2,t3,t4,t5,t6,el]),
                    OptVec10::Seven([t1,t2,t3,t4,t5,t6,t7]) => *self = OptVec10::Eight([t1,t2,t3,t4,t5,t6,t7,el]),
                    OptVec10::Eight([t1,t2,t3,t4,t5,t6,t7,t8]) => *self = OptVec10::Nine([t1,t2,t3,t4,t5,t6,t7,t8,el]),
                    OptVec10::Nine([t1,t2,t3,t4,t5,t6,t7,t8,t9]) => *self = OptVec10::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,el]),
                    OptVec10::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,t10]) => *self = OptVec10::Vec(vec![t1,t2,t3,t4,t5,t6,t7,t8,t9,t10,el]),
                }
            },
            
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self {
            OptVec10::None => None,
            OptVec10::Vec(v) => {
                v.pop()
            },
            _ => {
                let mut tmp = OptVec10::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptVec10::None |
                    OptVec10::Vec(_) => unreachable!(),
                    OptVec10::One(t) => Some(t),
                    OptVec10::Two([t1,q]) => {
                        *self = OptVec10::One(t1);
                        Some(q)
                    },
                    OptVec10::Three([t1,t2,q]) => {
                        *self = OptVec10::Two([t1,t2]);
                        Some(q)
                    },
                    OptVec10::Four([t1,t2,t3,q]) => {
                        *self = OptVec10::Three([t1,t2,t3]);
                        Some(q)
                    },
                    OptVec10::Five([t1,t2,t3,t4,q]) => {
                        *self = OptVec10::Four([t1,t2,t3,t4]);
                        Some(q)
                    },
                    OptVec10::Six([t1,t2,t3,t4,t5,q]) => {
                        *self = OptVec10::Five([t1,t2,t3,t4,t5]);
                        Some(q)
                    },
                    OptVec10::Seven([t1,t2,t3,t4,t5,t6,q]) => {
                        *self = OptVec10::Six([t1,t2,t3,t4,t5,t6]);
                        Some(q)
                    },
                    OptVec10::Eight([t1,t2,t3,t4,t5,t6,t7,q]) => {
                        *self = OptVec10::Seven([t1,t2,t3,t4,t5,t6,t7]);
                        Some(q)
                    },
                    OptVec10::Nine([t1,t2,t3,t4,t5,t6,t7,t8,q]) => {
                        *self = OptVec10::Eight([t1,t2,t3,t4,t5,t6,t7,t8]);
                        Some(q)
                    },
                    OptVec10::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,q]) => {
                        *self = OptVec10::Nine([t1,t2,t3,t4,t5,t6,t7,t8,t9]);
                        Some(q)
                    },
                }
            },
            
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self_f_macro!(get_macro,self,i)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self_f_macro!(get_mut_macro,self,i)
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
    }
    
    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        self_f_macro!(run_f_macro,self,f)
    }
    pub fn consume<F: FnMut(T)>(&mut self, mut f: F)  {
        let mut tmp = OptVec10::None;
        std::mem::swap(&mut tmp, self);
        self_f_macro!(run_f_macro,tmp,f)
    }
}

pub enum IntoIter<T> {
    None,
    One(T),
    Two([T;2]),
    Three([T;3]),
    Four([T;4]),
    Five([T; 5]),
    Six([T; 6]),
    Seven([T; 7]),
    Eight([T; 8]),
    Nine([T; 9]),
    Ten([T; 10]),
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
                    IntoIter::Two([q, t2]) => {
                        *self = IntoIter::One(t2);
                        Some(q)
                    },
                    IntoIter::Three([q, t2, t3]) => {
                        *self = IntoIter::Two([t2, t3]);
                        Some(q)
                    },
                    IntoIter::Four([q, t2, t3, t4]) => {
                        *self = IntoIter::Three([t2, t3, t4]);
                        Some(q)
                    },
                    IntoIter::Five([q, t2, t3, t4, t5]) => {
                        *self = IntoIter::Four([t2, t3, t4, t5]);
                        Some(q)
                    },
                    IntoIter::Six([q, t2, t3, t4, t5, t6]) => {
                        *self = IntoIter::Five([t2, t3, t4, t5, t6]);
                        Some(q)
                    },
                    IntoIter::Seven([q, t2, t3, t4, t5, t6, t7]) => {
                        *self = IntoIter::Six([t2, t3, t4, t5, t6, t7]);
                        Some(q)
                    },
                    IntoIter::Eight([q, t2, t3, t4, t5, t6, t7, t8]) => {
                        *self = IntoIter::Seven([t2, t3, t4, t5, t6, t7, t8]);
                        Some(q)
                    },
                    IntoIter::Nine([q, t2, t3, t4, t5, t6, t7, t8, t9]) => {
                        *self = IntoIter::Eight([t2, t3, t4, t5, t6, t7, t8, t9]);
                        Some(q)
                    },
                    IntoIter::Ten([q, t2, t3, t4, t5, t6, t7, t8, t9, t10]) => {
                        *self = IntoIter::Nine([t2, t3, t4, t5, t6, t7, t8, t9, t10]);
                        Some(q)
                    },
                }
            },
            
        }
    }
}

macro_rules! into_iter_macro {
    ( $slf:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => IntoIter::None,
            OptVec10::One(t) => IntoIter::One(t), 
            $( OptVec10::$value(s) => IntoIter::$value(s), )*
            OptVec10::Vec(v) => IntoIter::Vec(v.into_iter()),
        }
    }   
}

impl<T> IntoIterator for OptVec10<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self_macro!(into_iter_macro,self)
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

macro_rules! ref_into_iter_macro {
    ( $slf:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => Iter::None,
            OptVec10::One(t) => Iter::One(t),
            $( OptVec10::$value(s) => Iter::Slice(s.iter()), )*
            OptVec10::Vec(v) => Iter::Slice(v.iter()),
        }
    }   
}

impl<'t, T> IntoIterator for &'t OptVec10<T> {
    type Item = &'t T;
    type IntoIter = Iter<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        self_macro!(ref_into_iter_macro,self)
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

macro_rules! ref_mut_into_iter_macro {
    ( $slf:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec10::None => IterMut::None,
            OptVec10::One(t) => IterMut::One(t),
            $( OptVec10::$value(s) => IterMut::Slice(s.iter_mut()), )*
            OptVec10::Vec(v) => IterMut::Slice(v.iter_mut()),
        }
    }   
}

impl<'t, T> IntoIterator for &'t mut OptVec10<T> {
    type Item = &'t mut T;
    type IntoIter = IterMut<'t,T>;

    fn into_iter(self) -> Self::IntoIter {
        self_macro!(ref_mut_into_iter_macro,self)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut v = OptVec10::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        assert_eq!(v.len(),0);
        for i in &r {
            v.push(*i);
            assert_eq!(v.len(),i+1);
        }
        let mut lib_r = Vec::new();
        while v.len() > 0 {
            lib_r.insert(0,v.pop().unwrap());
        }
        assert_eq!(lib_r,r)
    }

    #[test]
    fn basic_get() {
        let mut v = OptVec10::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
            for n in 0 .. v.len() {
                assert_eq!(r[n],*(v.get(n).unwrap()));
                assert_eq!(r[n],*(v.get_mut(n).unwrap()));
            }
        }
    }
    
    #[test]
    fn basic_into_iter() {
        let mut v = OptVec10::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.into_iter().collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }

    #[test]
    fn basic_iter() {
        let mut v = OptVec10::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.iter().map(|x|*x).collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }

    #[test]
    fn basic_iter_mut() {
        let mut v = OptVec10::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.iter_mut().map(|x|*x).collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }
}
