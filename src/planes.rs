use crate::vector::Vector;

#[derive(Debug)]
pub struct Plane {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub bias: f64,
}

impl Plane {
    pub const fn from(x: f64, y: f64, z: f64, bias: f64) -> Self {
        Self { x, y, z, bias }
    }
    pub fn intersection(&self, other: &Plane) -> Option<Line> {
        let norm = Vector::cross_product(&self.norm(), &other.norm());
        if norm == Vector::zero() {
            return None;
        }
        // let z = 0, assumes that the intersection will pass through the z-axis...
        let [a1, b1, _, d1] = self.unpack();

        let [a2, b2, _, d2] = other.unpack();
        let x = (d2 * b1 - b2 * d1) / (a2 * b1 - a1 * b2);
        let y = (d1 - a1 * x) / b1;
        let z = 0.0;
        Some(Line {
            direction: norm,
            position: Vector([x, y, z]),
        })
    }

    fn norm(&self) -> Vector<3> {
        Vector([self.x, self.y, self.z])
    }

    fn unpack(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.bias]
    }
}

#[derive(Debug)]
pub struct Line {
    direction: Vector<3>,
    position: Vector<3>,
}

impl Line {
    // See if this line intersects with the unit circle x^2 + y^2 + z^2 = 1
    pub fn intersects_circle(&self) -> bool {
        self.determiant() >= 0.0
    }

    fn determiant(&self) -> f64 {
        let Vector([nx, ny, nz]) = self.direction;
        let Vector([px, py, pz]) = self.position;
        let a = nx.powi(2) + ny.powi(2) + nz.powi(2);
        let b = 2.0 * (nx * px + ny * py + nz * pz);
        let c = px.powi(2) + py.powi(2) + py.powi(2) - 1.0;

        b.powi(2) - 4.0 * a * c
    }

    pub fn new(direction: Vector<3>, position: Vector<3>) -> Self {
        Line {
            direction,
            position,
        }
    }

    pub fn format(&self) -> String {
        format!("t{:?} + {:?}", self.direction.0, self.position.0)
            .replace("[", "(")
            .replace("]", ")")
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    const N: usize = 100_000;

    #[test]
    fn test_intersects_circle() {
        for _ in 0..N {
            let mut rng = rand::rng();
            let (t, p) = rng.random::<(f64, f64)>();
            let r = rng.random_range(0..=1000) as f64 / 1000.0;
            let line = Line::new(
                Vector(rng.random()),
                Vector([r * t.sin() * p.cos(), r * t.sin() * p.sin(), r * t.cos()]),
            );

            assert!(
                line.intersects_circle(),
                "{} should intersect with the unit circle. det = {}",
                line.format(),
                line.determiant()
            )
        }
    }
}
