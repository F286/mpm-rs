use std::mem::MaybeUninit;

pub struct FixedVec<T, const N: usize> {
    len: usize,
    data: [MaybeUninit<T>; N],
}

impl<T, const N: usize> FixedVec<T, N> {
    pub const fn new() -> Self {
        // SAFETY: an uninitialized `[MaybeUninit<_>; N]` is valid
        Self { len: 0, data: unsafe { MaybeUninit::uninit().assume_init() } }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn capacity(&self) -> usize { N }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            self.data[self.len].write(value);
            self.len += 1;
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            // SAFETY: index checked
            Some(unsafe { &*self.data[idx].as_ptr() })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            // SAFETY: index checked
            Some(unsafe { &mut *self.data[idx].as_mut_ptr() })
        } else {
            None
        }
    }

    pub fn swap_remove(&mut self, idx: usize) -> Option<T> {
        if idx < self.len {
            // SAFETY: idx checked
            let value = unsafe { self.data[idx].assume_init_read() };
            self.len -= 1;
            if idx != self.len {
                let last = unsafe { self.data[self.len].assume_init_read() };
                self.data[idx].write(last);
            }
            Some(value)
        } else {
            None
        }
    }

    pub fn iter(&self) -> FixedVecIter<'_, T, N> {
        FixedVecIter { vec: self, index: 0 }
    }

    pub fn iter_mut(&mut self) -> FixedVecIterMut<'_, T, N> {
        FixedVecIterMut { vec: self, index: 0 }
    }
}

impl<T, const N: usize> Drop for FixedVec<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe { self.data[i].assume_init_drop(); }
        }
    }
}

pub struct FixedVecIter<'a, T, const N: usize> {
    vec: &'a FixedVec<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for FixedVecIter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = self.vec.get(self.index);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

pub struct FixedVecIterMut<'a, T, const N: usize> {
    vec: &'a mut FixedVec<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for FixedVecIterMut<'a, T, N> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let idx = self.index;
            self.index += 1;
            // SAFETY: index checked and mutable iteration guarantees unique access
            Some(unsafe { &mut *self.vec.data[idx].as_mut_ptr() })
        } else {
            None
        }
    }
}

