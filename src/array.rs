use std::marker::PhantomData;

use crate::SV;
use crate::convert::{FromSV, TryFromSV};
use crate::handle::Owned;
use crate::raw;
use crate::raw::SSize_t;

/// Perl array object.
pub struct AV(Owned<raw::AV>);

impl AV {
    fn pthx(&self) -> raw::Interpreter {
        self.0.pthx()
    }

    fn as_ptr(&self) -> *mut raw::AV {
        self.0.as_ptr()
    }

    method! {
        /// Empty the array, preserving underlying allocation.
        ///
        /// See [`av_clear`](http://perldoc.perl.org/perlapi.html#av_clear).
        simple fn clear() = av_clear()
    }
    method! {
        /// Delete element of array and return it.
        ///
        /// See [`av_delete`](http://perldoc.perl.org/perlapi.html#av_delete).
        getter fn delete(key: SSize_t) = av_delete(key, 0)
    }
    method! {
        /// Delete element of array without returning it.
        ///
        /// See [`av_delete`](http://perldoc.perl.org/perlapi.html#av_delete).
        simple fn discard(key: SSize_t) = av_delete(key, raw::G_DISCARD as raw::I32)
    }
    method! {
        /// Return true when element exists in the array.
        ///
        /// See [`av_exists`](http://perldoc.perl.org/perlapi.html#av_exists).
        simple fn exists(key: SSize_t) -> bool = av_exists(key) != 0
    }
    method! {
        /// Grow array to contain at least `key` elements.
        ///
        /// See [`av_extend`](http://perldoc.perl.org/perlapi.html#av_extend).
        simple fn extend(key: SSize_t) = av_extend(key)
    }
    method! {
        /// Fetch element from array.
        ///
        /// See [`av_fetch`](http://perldoc.perl.org/perlapi.html#av_fetch).
        getptr fn fetch(key: SSize_t) = av_fetch(key, 0)
    }
    method! {
        /// Fetch element from array ready for assignment.
        ///
        /// See [`av_fetch`](http://perldoc.perl.org/perlapi.html#av_fetch).
        getptr fn fetch_lvalue(key: SSize_t) = av_fetch(key, 1)
    }
    method! {
        /// Set array length to `fill + 1`.
        ///
        /// See [`av_fill`](http://perldoc.perl.org/perlapi.html#av_fill).
        simple fn fill(fill: SSize_t) = av_fill(fill)
    }
    method! {
        /// Return the highest index in the array.
        ///
        /// See [`av_top_index`](http://perldoc.perl.org/perlapi.html#av_top_index).
        simple fn top_index() -> SSize_t = av_len()
    }
    method! {
        /// Remove and return one element at the end of the array.
        ///
        /// See [`av_pop`](http://perldoc.perl.org/perlapi.html#av_pop).
        getter fn pop() = av_pop()
    }
    method! {
        /// Append new element to the array.
        ///
        /// See [`av_push`](http://perldoc.perl.org/perlapi.html#av_push).
        simple fn push(val: SV) = av_push(val.into_raw())
    }
    method! {
        /// Remove and return one element at the beginning of the array.
        ///
        /// See [`av_shift`](http://perldoc.perl.org/perlapi.html#av_shift).
        getter fn shift() = av_shift()
    }
    method! {
        ///  Clear the array and release array memory.
        ///
        /// See [`av_undef`](http://perldoc.perl.org/perlapi.html#av_undef).
        simple fn undef() = av_undef()
    }
    method! {
        /// Insert one or more `undef` elements at the beginning of the array.
        ///
        /// See [`av_unshift`](http://perldoc.perl.org/perlapi.html#av_unshift).
        simple fn unshift(num: SSize_t) = av_unshift(num)
    }

    /// Set array element at index `key` to `val`.
    ///
    /// See [`av_store`](http://perldoc.perl.org/perlapi.html#av_store).
    #[inline]
    pub fn store(&self, key: SSize_t, val: SV) {
        unsafe {
            let raw = val.into_raw();
            let svpp = self.pthx().av_store(self.as_ptr(), key, raw);
            if svpp.is_null() {
                self.pthx().ouroboros_sv_refcnt_dec(raw)
            }
        }
    }

    /// Construct new AV from a raw pointer without incrementing reference counter (raw pointer
    /// already "owns" one incref).
    #[inline]
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_owned(pthx, raw))
    }

    /// Construct new AV from a raw pointer and increment its reference counter (raw pointer is
    /// "borrowed" from another structure that owns one incref).
    #[inline]
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_borrowed(pthx, raw))
    }

    /// Get an iterator over the array.
    #[inline]
    pub fn iter<T: FromSV>(&self) -> IterAV<T> {
        IterAV::new(self)
    }
}

impl TryFromSV for AV {
    type Error = &'static str;
    unsafe fn try_from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Result<AV, Self::Error> {
        if pthx.ouroboros_sv_rok(raw) == 0 {
            return Err("not an array reference");
        }

        Ok(AV::from_raw_borrowed(
            pthx,
            pthx.ouroboros_sv_rv(raw) as *mut _,
        ))
    }
}

pub struct IterAV<'a, T> {
    av: &'a AV,
    pos: SSize_t,
    top: SSize_t,
    ty: PhantomData<T>,
}

impl<'a, T> IterAV<'a, T> {
    fn new(av: &'a AV) -> Self {
        IterAV {
            pos: 0,
            top: av.top_index(),
            av: av,
            ty: PhantomData,
        }
    }
}

impl<'a, T: FromSV> Iterator for IterAV<'a, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= self.top {
            let v = self.av.fetch(self.pos);
            self.pos += 1;
            Some(v)
        } else {
            None
        }
    }
}
