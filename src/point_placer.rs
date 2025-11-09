use crate::{Point, gradient_descent::GradientDescent, vector::Vector};


#[derive(Debug)]
pub struct DiceProblem<const T: usize> {
    points: [Point; T]
}

impl <const T: usize> DiceProblem<T> {
    /// Generate T random points
    pub fn new() -> Self {
        Self {
            points: (0..T).map(|_| Point::random())
                .collect::<Vec<_>>().try_into().unwrap()
        }
    }
}

impl<const T: usize> Default for DiceProblem<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn vec_to_points<const T: usize>(vector: Vector<{2*T}>) -> [Point; T] {
    let mut points = Vec::with_capacity(2*T);
    for i in (0..(2*T)).step_by(2) {
        let (x, y) = (vector[i], vector[i+1]);
        points.push(Point::new(x, y));
    }

    points.try_into().unwrap()

}

impl <const T: usize> GradientDescent<{2*T}> for DiceProblem<T> {
    fn parameters(&self) -> Vector<{2*T}> {
        let mut params = [0.0; _];
        for i in 0..T {
            let (x, y) = self.points[i].coords();
            params[2*i] = x;
            params[2*i+1] = y;
        }
        Vector(params)
    }

    fn loss(&self, parameters: Vector<{2*T}>) -> f64 {
        let mut total_distance = 0.0;
        let points = vec_to_points(parameters);

        for i in 0..T {
            for j in (i+1)..T {
                total_distance += points[i].distance(&points[j])
            }
        }
        -total_distance
    }
}

