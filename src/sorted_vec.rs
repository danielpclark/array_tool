use vec::Uniq;

/// Collection of methods for getting or evaluating uniqueness, assuming some
/// kind of sorted-ness
pub trait SortedUniq<T>: Uniq<T> {
    /// `uniq` returns a vector of unique values within itself as compared to the
    /// other vector, which is provided as an input parameter. Both of these must
    /// have non-decreasing values.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1,2,3,4,5,6].uniq(vec![1,2,5,7,9]);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![3,4,6]
    /// ```
    fn uniq(&self, other: Self) -> Self;

    /// `unique` returns a vector like Self but with all duplicated elements
    /// removed. Must have non-decreasing values.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1,1,1,2,3,3,4,5,6].unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,3,4,5,6]
    /// ```
    fn unique(&self) -> Self;

    /// `is_unique` returns boolean value on whether all values within Self are
    /// unique. Self must be sorted in some way.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1,2,4,6,6,7,8].is_unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// false
    /// ```
    fn is_unique(&self) -> bool;

    /// `uniq_via` returns a vector of unique values within itself as compared to
    /// the other vector which is provided as an input parameter, as defined by
    /// the two provided custom comparators. Both vectors must be sorted, and the
    /// sort direction is determined by the second comparator: l < r for increasing.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1,2,3,4,5,6].uniq_via(
    ///     vec![1,2,5,7,9],
    ///     |l, r| l == r,
    ///     |l, r| l < r
    /// );
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![3, 4, 6]
    /// ```
    fn uniq_via<F: Fn(&T, &T) -> bool, K: Fn(&T, &T) -> bool>(
        &self,
        other: Self,
        eq: F,
        ord: K,
    ) -> Self;

    /// `unique_via` removes duplicates from within the vector and returns Self.
    /// Self must be a vector sorted in some way.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1,2,3,3,4,5,6].unique_via(|l, r| l == r);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,3,4,5,6]
    /// ```
    fn unique_via<F: Fn(&T, &T) -> bool>(&self, eq: F) -> Self;

    /// `is_unique_via` returns boolean value on whether all values within
    /// Self are unique, as defined by a provided custom comparator. Self must
    /// be a vector sorted in some way.
    ///
    /// # Example
    /// ```
    /// use array_tool::sorted_vec::SortedUniq;
    ///
    /// vec![1.0,2.0,2.4,3.3,3.1,3.5,4.6,5.2,6.2].is_unique_via( |l: &f64, r: &f64| l.floor() == r.floor() );
    /// ```
    ///
    /// # Output
    /// ```text
    /// false
    /// ```
    fn is_unique_via<F: Fn(&T, &T) -> bool>(&self, eq: F) -> bool;
}

impl<T: Copy + PartialEq + PartialOrd> SortedUniq<T> for Vec<T> {
    fn uniq(&self, other: Vec<T>) -> Vec<T> {
        SortedUniq::<T>::uniq_via(self, other, |l, r| l == r, |l, r| l < r)
    }

    fn unique(&self) -> Vec<T> {
        SortedUniq::<T>::unique_via(self, |l, r| l == r)
    }

    fn is_unique(&self) -> bool {
        SortedUniq::<T>::is_unique_via(self, |l, r| l == r)
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
                // ^ should other[j] be ordered before out[i]?
                j += 1;
            } else if eq(&other[j], &out[i]) {
                i += 1;
                j += 1;
            } else {
                if i == 0 || !eq(&out[i - 1], &out[i]) {
                    out[cursor] = out[i];
                    cursor += 1;
                }
                i += 1;
            }
        }
        while i < out.len() {
            if i == 0 || !eq(&out[i - 1], &out[i]) {
                out[cursor] = out[i];
                cursor += 1;
            }
            i += 1;
        }
        out.truncate(cursor);
        out
    }
    fn unique_via<F: Fn(&T, &T) -> bool>(&self, eq: F) -> Vec<T> {
        let mut out = self.clone();
        let mut cursor: usize = 1;
        for i in 1..out.len() {
            if !eq(&out[i], &out[i - 1]) {
                if i != cursor {
                    out[cursor] = out[i];
                }
                cursor += 1;
            }
        }
        out.truncate(cursor);
        out
    }

    fn is_unique_via<F: Fn(&T, &T) -> bool>(&self, eq: F) -> bool {
        for i in 1..self.len() {
            if eq(&self[i], &self[i - 1]) {
                return false;
            }
        }
        true
    }
}
