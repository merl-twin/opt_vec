use std::{
    hash::Hash,
    collections::HashSet,
};


// effective size can be 6

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum OptHashSet<T: Hash + Eq> {
    None,
    One(T),
    Two([T; 2]),
    Three([T; 3]),
    Four([T; 4]),
    Five([T; 5]),
    Six([T; 6]),
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
                OptHashSet::Two([
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                ])
            },
            3 => {
                let mut it = ts.into_iter();
                OptHashSet::Three([
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                ])
            },
            4 => {
                let mut it = ts.into_iter();
                OptHashSet::Four([
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                ])
            },
            5 => {
                let mut it = ts.into_iter();
                OptHashSet::Five([
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                ])
            },
            6 => {
                let mut it = ts.into_iter();
                OptHashSet::Six([
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                    it.next().unwrap(), //safe
                ])
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
            OptHashSet::Three(_) => 3,
            OptHashSet::Four(_) => 4,
            OptHashSet::Five(_) => 5,
            OptHashSet::Six(_) => 6,
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
                            *self = OptHashSet::Three([t1,t2,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Two([t1,t2]);
                            false
                        }
                    },
                    OptHashSet::Three([t1,t2,t3]) => match (t1 != el)&&(t2 != el)&&(t3 != el) {
                        true => {
                            *self = OptHashSet::Four([t1,t2,t3,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Three([t1,t2,t3]);
                            false
                        }
                    },
                    OptHashSet::Four([t1,t2,t3,t4]) => match (t1 != el)&&(t2 != el)&&(t3 != el)&&(t4 != el) {
                        true => {
                            *self = OptHashSet::Five([t1,t2,t3,t4,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Four([t1,t2,t3,t4]);
                            false
                        }
                    },
                    OptHashSet::Five([t1,t2,t3,t4,t5]) => match (t1 != el)&&(t2 != el)&&(t3 != el)&&(t4 != el)&&(t5 != el) {
                        true => {
                            *self = OptHashSet::Six([t1,t2,t3,t4,t5,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Five([t1,t2,t3,t4,t5]);
                            false
                        }
                    },
                    OptHashSet::Six([t1,t2,t3,t4,t5,t6]) => match (t1 != el)&&(t2 != el)&&(t3 != el)&&(t4 != el)&&(t5 != el)&&(t6 != el) {
                        true => {
                            *self = OptHashSet::from(vec![t1,t2,t3,t4,t5,t6,el]);
                            true
                        },
                        false => {
                            *self =  OptHashSet::Six([t1,t2,t3,t4,t5,t6]);
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
            OptHashSet::Three([t1,t2,t3]) => (t1 == value)||(t2 == value)||(t3 == value),
            OptHashSet::Four([t1,t2,t3,t4]) => (t1 == value)||(t2 == value)||(t3 == value)||(t4 == value),
            OptHashSet::Five([t1,t2,t3,t4,t5]) => (t1 == value)||(t2 == value)||(t3 == value)||(t4 == value)||(t5 == value),
            OptHashSet::Six([t1,t2,t3,t4,t5,t6]) => (t1 == value)||(t2 == value)||(t3 == value)||(t4 == value)||(t5 == value)||(t6 == value),
            OptHashSet::Set(v) => v.contains(value),
        }
    }
    
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }

    pub fn intersection_size(&self, other: &OptHashSet<T>) -> usize {
        match (self,other) {
            (OptHashSet::None,_) |
            (_,OptHashSet::None) => 0,
            (OptHashSet::One(v1),OptHashSet::One(v2)) => match *v1 == *v2 {
                true => 1,
                false => 0,
            },
            (OptHashSet::Set(hv1),OptHashSet::Set(hv2)) => hv1.intersection(hv2).count(),
            (_,_) => {
                let mut cnt = 0;
                match other.len() > self.len() {
                    true => for v1 in self.iter() {
                        if other.contains(v1) { cnt += 1; }
                    },
                    false => for v2 in other.iter() {
                        if self.contains(v2) { cnt += 1; }
                    },
                }
                cnt
            },
        }
    }

    pub fn unite(self, other: OptHashSet<T>) -> OptHashSet<T> {
        match (self,other) {
            (OptHashSet::None,qs) | (qs,OptHashSet::None) => qs,
            (OptHashSet::One(v1),OptHashSet::One(v2)) => match v1 == v2 {
                true => OptHashSet::One(v1),
                false => OptHashSet::Two([v1,v2]),
            },
            (mut slf, mut oth) => match slf.len() > oth.len() {                
                true => {
                    slf.extend(oth);
                    slf
                },
                false => {
                    oth.extend(slf);
                    oth
                }
            },
        }
    }


    pub fn intersect(self, other: OptHashSet<T>) -> OptHashSet<T> {
        match (self,other) {
            (OptHashSet::None,_) |
            (_,OptHashSet::None) => OptHashSet::None,
            (OptHashSet::One(v1),OptHashSet::One(v2)) => match v1 == v2 {
                true => OptHashSet::One(v1),
                false => OptHashSet::None,
            },
            (OptHashSet::One(v),OptHashSet::Set(hv)) => match hv.contains(&v) {
                true => OptHashSet::One(v),
                false => OptHashSet::None,
            },
            (OptHashSet::Set(hv),OptHashSet::One(v)) => match hv.contains(&v) {
                true => OptHashSet::One(v),
                false => OptHashSet::None,
            },
            (hv1,hv2) => {
                let iter = hv1.into_iter().filter_map(|v1| match hv2.contains(&v1) {
                    true => Some(v1),
                    false => None,
                });
                let mut res = OptHashSet::None;
                res.extend(iter);
                res
            },
        }
    }

    pub fn filter_set<B, F>(self, mut f: F) -> OptHashSet<B>
    where F: FnMut(T) -> Option<B>,
          B: Eq + Hash
    {
        match self {
            OptHashSet::None => OptHashSet::None,
            OptHashSet::One(v) => match f(v) {
                None => OptHashSet::None,
                Some(b) => OptHashSet::One(b),
            },
            hs @ _ => {
                let mut res = OptHashSet::None;
                res.extend(hs.into_iter().filter_map(f));
                res
            },
        }
    }
}

impl<T: Eq + Hash + Clone> OptHashSet<T> {
    pub fn intersection(&self, other: &OptHashSet<T>) -> OptHashSet<T> {
        match (self,other) {
            (OptHashSet::None,_) |
            (_,OptHashSet::None) => OptHashSet::None,
            (OptHashSet::One(v1),OptHashSet::One(v2)) => match *v1 == *v2 {
                true => OptHashSet::One(v1.clone()),
                false => OptHashSet::None,
            },
            (OptHashSet::One(v),OptHashSet::Set(hv)) => match hv.contains(v) {
                true => OptHashSet::One(v.clone()),
                false => OptHashSet::None,
            },
            (OptHashSet::Set(hv),OptHashSet::One(v)) => match hv.contains(v) {
                true => OptHashSet::One(v.clone()),
                false => OptHashSet::None,
            },
            (hv1,hv2) => {
                let iter = hv1.iter().filter_map(|v1| match hv2.contains(&v1) {
                    true => Some(v1),
                    false => None,
                });
                let mut res = OptHashSet::None;
                res.extend(iter.cloned());
                res
            },
        }
    }
}




pub enum IntoIter<T> {
    None,
    One(T),
    Two([T;2]),
    Three([T; 3]),
    Four([T; 4]),
    Five([T; 5]),
    Six([T; 6]),
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
                    IntoIter::Two([q, t2]) => {
                        *self = IntoIter::One(t2);
                        Some(q)
                    },
                    IntoIter::Three([q, t2, t3]) => {
                        *self = IntoIter::Two([t2,t3]);
                        Some(q)
                    },
                    IntoIter::Four([q, t2, t3, t4]) => {
                        *self = IntoIter::Three([t2,t3,t4]);
                        Some(q)
                    },
                    IntoIter::Five([q, t2, t3, t4, t5]) => {
                        *self = IntoIter::Four([t2,t3,t4,t5]);
                        Some(q)
                    },
                    IntoIter::Six([q, t2, t3, t4, t5, t6]) => {
                        *self = IntoIter::Five([t2,t3,t4,t5,t6]);
                        Some(q)
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
            OptHashSet::Three(s) => IntoIter::Three(s),
            OptHashSet::Four(s) => IntoIter::Four(s),
            OptHashSet::Five(s) => IntoIter::Five(s),
            OptHashSet::Six(s) => IntoIter::Six(s),
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
            OptHashSet::Three(s) => Iter::Slice(s.iter()),
            OptHashSet::Four(s) => Iter::Slice(s.iter()),
            OptHashSet::Five(s) => Iter::Slice(s.iter()),
            OptHashSet::Six(s) => Iter::Slice(s.iter()),
            OptHashSet::Set(v) => Iter::Set(v.iter()),
        }
    }
}

