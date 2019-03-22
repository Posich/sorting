use std::iter::{IntoIterator};
use std::collections::VecDeque;

/// Basic structure storing data to be processed by a heap sorting algorithm.
/// At least I think that's what it's called.  I think I've read it being referred
/// to as node-tree sorting/binary tree sorting/etc.. before.  I'm not big into
/// technical lingo.
/// value: The actual data to be sorted
/// lesser: Optional pointer to a lower value
/// greater: Optional pointer to a higher value
/// Alternative, and probably better names for lesser and greater would be left and right,
/// because a real sorting algorithm wouldn't be biased about ordering.  Just went with
/// lesser and greater because this is for play and practice, so I wasn't thinking
/// about it at the time.
struct Element<T: Comparable> {
    value: T,
    lesser: Option<Box<Element<T>>>,
    greater: Option<Box<Element<T>>>,
}

//pub trait Comparable<T> {
//    fn compare(&self, other: &T) -> i64;
//}

/// Trait to implement comparisons between data types.  Defaults to same
/// data types, but could be used in theory on differing types.
/// given x.compare(y): if x >  y: returns a positive int,
///                     if x == y: returns 0,
///                     if x <  y: returns a negative int.
/// although this behavior can be changed if a different ordering is desired.
/// inspired by Java style collection sorting.
/// A proper implementation would likely make use of closures for fully
/// customisable behavior.
pub trait Comparable<OTH=Self> {
    fn compare(&self, other: &OTH) -> i64;
}

/// Test implementation of Comparable trait for i32.  Compare(i32) returns
/// 0 if equal, 1 if self is greater than other, and -1 if self is lesser.
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

/// The real meat of this program:  Experimenting to see if a trait could be applied
/// universally to collections.  In theory, any collection that can produce an iterator,
/// and be created from an iterator, should be compatible with this trait.  The only
/// remaining requirement would be that the actual data stored in the collection has
/// some sort of comparison logic implemented for it.
/// The FromIterator requirement is not necessary if you don't want to receive the
/// output in the same type of collection as the input.  That was just my personal
/// preference.  You could have it just return a sorted Iterator and do whatever you
/// feel like doing with it from there.
pub trait Sortable where Self: IntoIterator,
                         Self: std::iter::FromIterator<<Self as std::iter::IntoIterator>::Item>,
                         <Self as std::iter::IntoIterator>::Item: Comparable {

    /// Provided function should work with most collections.  Makes the trait pretty
    /// convenient to implement.
    /// Returns a result wrapping either a sorted version of the original collection,
    /// or a simple error string.
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

        let it_be_sorted = root.into_iter(); // <- this is where the actual sorting happens.

        Ok(Self::from_iter(it_be_sorted))
    }
}

// Old impl to test on Vec's
//impl<T> Sortable for Vec<T> where T: Comparable {}

/// This generic impl should give the Sortable trait to just about any collection
/// as long as Comparable is implemented for <T>.  Well, it works on Vec, anyway...
impl<T> Sortable for T where T: std::iter::IntoIterator,
                             T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
                             <T as std::iter::IntoIterator>::Item: Comparable, {}

/// Following the model of the standard library, this holds the data after sorting
/// as an Iterator.
struct IntoIter<T: Comparable> {
    values: VecDeque<T>,
    index: usize,
}

/// Iterator implementation to provide a sorted Iterator for output.
impl<T> Iterator for IntoIter<T> where T: Comparable {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = self.values.pop_front();
        self.index += 1;
        return retval;
    }
}

/// The implementation for the IntoIter struct, this is where the logic for the
/// final stage of the sorting algorithm lives.
/// An explanation for those that may not know:
/// This kind of sorting involves making a tree of structs, typically of a similar form
/// to the Element struct defined at the top of this file.
/// A simple one like that is composed of three parts.  A place to store the actual data
/// being processed, and two pointers to possibly more data, which would be contained
/// in the same type of struct.  This is very similar to the way a doubly linked list
/// is set up, but if you were to draw it on paper it would look like an ugly upside
/// down tree, rather than a straight line like a linked list would be.
/// Instead of a linear arrangement where the two pointers point the previous and next
/// structs, the pointers in the tree point to data placed there according to some
/// arbitrary comparison.
/// In the case of this program, the first element of a collection is stored as the root
/// of the tree.  The next element would be stored in either the 'lesser' or 'greater'
/// pointer depending on how it compared to the root element.  For the next element,
/// if the position it wants is already occupied, it gets compared against the value
/// that's occupying that position, and its position is determined from there.  This
/// happens for every element until they've all found a place on the tree.
/// If two elements are equal to eachother, then by convention the one that was encountered
/// first in the input should be listed first in the output, which happens automatically with
/// this particular sorting style.
impl<T> IntoIter<T> where T: Comparable {
    fn new(root: Element<T>) -> crate::heap_sort::IntoIter<T> {
        let mut it = IntoIter { values: VecDeque::new(), index: 0, };

        it.build_rec(root);

        return it;
    }

    /// A textbook implementation.  No, I'm not bragging, I remembered it from a textbook.
    /// Actually this is the kind of thing they would have you write to teach recursion.
    /// I'm sure using a loop or something would actually be a better way of doing it if
    /// this was for real real and not for play play, if only to avoid hitting the recursion
    /// limit.  It's been tested with a 1,000,000 element vector, but that's nothing compared
    /// to what a production ready program would be expected to handle.
    /// That said, I think doing it recursively makes it a little easier to read and
    /// visualise what's going on.  That and it's just the first thing I though of so I went
    /// with it because lazy
    /// This is taking the finished tree structure and returning it to a linear arrangement.
    fn build_rec(&mut self, elem: Element<T>) {
        // Check the 'lesser' value to see if it's pointing to anything.
        if let Some(val) = elem.lesser {
            // If so, the method calls itself again on that element.  Like a private
            // eye, it won't stop 'till it gets to the bottom of it.  Most programming
            // languages have some arbitrary limit on how many times you can do this, which
            // can vary by language/hardware.
            // That, plus the function call overhead probably means it's better to use a
            // loop instead.  As a side note, if it was just tail recursion, some compilers
            // actually detect that pattern and turn it into a loop for you with the right
            // optimisation flags.  GCC is known for doing stuff like that.
            &mut self.build_rec(*val);
        };

        // If you make it here, then this is the lowest value down that particular path.
        // So it gets added as the next(or first, as the case may be) element for output.
        &mut self.values.push_back(elem.value);

        // Here, we do the same thing, but starting from the 'greater' pointer of the element
        // we just processed.  This is where it gets a little harder to visualise what's going
        // on, and I can't really describe it without making some kind of cartoon or something
        // so you'll have to settle for stepping through the logic in your mind until you can
        // understand it or your neurons start to burn out.
        if let Some(val) = elem.greater {
            &mut self.build_rec(*val);
        };
    }
}

/// Nothing to see here, just a presumably standard implementation of IntoIterator.
impl<T> IntoIterator for Element<T> where T: Comparable {
    type Item = T;
    type IntoIter = crate::heap_sort::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        crate::heap_sort::IntoIter::new(self)
    }
}

/// The logic for building the Element tree.  No reason it's at the bottom, I just kept
/// adding the newer stuff above it for some reason.
impl<T> Element<T> where T: Comparable {
    /// Does what you think it does, instantiates a new Element struct of type T, with
    /// the provided value 'val', and returns it.
    fn new(val: T) -> Element<T> {
        Element { value: val, lesser: None, greater: None, }
    }

    /// Embarrassingly(because it's way at the bottom), this is where the first stage of
    /// sorting happens.  This is another recursive method because I just love 'em so
    /// much.  It takes a value, walks the branches of the tree, doing comparisons the
    /// whole way down, until it finds the right empty spot for it.  It works pretty much
    /// like the de-treeing method from before, just in the other direction.
    fn add(&mut self, val: T) {
        let comp = val.compare(&self.value);
        if comp < 0 {
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
}

// This sounds like a lot of work to me, but as I understand, the advantage to this type
// of sorting is that even with large data sets, the paths to find the correct location
// for any given value are relatively short.  Instead of iterating over the entire array
// almost every time, which is the approach damn near everyone took on their first time
// being asked to write a sorting program(don't lie, you know you did), this method
// manages to bypass huge amounts of data during the comparison phase.
// Another advantage is that it actually works better the more disorganised the data is
// initially.  Of course, that also means it loses more of its benefits the more organised
// the input is, becoming closer and closer to just being a regular old linked list, at which
// point it's easily beaten by any kind of algorithm that operates directly on an array.
// Including the aforementioned method of iterating over the entire array start to finish.
// Because that's what it would be doing, more or less, and accessing arrays is waaay faster
// than seeking through a linked list.