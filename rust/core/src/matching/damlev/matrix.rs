use std::fmt;


pub struct DistMatrix {
    size: usize,
    raw: Vec<f64>,
}


impl DistMatrix {
    pub fn new(size: usize) -> Self {
        let raw = vec![0.0; size * size];
        let mut matrix = Self { size, raw };
        matrix.init();
        matrix
    }

    pub fn grow(&mut self, size: usize) {
        if size <= self.size {
            return;
        }
        let size = size + size / 2;
        self.raw.resize(size * size, 0.0);
        self.size = size;
        self.init();
    }

    pub fn init(&mut self) {
        if self.size == 0 { return; }
        unsafe {
            for i in 0..self.size {
                self.set_unchecked(i, 0, self.size as f64);
                self.set_unchecked(0, i, self.size as f64);
            }
            for i in 1..self.size {
                self.set_unchecked(i, 1, (i - 1) as f64);
                self.set_unchecked(1, i, (i - 1) as f64);
            }
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> f64 {
        *self.raw.get_unchecked(i * self.size + j)
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, i: usize, j: usize, val: f64) {
        *self.raw.get_unchecked_mut(i * self.size + j) = val;
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.raw[i * self.size + j]
    }
}


impl fmt::Debug for DistMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut imax = self.size;
        let mut jmax = self.size;
        while imax > 0 && self.get(imax - 1, 2) <= std::f64::EPSILON { imax -= 1; }
        while jmax > 0 && self.get(2, jmax - 1) <= std::f64::EPSILON { jmax -= 1; }

        write!(f, "DistMatrix:\n")?;
        for i in 2 .. imax {
            for j in 2 .. jmax {
                write!(f, "{:2} ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}