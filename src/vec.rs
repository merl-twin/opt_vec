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
pub enum OptVec<T> {
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
                let mut iter = ts.into_iter();
                OptVec::Two([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            3 => {
                let mut iter = ts.into_iter();
                OptVec::Three([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
            },
            4 => {
                let mut iter = ts.into_iter();
                OptVec::Four([
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                    iter.next().unwrap(), // safe
                ])
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

macro_rules! run_f_macro {
    ( $slf:ident, $f:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec::None => {},
            OptVec::One(t) => $f(t), 
            $( OptVec::$value(s) => for t in s { $f(t); }, )*
            OptVec::Vec(v) => for t in v { $f(t); },
        }
    }   
}

macro_rules! len_macro {
    ( $slf:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec::None => 0,
            OptVec::One(_) => 1,
            $( OptVec::$value(s) => s.len(), )*
            OptVec::Vec(v) => v.len(),
        }
    }   
}

macro_rules! get_macro {
    ( $slf:ident, $i:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec::None => None,
            OptVec::One(t) => match $i == 0 {
                true => Some(t),
                false => None,
            },
            $( OptVec::$value(s) => s.get($i), )*
            OptVec::Vec(v) => v.get($i),
        }
    }   
}

macro_rules! get_mut_macro {
    ( $slf:ident, $i:ident, $( $value:ident ),* ) => {
        match $slf {
            OptVec::None => None,
            OptVec::One(t) => match $i == 0 {
                true => Some(t),
                false => None,
            },
            $( OptVec::$value(s) => s.get_mut($i), )*
            OptVec::Vec(v) => v.get_mut($i),
        }
    }   
}

impl<T> OptVec<T> {
    pub fn new() -> OptVec<T> {
        OptVec::None
    }
    pub fn len(&self) -> usize {
        self_macro!(len_macro,self)
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
                    OptVec::Two([t1,t2]) => *self = OptVec::Three([t1,t2,el]),
                    OptVec::Three([t1,t2,t3]) => *self = OptVec::Four([t1,t2,t3,el]),
                    OptVec::Four([t1,t2,t3,t4]) => *self = OptVec::Five([t1,t2,t3,t4,el]),
                    OptVec::Five([t1,t2,t3,t4,t5]) => *self = OptVec::Six([t1,t2,t3,t4,t5,el]),
                    OptVec::Six([t1,t2,t3,t4,t5,t6]) => *self = OptVec::Seven([t1,t2,t3,t4,t5,t6,el]),
                    OptVec::Seven([t1,t2,t3,t4,t5,t6,t7]) => *self = OptVec::Eight([t1,t2,t3,t4,t5,t6,t7,el]),
                    OptVec::Eight([t1,t2,t3,t4,t5,t6,t7,t8]) => *self = OptVec::Nine([t1,t2,t3,t4,t5,t6,t7,t8,el]),
                    OptVec::Nine([t1,t2,t3,t4,t5,t6,t7,t8,t9]) => *self = OptVec::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,el]),
                    OptVec::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,t10]) => *self = OptVec::Vec(vec![t1,t2,t3,t4,t5,t6,t7,t8,t9,t10,el]),
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
                    OptVec::Two([t1,q]) => {
                        *self = OptVec::One(t1);
                        Some(q)
                    },
                    OptVec::Three([t1,t2,q]) => {
                        *self = OptVec::Two([t1,t2]);
                        Some(q)
                    },
                    OptVec::Four([t1,t2,t3,q]) => {
                        *self = OptVec::Three([t1,t2,t3]);
                        Some(q)
                    },
                    OptVec::Five([t1,t2,t3,t4,q]) => {
                        *self = OptVec::Four([t1,t2,t3,t4]);
                        Some(q)
                    },
                    OptVec::Six([t1,t2,t3,t4,t5,q]) => {
                        *self = OptVec::Five([t1,t2,t3,t4,t5]);
                        Some(q)
                    },
                    OptVec::Seven([t1,t2,t3,t4,t5,t6,q]) => {
                        *self = OptVec::Six([t1,t2,t3,t4,t5,t6]);
                        Some(q)
                    },
                    OptVec::Eight([t1,t2,t3,t4,t5,t6,t7,q]) => {
                        *self = OptVec::Seven([t1,t2,t3,t4,t5,t6,t7]);
                        Some(q)
                    },
                    OptVec::Nine([t1,t2,t3,t4,t5,t6,t7,t8,q]) => {
                        *self = OptVec::Eight([t1,t2,t3,t4,t5,t6,t7,t8]);
                        Some(q)
                    },
                    OptVec::Ten([t1,t2,t3,t4,t5,t6,t7,t8,t9,q]) => {
                        *self = OptVec::Nine([t1,t2,t3,t4,t5,t6,t7,t8,t9]);
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
        let mut tmp = OptVec::None;
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
            OptVec::None => IntoIter::None,
            OptVec::One(t) => IntoIter::One(t), 
            $( OptVec::$value(s) => IntoIter::$value(s), )*
            OptVec::Vec(v) => IntoIter::Vec(v.into_iter()),
        }
    }   
}

impl<T> IntoIterator for OptVec<T> {
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
            OptVec::None => Iter::None,
            OptVec::One(t) => Iter::One(t),
            $( OptVec::$value(s) => Iter::Slice(s.iter()), )*
            OptVec::Vec(v) => Iter::Slice(v.iter()),
        }
    }   
}

impl<'t, T> IntoIterator for &'t OptVec<T> {
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
            OptVec::None => IterMut::None,
            OptVec::One(t) => IterMut::One(t),
            $( OptVec::$value(s) => IterMut::Slice(s.iter_mut()), )*
            OptVec::Vec(v) => IterMut::Slice(v.iter_mut()),
        }
    }   
}

impl<'t, T> IntoIterator for &'t mut OptVec<T> {
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
        let mut v = OptVec::new();
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
        let mut v = OptVec::new();
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
        let mut v = OptVec::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.into_iter().collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }

    #[test]
    fn basic_iter() {
        let mut v = OptVec::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.iter().map(|x|*x).collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }

    #[test]
    fn basic_iter_mut() {
        let mut v = OptVec::new();
        let r: Vec<usize> = (0 .. 15).into_iter().collect::<Vec<_>>();
        for i in &r {
            v.push(*i);
        }
        let lib_r = v.iter_mut().map(|x|*x).collect::<Vec<_>>();
        assert_eq!(lib_r,r)
    }
}
