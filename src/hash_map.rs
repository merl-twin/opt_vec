use std::{
    hash::Hash,
    collections::HashMap,
};


#[derive(Debug,Clone,PartialEq,Eq)]
pub enum OptHashMap<T: Hash + Eq, V> {
    None,
    One((T, V)),
    Two([(T,V); 2]),
    Map(HashMap<T,V>),
}
impl<T: Hash + Eq,V> From<(T,V)> for OptHashMap<T,V> {
    fn from(p: (T,V)) -> OptHashMap<T,V> {
        OptHashMap::One(p)
    }
}
impl<T: Hash + Eq,V> From<Vec<(T,V)>> for OptHashMap<T,V> {
    fn from(ts: Vec<(T,V)>) -> OptHashMap<T,V> {
        let ts = ts.into_iter().collect::<HashMap<_,_>>();
        OptHashMap::from(ts)
    }
}
impl<T: Hash + Eq,V> From<HashMap<T,V>> for OptHashMap<T,V> {
    fn from(ts: HashMap<T,V>) -> OptHashMap<T,V> {        
        match ts.len() {
            0 => OptHashMap::None,
            1 => OptHashMap::One(ts.into_iter().next().unwrap()), // safe
            2 => {
                let mut it = ts.into_iter();
                let v1 = it.next().unwrap(); //safe
                let v2 = it.next().unwrap(); // safe
                OptHashMap::Two([v1, v2])
            },
            _ => OptHashMap::Map(ts),
        }
    }
}

impl<T: Hash + Eq,V> Extend<(T,V)> for OptHashMap<T,V> {
    fn extend<I: IntoIterator<Item = (T,V)>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        loop {
            match self {
                OptHashMap::Map(v) => {
                    v.extend(iter);
                    break;
                },
                _ => match iter.next() {
                    None => break,
                    Some((t,v)) => { self.insert(t,v); },
                },
            }
        }
    }
}

impl<T: Hash + Eq,V> FromIterator<(T,V)> for OptHashMap<T,V> {
    fn from_iter<I: IntoIterator<Item = (T,V)>>(iter: I) -> OptHashMap<T,V> {
        let mut slf = OptHashMap::None;
        slf.extend(iter);
        slf
    }
}

impl<T: Hash + Eq,V> OptHashMap<T,V> {
    pub fn new() -> OptHashMap<T,V> {
        OptHashMap::None
    }
    pub fn len(&self) -> usize {
        match self {
            OptHashMap::None => 0,
            OptHashMap::One(_) => 1,
            OptHashMap::Two(_) => 2,
            OptHashMap::Map(v) => v.len(),
        }
    }
    pub fn insert(&mut self, el: T, v: V) -> Option<V> { // some if already present
        match self {
            OptHashMap::None => {
                *self = OptHashMap::One((el,v));
                None
            },
            OptHashMap::Map(m) => m.insert(el,v),
            _ => {
                let mut tmp = OptHashMap::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    OptHashMap::None |
                    OptHashMap::Map(_) => unreachable!(),
                    OptHashMap::One((t,old_v)) => match t != el {
                        true => {
                            *self = OptHashMap::Two([(t,old_v),(el,v)]);
                            None
                        },
                        false => {
                            *self = OptHashMap::One((t,v));
                            Some(old_v)
                        }
                    },
                    OptHashMap::Two([(t1,old1),(t2,old2)]) => match (t1 == el, t2 == el) {
                        (false,false) => {
                            *self = OptHashMap::from(vec![(t1,old1),(t2,old2),(el,v)]);
                            None                            
                        }
                        (true,false) => {
                            *self =  OptHashMap::Two([(t1,v),(t2,old2)]);
                            Some(old1)
                        },
                        (false,true) => {
                            *self =  OptHashMap::Two([(t1,old1),(t2,v)]);
                            Some(old2)
                        },
                        (true,true) => unreachable!(), // can't be                                                       
                    },
                }
            },           
        }
    }

    pub fn contains_key(&self, value: &T) -> bool {
        match self {
            OptHashMap::None => false,
            OptHashMap::One((t,_)) => t == value,
            OptHashMap::Two([(t1,_),(t2,_)]) => (t1 == value)||(t2 == value),
            OptHashMap::Map(v) => v.contains_key(value),
        }
    }
    
    pub fn iter(&self) -> Iter<T,V> {
        self.into_iter()
    }
}




pub enum IntoIter<T,V> {
    None,
    One((T,V)),
    Two([(T,V);2]),
    Map(std::collections::hash_map::IntoIter<T,V>),
}
impl<T,V> Iterator for IntoIter<T,V> {
    type Item = (T,V);

    fn next(&mut self) -> Option<(T,V)> {
        match self {
            IntoIter::None => None,
            IntoIter::Map(v) => v.next(),
            _ => {
                let mut tmp = IntoIter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    IntoIter::None |
                    IntoIter::Map(_) => unreachable!(),
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

impl<T: Hash + Eq,V> IntoIterator for OptHashMap<T,V> {
    type Item = (T,V);
    type IntoIter = IntoIter<T,V>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptHashMap::None => IntoIter::None,
            OptHashMap::One(t) => IntoIter::One(t),
            OptHashMap::Two(s) => IntoIter::Two(s),
            OptHashMap::Map(v) => IntoIter::Map(v.into_iter()),
        }
    }
}


pub enum Iter<'t,T,V> {
    None,
    One((&'t T, &'t V)),
    Two([(&'t T, &'t V); 2]),
    Map(std::collections::hash_map::Iter<'t,T,V>),
}
impl<'t,T,V> Iterator for Iter<'t,T,V> {
    type Item = (&'t T, &'t V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::None => None,
            Iter::Map(s) => s.next(),
            _ => {
                let mut tmp = Iter::None;
                std::mem::swap(&mut tmp, self);
                match tmp {
                    Iter::None |
                    Iter::Map(_) => unreachable!(),
                    Iter::One(t) => Some(t),
                    Iter::Two([t1,t2]) => {
                        *self = Iter::One(t2);
                        Some(t1)
                    },
                }
            },
        }
    }
}

pub struct TupleMapper<'t,T,V>(&'t (T,V));


impl<'t, T: Hash + Eq,V> IntoIterator for &'t OptHashMap<T,V> {
    type Item = (&'t T, &'t V);
    type IntoIter = Iter<'t,T,V>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OptHashMap::None => Iter::None,
            OptHashMap::One(t) => Iter::One((&t.0,&t.1)),
            OptHashMap::Two(s) => Iter::Two([(&s[0].0,&s[0].1),(&s[1].0,&s[1].1)]),
            OptHashMap::Map(v) => Iter::Map(v.iter()),
        }
    }
}


