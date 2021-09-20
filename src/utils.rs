/// Euclidian distance squared between colors
///
/// Uses the 3-dimensional euclidian distance formula to get the distance
/// between two colors. This function returns an [`i32`] because the
/// intermediate calculations would overflow [`u8`]. It is guaranteed to not
/// overflow for slices with a length at most 33,025, but if you reach this
/// limit, I guarantee you're doing something wrong.
///
/// ```
/// # use image_go_nord::utils::delta;
///
/// let a = [173, 87, 119, 255]; // a pinkish color
/// let b = [0, 255, 255]; // cyan, my favorite color
///
/// // `a` is rbga, but it doesn't matter. `delta` will only take components up to the length of
/// //the smallest slice
/// assert_eq!(delta(&a, &b), 76649);
/// ```
///
/// `delta` takes two slices and zips them to calculate their component-wise
/// distance. This is safe because it will only take elements up to the last of
/// the shortest slice, which usually comes from a palette color (which is an
/// array).
///
/// This function can be proven not to overflow with some simple set theory. If
/// $d$ is the function `delta`, $B \subset \Z^+$ is the set of unsigned 8-bit
/// integers (`u8`), and $I \subset \Z$ is the set of signed 32-bit integers,
/// and we know $B \subset I$, then we can prove, for two arbitrary 8-bit
/// subpixel colors $x = (r_1, g_1, b_1)$ and $y = (r_2, g_2, b_2)$ (where $r
/// \in B \and g \in B \and b \in B$), that $\forall x \forall y, d(x, y) \in
/// I$.
///
/// $d$ can be defined as follows (where $n = min(|x|, |y|)$).
///
/// $$
/// d(x, y) = \sum_{i=1}^n (x_i - y_i)^2
/// $$
///
/// Since $d : B \rightarrow I$ and $min(A) < min(B) < max(B) < max(A)$, we just
/// need to prove the range of $d$ does not exceed the minimum or maximum of its
/// codomain.
pub fn delta(a: &[u8], b: &[u8]) -> i32 {
    a.iter()
        .zip(b.iter())
        .map(|(&a_c, &b_c)| (a_c as i32 - b_c as i32).pow(2))
        .sum()
}

#[cfg(test)]
pub fn setup_tinycross() -> Result<assert_fs::TempDir, assert_fs::fixture::FixtureError> {
    use std::path::Path;

    use assert_fs::{
        fixture::{FileWriteFile, PathChild},
        TempDir,
    };

    let temp = TempDir::new()?;
    let input_file = temp.child("tinycross.png");
    input_file.write_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/tinycross.png"))?;
    Ok(temp)
}

#[cfg(test)]
mod tests {
    use crate::utils::delta;

    /// This covers the most common use case
    #[test]
    fn test_delta_overflow_3() {
        assert!(delta(&[u8::MAX; 3], &[0; 3]) < i32::MAX)
    }

    #[test]
    fn test_delta_overflow_max() {
        const LEN: usize = 33025;
        assert!(delta(&[u8::MAX; LEN], &[0; LEN]) < i32::MAX)
    }

    #[test]
    #[should_panic]
    fn test_delta_overflow_over_max() {
        const LEN: usize = 33026;
        assert!(delta(&[u8::MAX; LEN], &[0; LEN]) > i32::MAX)
    }
}
