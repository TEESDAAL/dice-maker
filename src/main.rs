#![feature(generic_const_exprs)]
#![warn(
     clippy::pedantic,
 )]
use dice_maker::{
    gradient_descent::GradientDescent,
    planes::Plane,
    point_placer::{PointPlacer, vec_to_points},
};

const STEPS: usize = 100_000;
const NUM_FACES: usize = 100;


fn main() {
    let mode: fn(usize) -> f64 = |_| 0.0001;

    let dice: PointPlacer<NUM_FACES> = PointPlacer::fib_initialisation();

    let params = dice.gradient_descent(STEPS, mode);

    let final_points = vec_to_points(params);
    let mut planes = Vec::with_capacity(NUM_FACES);
    for point in final_points {
        let [x, y, z] = point.cartiesian_coords();
        planes.push(Plane::from(x, y, z, 1.0));
    }

    while !any_intersect_in_sphere(&planes) {
        for plane in &mut planes {
            plane.bias -= 1e-8;
        }
    }

    let mut dice = String::from("");
    for plane in planes {
        let Plane { x, y, z, bias } = plane;
        println!("(x, y, z) \\cdot ({}, {}, {}) = {} ", x, y, z, bias);
        let Plane { x, y, z, bias } = plane;
        dice += format!("(x, y, z) \\cdot ({}, {}, {}) - {} - b, ", x, y, z, bias).as_str();
    }

    println!("max({}x^2+y^2+z^2-1) \\leq 0", dice);
}

fn any_intersect_in_sphere(planes: &[Plane]) -> bool {
    for i in 0..planes.len() {
        let p1 = &planes[i];
        for p2 in planes.iter().skip(i + 1) {
            let should_return =
                matches!(p1.intersection(p2), Some(line) if line.intersects_circle());
            if should_return {
                return true;
            }
        }
    }
    false
}

fn _linearly_interpolate(start: f64, end: f64, steps: usize) -> impl Fn(usize) -> f64 {
    move |step| start + (end - start) * (step as f64) / steps as f64
}


