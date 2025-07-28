use std::{mem, ops::Deref, path::PathBuf};

/// An extension trait to clear duplicates from a collection.
pub(crate) trait Dedup<T: PartialEq> {
    fn clear_duplicates(&mut self);
}

/// Clear duplicates from a collection, keep the first one seen.
///
/// For small vectors, this will be faster than a `HashSet`.
impl<T: PartialEq> Dedup<T> for Vec<T> {
    fn clear_duplicates(&mut self) {
        let orig = mem::replace(self, Vec::with_capacity(self.len()));
        for item in orig {
            if !self.contains(&item) {
                self.push(item);
            }
        }
    }
}

/// Like `str::find`, but starts searching at `start`.
pub(crate) trait FindFrom {
    fn find_from(&self, needle: &Self, start: usize) -> Option<usize>;
}

impl FindFrom for str {
    fn find_from(&self, needle: &Self, start: usize) -> Option<usize> {
        self.get(start..)
            .and_then(|s| s.find(needle))
            .map(|i| i + start)
    }
}

/// Add the `with` method to `PathBuf` which can be used to temporarily push components and
/// automatically pop them at the end of the scope.
pub(crate) trait PathBufExt {
    fn with<'a, const N: usize>(&'a mut self, components: [&str; N]) -> PathBufWith<'a, N>;
}

impl PathBufExt for PathBuf {
    fn with<'a, const N: usize>(&'a mut self, components: [&str; N]) -> PathBufWith<'a, N> {
        for comp in components {
            self.push(comp);
        }
        PathBufWith { buf: self }
    }
}

pub(crate) struct PathBufWith<'a, const N: usize> {
    buf: &'a mut PathBuf,
}

impl<const N: usize> Drop for PathBufWith<'_, N> {
    fn drop(&mut self) {
        for _ in 0..N {
            self.buf.pop();
        }
    }
}

impl<const N: usize> PathBufWith<'_, N> {
    pub fn take(self) -> PathBuf {
        mem::take(self.buf)
    }
}

impl<const N: usize> Deref for PathBufWith<'_, N> {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        self.buf
    }
}
