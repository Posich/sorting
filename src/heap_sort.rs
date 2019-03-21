use std::iter::{IntoIterator};
use std::collections::VecDeque;

struct Element<T: Comparable> {
    value: T,
    lesser: Option<Box<Element<T>>>,
    greater: Option<Box<Element<T>>>,
}

//pub trait Comparable<T> {
//    fn compare(&self, other: &T) -> i64;
//}

pub trait Comparable<OTH=Self> {
    fn compare(&self, other: &OTH) -> i64;
}

impl Comparable for i32 {
    fn compare(&self, other: &i32) -> i64 {
        if self == other {
            0
        } else if self > other {
            1
        } else {
            -1
        }
    }
}

pub trait Sortable where Self: IntoIterator,
                         Self: std::iter::FromIterator<<Self as std::iter::IntoIterator>::Item>,
                         <Self as std::iter::IntoIterator>::Item: Comparable {

    fn heap_sort(self) -> Result<Self, &'static str> {
        let mut it = self.into_iter();
        let mut root: Element<<Self as std::iter::IntoIterator>::Item>;
        match it.next() {
            Some(first) => root = Element::new(first),
            None => return Err("Empty list!"),
        }

        loop {
            match it.next() {
                Some(val) => root.add(val),
                None => break,
            }
        }

        let it_be_sorted = root.into_iter();

        Ok(Self::from_iter(it_be_sorted))
    }
}

impl<T> Sortable for Vec<T> where T: Comparable {}

struct IntoIter<T: Comparable> {
    values: VecDeque<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> where T: Comparable {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = self.values.pop_front();
        self.index += 1;
        return retval;
    }
}

impl<T> IntoIter<T> where T: Comparable {
    fn new(root: Element<T>) -> crate::heap_sort::IntoIter<T> {
        let mut it = IntoIter { values: VecDeque::new(), index: 0, };

        it.build_rec(root);

        return it;
    }

    fn build_rec(&mut self, elem: Element<T>) {
        if let Some(val) = elem.lesser {
            &mut self.build_rec(*val);
        };

        &mut self.values.push_back(elem.value);

        if let Some(val) = elem.greater {
            &mut self.build_rec(*val);
        };
    }
}

impl<T> IntoIterator for Element<T> where T: Comparable {
    type Item = T;
    type IntoIter = crate::heap_sort::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        crate::heap_sort::IntoIter::new(self)
    }
}

impl<T> Element<T> where T: Comparable {
    fn new(val: T) -> Element<T> {
        Element { value: val, lesser: None, greater: None, }
    }

    fn add(&mut self, val: T) {
        let comp = val.compare(&self.value);
        if comp == -1 {
            match &mut self.lesser {
                Some(less) => less.add(val),
                None => self.lesser = Some(Box::new(Element::new(val))),
            }
        } else {
            match &mut self.greater {
                Some(more) => more.add(val),
                None => self.greater = Some(Box::new(Element::new(val))),
            }
        }
    }

    fn greatest(&self) -> &Element<T> {
        match &self.greater {
            Some(more) => { more.greatest() },
            None => &self,
        }
    }

    fn leastest(&self) -> &Element<T> {
        match &self.lesser {
            Some(less) => { less.leastest() },
            None => &self,
        }
    }
}

