use crate::vector::Vector;

pub trait GradientDescent<const T: usize> {
    fn parameters(&self) -> Vector<T>;
    fn loss(&self, parameters: Vector<T>) -> f64;
    fn gradient(&self, parameters: Vector<T>) -> Vector<T> {
        self.numeric_gradient(parameters)
    }

    fn gradient_descent(&self, steps: usize, learning_rate: impl Fn(usize) -> f64) -> Vector<T> {
        let mut parameters = self.parameters();

        for step in 0..steps {
            parameters = parameters - learning_rate(step) * self.gradient(parameters)
        }

        parameters
    }

    fn numeric_gradient(&self, x: Vector<T>) -> Vector<T> {
        let mut result = [0.0; T];
        let base_loss = self.loss(x);
        let e = f64::sqrt(f64::EPSILON);

        for i in 0..T {
            let mut h = Vector([0.0; T]);
            let h_value = x[i] * e;

            h[i] = h_value;
            result[i] = (self.loss(x + h) - base_loss) / h_value;
        }

        Vector(result)
    }
}
