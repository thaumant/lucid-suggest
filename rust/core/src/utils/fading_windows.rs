use std::cmp::min;


#[derive(Debug)]
pub struct FadingWindows<'a, T: 'a> {
    v: &'a [T],
    size: usize,
}

impl<'a, T: 'a> FadingWindows<'a, T> {
    pub fn new(v: &'a [T], size: usize) -> Self {
        if size == 0 && v.len() > 0 {
            panic!("Can't have zero window size with nonempty slice");
        }
        Self { v, size }
    }
}

impl<'a, T> Iterator for FadingWindows<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.len() == 0 {
            None
        } else {
            let window = Some(&self.v[.. min(self.size, self.v.len())]);
            self.v = &self.v[1..];
            window
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.v.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for FadingWindows<'a, T> { }


#[cfg(test)]
mod tests {
    use super::FadingWindows;

    #[test]
    fn fading_windows_empty() {
        let input: [usize; 0] = [];
        for size in 0 .. 4 {
            let output = FadingWindows::new(&input, size).collect::<Vec<_>>();
            let expect: Vec<&[usize]> = Vec::new();
            assert_eq!(output, expect);
        }
    }


    #[test]
    fn fading_windows_singleton() {
        let input: [usize; 1] = [10];
        for size in 1 .. 4 {
            let output = FadingWindows::new(&input, size).collect::<Vec<_>>();
            let expect: Vec<&[usize]> = vec![&[10]];
            assert_eq!(output, expect);
        }
    }


    #[test]
    fn fading_windows_len_gt_size() {
        let size = 3;
        let input: [usize; 5] = [10, 20, 30, 40, 50];
        let expect: [&[usize]; 5] = [&[10, 20, 30], &[20, 30, 40], &[30, 40, 50], &[40, 50], &[50]];
        let output = FadingWindows::new(&input, size).collect::<Vec<_>>();
        assert_eq!(output, expect);
    }


    #[test]
    fn fading_windows_size_gt_len() {
        let size = 5;
        let input: [usize; 3] = [10, 20, 30];
        let expect: [&[usize]; 3] = [&[10, 20, 30], &[20, 30], &[30]];
        let output = FadingWindows::new(&input, size).collect::<Vec<_>>();
        assert_eq!(output, expect);
    }


    #[test]
    #[should_panic]
    fn fading_windows_zero_size() {
        let size = 0;
        let input: [usize; 3] = [10, 20, 30];
        FadingWindows::new(&input, size);
    }
}
