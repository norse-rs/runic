#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeffs: Vec<f32>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f32>) -> Self {
        Polynomial { coeffs }
    }

    pub fn eval(&self, t: f32) -> f32 {
        let mut tmp = 1.0;
        let mut result = 0.0;

        for coeff in &self.coeffs {
            result += coeff * tmp;
            tmp *= t;
        }

        result
    }

    /// Indefinite integral excluding the constant
    pub fn integral(&self) -> Self {
        let mut coeffs = vec![0.0];
        for (i, c) in self.coeffs.iter().enumerate() {
            coeffs.push(*c / (i + 1) as f32);
        }
        Polynomial::new(coeffs)
    }

    /// Parametric line
    pub fn line(p0: f32, p1: f32) -> Self {
        Polynomial::new(vec![p1 - p0, p0])
    }

    pub fn mul(&self, rhs: &Polynomial) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len() + rhs.coeffs.len() - 1];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                let order = i + j;
                coeffs[order] += a * b;
            }
        }

        Polynomial::new(coeffs)
    }

    pub fn mul_scalar(&self, scalar: f32) -> Self {
        let mut coeffs = self.coeffs.clone();
        for c in &mut coeffs {
            *c *= scalar;
        }
        Polynomial::new(coeffs)
    }

    pub fn compose(&self, g: &Polynomial) -> Self {
        let mut tmp = Polynomial::new(vec![1.0]);
        let mut result = Polynomial::new(vec![0.0]);

        for c in &self.coeffs {
            let term = tmp.mul_scalar(*c);
            result = result.add(&term);
            tmp = tmp.mul(g);
        }

        result
    }

    pub fn add(&self, rhs: &Polynomial) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len().max(rhs.coeffs.len())];
        for (i, c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        Polynomial::new(coeffs)
    }

    pub fn sub(&self, rhs: &Polynomial) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len().max(rhs.coeffs.len())];
        for (i, c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, c) in rhs.coeffs.iter().enumerate() {
            coeffs[i] -= c;
        }
        Polynomial::new(coeffs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integral() {
        // 1 /-> x
        assert_eq!(
            Polynomial::new(vec![1.0]).integral(),
            Polynomial::new(vec![0.0, 1.0])
        );
        // 1 + x /-> x + 0.5 * x^2
        assert_eq!(
            Polynomial::new(vec![1.0, 1.0]).integral(),
            Polynomial::new(vec![0.0, 1.0, 0.5])
        );
    }

    #[test]
    fn compose() {
        // x(x) /-> x
        assert_eq!(
            Polynomial::new(vec![0.0, 1.0]).compose(&Polynomial::new(vec![0.0, 1.0])),
            Polynomial::new(vec![0.0, 1.0])
        );
        // x(x + 1) /-> x + 1
        assert_eq!(
            Polynomial::new(vec![0.0, 1.0]).compose(&Polynomial::new(vec![1.0, 1.0])),
            Polynomial::new(vec![1.0, 1.0])
        );
    }
}
