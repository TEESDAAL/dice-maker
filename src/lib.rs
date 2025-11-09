#![feature(generic_const_exprs)]
use rand::Rng;
pub mod gradient_descent;
pub mod planes;
pub mod point_placer;
pub mod vector;

/// theta in [0, pi]
#[derive(Debug, Clone, Copy, PartialEq)]
struct PolarAngle(f64);

impl PolarAngle {
    pub fn new(angle: f64) -> Self {
        PolarAngle(angle.rem_euclid(std::f64::consts::PI))
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let random_number: f64 = rng.random::<f64>() * std::f64::consts::PI;
        PolarAngle::new(random_number)
    }
}

/// theta in [0, 2pi]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Azimuth(f64);

impl Azimuth {
    pub fn new(angle: f64) -> Self {
        Azimuth(angle.rem_euclid(std::f64::consts::TAU))
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let random_number: f64 = rng.random::<f64>() * std::f64::consts::TAU;
        Azimuth::new(random_number)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(PolarAngle, Azimuth);

pub fn hav(x: f64) -> f64 {
    (1.0 - f64::cos(x)) / 2.0
}

pub fn ahav(x: f64) -> f64 {
    2.0 * f64::asin(f64::sqrt(x))
}

impl Point {
    pub fn coords(&self) -> (f64, f64) {
        let Point(PolarAngle(x), Azimuth(y)) = *self;
        (x, y)
    }

    pub fn cartiesian_coords(&self) -> [f64; 3] {
        [
            f64::sin(self.0.0) * f64::cos(self.1.0),
            f64::sin(self.0.0) * f64::sin(self.1.0),
            f64::cos(self.0.0),
        ]
    }

    pub fn new(polar_angle: f64, azimuth: f64) -> Self {
        Point(PolarAngle::new(polar_angle), Azimuth::new(azimuth))
    }

    pub fn random() -> Self {
        Point(PolarAngle::random(), Azimuth::random())
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.euclidian_distance(other)
    }

    pub fn euclidian_distance(&self, other: &Point) -> f64 {
        self.cartiesian_coords()
            .iter()
            .zip(other.cartiesian_coords().iter())
            .map(|(c1, c2)| (c1 - c2).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 100_000;
    #[test]
    fn distance_to_self_is_zero() {
        let p = Point::new(1.0, 1.0);
        assert_eq!(
            p.distance(&p),
            0.0,
            "The distance between {:?} and itself is not 0",
            p
        );

        for _ in 0..N {
            let p = Point::random();
            assert_eq!(
                p.distance(&p),
                0.0,
                "The distance between {:?} and itself is not 0",
                p
            );
        }
    }

    #[test]
    fn distance_to_other_not_zero() {
        for _ in 0..N {
            let (p1, p2) = (Point::random(), Point::random());
            assert_ne!(
                p1.distance(&p2),
                0.0,
                "The distance between {:?} and {:?} is 0",
                p1,
                p2
            );
        }
    }

    #[test]
    fn distance_symmetric() {
        for _ in 0..N {
            let (p1, p2) = (Point::random(), Point::random());
            assert_eq!(p1.distance(&p2), p2.distance(&p1));
        }
    }

    #[test]
    fn triangle() {
        for _ in 0..N {
            let (p1, p2, p3) = (Point::random(), Point::random(), Point::random());
            let long_distance = p1.distance(&p2) + p2.distance(&p3);
            let short_distance = p1.distance(&p3);

            assert!(short_distance <= long_distance);
        }
    }
}
