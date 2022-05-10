use vec::Uniq;

/// TODO: Documentation for SortedUniq trait
pub trait SortedUniq<T>: Uniq<T> {
    /// TODO
    fn uniq(&self, other: Self) -> Self;
    /// TODO: add documentation
    fn unique(&self) -> Self;
    /// TODO: add documentation
    fn is_unique(&self) -> bool;
    /// TODO: add documentation
    fn uniq_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(
        &self,
        other: Self,
        eq: F,
        ord: K,
    ) -> Self;
    /// TODO: add documentation
    fn unique_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(&self, eq: F, ord: K) -> Self;
    /// TODO: add documentation
    fn is_unique_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(&self, eq: F, ord: K) -> bool;
}

impl<T: Copy + PartialEq + PartialOrd> SortedUniq<T> for Vec<T> {
    fn uniq(&self, other: Vec<T>) -> Vec<T> {
        SortedUniq::<T>::uniq_via(self, other, |l, r| l == r, |l, r| l < r)
    }

    fn unique(&self) -> Vec<T> {
        SortedUniq::<T>::unique_via(self, |l, r| l == r, |l, r| l < r)
    }

    fn is_unique(&self) -> bool {
        SortedUniq::<T>::is_unique_via(self, |l, r| l == r, |l, r| l < r)
    }

    fn uniq_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(
        &self,
        other: Self,
        eq: F,
        ord: K,
    ) -> Vec<T> {
        let mut out = self.clone();
        let mut cursor: usize = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < out.len() && j < other.len() {
            if ord(&other[j], &out[i]) {
                // generally: other[j] < out[i]
                j += 1;
            } else if eq(&other[j], &out[i]) {
                i += 1;
                j += 1;
            } else {
                if cursor == 0 || out[cursor - 1] != out[i] {
                    out[cursor] = out[i];
                    cursor += 1;
                }
                i += 1;
            }
        }
        while i < out.len() {
            if cursor == 0 || out[cursor - 1] != out[i] {
                out[cursor] = out[i];
                cursor += 1;
            }
            i += 1;
        }
        out.resize(cursor, out[0]);
        out
    }

    fn unique_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(&self, eq: F, ord: K) -> Vec<T> {
        self.clone()
    }

    fn is_unique_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(&self, eq: F, ord: K) -> bool {
        false
    }
}
