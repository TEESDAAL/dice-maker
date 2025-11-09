use std::ops::{Index, Add, Sub, Div, IndexMut, Mul, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const T: usize>(pub [f64; T]);

impl Vector<3> {
    pub fn cross_product(&self, other: &Self) -> Self {
        let (Vector([x1, y1, z1]), Vector([x2, y2, z2])) = (self, other);
        Vector([
            y1*z2 - y2*z1,
            -(x1*z2 - x2*z1),
            x1*y2 - x2*y1
        ])

    }
}

impl <const T: usize> IntoIterator for Vector<T> {
    type Item = f64;

    type IntoIter = std::array::IntoIter<Self::Item, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl <const T: usize> Vector<T> {
    pub const fn zero() -> Self {
        Vector([0.0; T])
    }

    pub fn dot(&self, other: &Self) -> f64 {
        let mut total = 0.0;
        for i in 0..T {
            total += self[i]*other[i];
        }

        total
    }
}

impl <const T: usize> Index<usize> for Vector<T> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}


impl <const T: usize> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <const T: usize> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; T];
        for i in 0..T {
            res[i] = self[i] + rhs[i];
        }
        Vector(res)
    }
}

impl <const T: usize> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; T];
        for i in 0..T {
            res[i] = self[i] - rhs[i];
        }
        Vector(res)
    }
}

impl <const T: usize> Div for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; T];
        for i in 0..T {
            res[i] = self[i] / rhs[i];
        }
        Vector(res)
    }
}


impl <const T: usize> Mul<Vector<T>> for f64 {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        let mut res = [0.0; T];

        for i in 0..T {
            res[i] = self * rhs[i]
        }

        Vector(res)
    }
}

impl <const T: usize> Neg for Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl <const T: usize> Mul<f64> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut res = [0.0; T];

        for i in 0..T {
            res[i] = rhs * self[i]
        }

        Vector(res)
    }
}


#[cfg(test)]
mod tests {
    use std::f64;

    use super::*;
    use rand::Rng;

    const N: usize = 100_000;

    #[test]
    fn cross_with_self_is_zero() {
        let mut rng = rand::rng();
        for _ in 0..N {
            let v = Vector(rng.random::<[f64; 3]>());
            assert_eq!(v.cross_product(&v), Vector::zero());
        }
    }

    #[test]
    fn orthogal_test() {
        let mut rng = rand::rng();
        for _ in 0..N {
            let v1 = Vector(rng.random::<[f64; 3]>());
            let v2 = Vector(rng.random::<[f64; 3]>());

            assert!(v1.cross_product(&v2).dot(&v1).abs() <= 1e-8, "{} != 0", v1.cross_product(&v2).dot(&v1).abs());
            assert!(v1.cross_product(&v2).dot(&v2).abs() <= 1e-8, "{} != 0", v1.cross_product(&v2).dot(&v1).abs());
        }
    }

    #[test]
    fn test_swapping_negates() {
        let mut rng = rand::rng();
        for _ in 0..N {
            let v1 = Vector(rng.random::<[f64; 3]>());
            let v2 = Vector(rng.random::<[f64; 3]>());

            let cross = v1.cross_product(&v2) + v2.cross_product(&v1);
            for x in cross {
                assert!(x.abs() < f64::EPSILON, "{:?} cross {:?} \n  = {:?} \n != -{:?}", v1, v2, v1.cross_product(&v2), -v2.cross_product(&v1));
            }
        }
    }
}
