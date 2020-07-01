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

    pub fn prepare(&mut self, coefs1: &[f64], coefs2: &[f64]) {
        let size = max!(coefs1.len() + 2, coefs2.len() + 2);
        if size > self.size {
            let size = size + size / 2;
            self.raw.resize(size * size, 0.0);
            self.size = size;
            self.init();
        }
        unsafe {
            for (i1, coef) in coefs1.iter().enumerate() {
                let prev = self.get_unchecked(i1 + 1, 1);
                self.set_unchecked(i1 + 2, 1, prev + coef);
            }
            for (i2, coef) in coefs2.iter().enumerate() {
                let prev = self.get_unchecked(1, i2 + 1);
                self.set_unchecked(1, i2 + 2, prev + coef);
            }
        }
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
        for i in 0 .. imax {
            for j in 0 .. jmax {
                write!(f, "{:4}  ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}