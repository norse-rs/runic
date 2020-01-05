#[derive(Debug)]
pub struct Polynomial {
    coeffs: Vec<f32>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f32>) -> Self {
        Polynomial {
            coeffs,
        }
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
}
