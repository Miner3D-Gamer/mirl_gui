//! A vec that can have multiple indexes point at the same value to save on memory
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A vec that can have multiple indexes point at the same value
pub struct DeduplicatedVec<T> {
    /// The held values
    _values: Vec<T>,
    /// Maps the direct indexes to the objects below
    _references: Vec<usize>,
    /// Maps the objects to the references above
    _reverse_references: Vec<Vec<usize>>,
}
impl<T> Default for DeduplicatedVec<T> {
    fn default() -> Self {
        Self {
            _values: Vec::default(),
            _references: Vec::default(),
            _reverse_references: Vec::default(),
        }
    }
}
impl<T> ListLikeIter<T, usize> for DeduplicatedVec<T> {
    type Iterator<'a>
        = DeduplicatedVecIter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        DeduplicatedVecIter {
            _values: &self._values,
            _references: self._references.iter(),
        }
    }
}
// I wrote this impl with 32° on my thermometer so I am not promising that any of this code is safe
impl<T> ListLike<T, usize> for DeduplicatedVec<T> {
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe {
            self._values
                .get_unchecked(*self._references.get_unchecked(index))
        }
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe {
            self._values
                .get_unchecked_mut(*self._references.get_unchecked(index))
        }
    }

    fn push_mut(&mut self, value: T) -> &mut T {
        self._references.push(self._values.len() + 1);
        let r = self._values.push_mut(value);
        self._reverse_references.push(vec![self._references.len()]);
        r
    }
    /// Try to remove the value
    ///
    /// Keep in mind that returning None does not only mean that the index is out of range, it only returns T when no further references reference T
    fn try_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        unsafe {
            let idx = *self._references.get_unchecked(index);

            let reverse_ref = self._reverse_references.get_unchecked_mut(idx);
            reverse_ref.delete(&index).unwrap_unchecked();

            let _ = self._reverse_references.iter_mut().map(|x| {
                x.iter_mut().map(|y| {
                    if *y > index {
                        *y -= 1;
                    }
                })
            });

            self._references.try_remove(index).unwrap_unchecked();
            if self._reverse_references.get_unchecked(idx).is_empty() {
                // TODO: Is this off by 1? (index)
                let _ = self._references.iter_mut().skip(index).map(|x| *x -= 1);
                self._reverse_references.try_remove(idx).unwrap_unchecked();
                let value = self._values.try_remove(idx).unwrap_unchecked();
                Some(value)
            } else {
                None
            }
        }
    }

    fn swap_values(&mut self, a: usize, b: usize) -> bool {
        if a >= self.len() || b >= self.len() {
            return false;
        }
        let idx_a = self._references[a];
        let idx_b = self._references[b];
        unsafe {
            self._reverse_references[idx_a]
                .delete(&a)
                .unwrap_unchecked();
            self._reverse_references[idx_b]
                .delete(&b)
                .unwrap_unchecked();

            self._references.swap_unchecked(a, b);
        }
        self._reverse_references[idx_a].push(b);
        self._reverse_references[idx_b].push(a);
        true
    }

    fn len(&self) -> usize {
        self._references.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.try_remove(self.len())
    }

    fn try_insert_mut(&mut self, index: usize, value: T) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        self._references.insert(index, self._values.len() + 1);
        let r = self._values.push_mut(value);
        self._reverse_references.push(vec![index]);
        // TODO: Is this off by 1? (index)
        let _ = self._reverse_references.iter_mut().map(|x| {
            x.iter_mut().map(|y| {
                if *y > index {
                    *y += 1;
                }
            })
        });
        Some(r)
    }

    fn try_replace(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        unsafe {
            let idx = self._references.get_unchecked_mut(index);
            let reverse_ref = self._reverse_references.get_unchecked_mut(*idx);
            if reverse_ref.len() == 1 {
                return Some(self._values.try_replace(*idx, value).unwrap_unchecked());
            }
            reverse_ref.delete(idx).unwrap_unchecked();

            self._values.push(value);
            self._reverse_references.push(vec![*idx]);
            self._references
                .try_replace(index, self._values.len())
                .unwrap_unchecked();

            None
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self._references.try_reserve(additional)
    }

    fn find_position(&self, item: &T) -> Option<usize>
    where
        T: std::cmp::PartialEq,
    {
        let idx = self._values.find_position(item)?;
        Some(unsafe { self._reverse_references.get_unchecked(idx)[0] })
    }

    fn clear(&mut self) {
        self._references.clear();
        self._reverse_references.clear();
        self._values.clear();
    }
}
impl<T: PartialEq> DeduplicatedVec<T> {
    /// Push the value on the vec based on the configuration
    pub fn push_configured(&mut self, item: T, configuration: Deduplication) {
        match configuration {
            Deduplication::Deduplicate => self.push_deduplicated(item),
            Deduplication::Simple => self.push(item),
        }
    }
    /// Push the value on the vec if it doesn't exist yet, otherwise create a reference
    pub fn push_deduplicated(&mut self, item: T) {
        if let Some(pos) = self._values.find_position(&item) {
            self._references.push(pos);
            let reverse_ref = unsafe { self._reverse_references.get_unchecked_mut(pos) };
            reverse_ref.push(self._references.len());
        } else {
            self.push(item);
        }
    }
}

impl<T> std::ops::Index<usize> for DeduplicatedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for DeduplicatedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<T> DeduplicatedVec<T> {
    fn iter(&self) -> DeduplicatedVecIter<'_, T> {
        <&Self as IntoIterator>::into_iter(self)
    }
}
impl<'a, T> IntoIterator for &'a DeduplicatedVec<T> {
    type Item = &'a T;
    type IntoIter = DeduplicatedVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for DeduplicatedVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self._values.into_iter()
    }
}

#[derive(Debug, Clone, Default)]
/// An iter over [`DeduplicatedVec`] that doesn't retain insertion order
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct DeduplicatedVecIter<'a, T> {
    /// References to the values
    pub _values: &'a [T],
    /// References to the references
    pub _references: std::slice::Iter<'a, usize>,
}

impl<'a, T> Iterator for DeduplicatedVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: The references never point to an invalid index
        Some(unsafe { self._values.get_unchecked(*self._references.next()?) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self._references.size_hint()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// How things should be inserted in the asset server
pub enum Deduplication {
    /// Simply insert it without doing anything else
    Simple,
    /// Check if any other type of the same module type has the same image.
    ///
    /// If so, create a reference to it (More memory efficient)
    Deduplicate,
}
