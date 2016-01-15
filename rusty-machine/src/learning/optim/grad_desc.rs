//! Gradient Descent
//!
//! Implementation of gradient descent algorithm. Module contains
//! the struct GradientDesc which is instantiated within models
//! implementing the Optimizable trait.
//!
//! Currently standard batch gradient descent is the only implemented
//! optimization algorithm but there is flexibility to introduce new
//! algorithms and git them into the same scheme easily.

use learning::optim::{Optimizable, OptimAlgorithm};
use linalg::vector::Vector;
use linalg::matrix::Matrix;

/// Batch Gradient Descent algorithm
pub struct GradientDesc {
    /// The step-size for the gradient descent steps.
    pub alpha: f64,
    /// The number of iterations to run.
    pub iters: usize,
}

impl Default for GradientDesc {
    /// Constructs a gradient descent algorithm
    /// with default settings.
    ///
    /// Uses 10000 iterations and step size of 0.3.
    fn default() -> GradientDesc {
        GradientDesc {
            alpha: 0.3,
            iters: 100,
        }
    }
}

impl GradientDesc {
    /// Construct a gradient descent algorithm.
    ///
    /// Requires the step size and iteration count
    /// to be specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::optim::grad_desc::GradientDesc;
    ///
    /// let gd = GradientDesc::new(0.3, 10000);
    /// ```
    pub fn new(alpha: f64, iters: usize) -> GradientDesc {
        GradientDesc {
            alpha: alpha,
            iters: iters,
        }
    }
}

impl<M: Optimizable> OptimAlgorithm<M> for GradientDesc {
    fn optimize(&self, model: &M, start: &[f64], data: &M::Data, outputs: &M::Target) -> Vec<f64> {

        let mut optimizing_val = Vector::new(start.to_vec());

        for _ in 0..self.iters {
            optimizing_val = &optimizing_val -
                             Vector::new(model.compute_grad(&optimizing_val.data()[..],
                                                            data,
                                                            outputs)
                                              .1) * self.alpha;
        }
        optimizing_val.into_vec()
    }
}

/// Stochastic Gradient Descent algorithm.
///
/// Uses basic momentum to control the learning rate.
pub struct StochasticGD {
    /// Controls the momentum of the descent
    pub alpha: f64,
    /// The square root of the raw learning rate.
    pub mu: f64,
    /// The number of passes through the data.
    pub iters: usize,
}

impl Default for StochasticGD {
    /// Constructs a stochastic gradient descent algorithm
    /// with default settings.
    ///
    /// Uses 5 iterations, momentum of 0.1 and rate of 0.3.
    fn default() -> StochasticGD {
        StochasticGD {
            alpha: 0.1,
            mu: 0.1,
            iters: 20,
        }
    }
}

impl StochasticGD {
    /// Construct a stochastic gradient descent algorithm.
    ///
    /// Requires the learning rate, momentum rate and iteration count
    /// to be specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::optim::grad_desc::StochasticGD;
    ///
    /// let sgd = StochasticGD::new(0.1, 0.3, 5);
    /// ```
    pub fn new(alpha: f64, mu: f64, iters: usize) -> StochasticGD {
        StochasticGD {
            alpha: alpha,
            mu: mu,
            iters: iters,
        }
    }
}

impl<M: Optimizable<Data = Matrix<f64>, Target = Matrix<f64>>> OptimAlgorithm<M> for StochasticGD {
    fn optimize(&self, model: &M, start: &[f64], data: &M::Data, outputs: &M::Target) -> Vec<f64> {

        let (_, vec_data) = model.compute_grad(start,
                                               &data.select_rows(&[0]),
                                               &outputs.select_rows(&[0]));
        let grad = Vector::new(vec_data);
        let mut delta_w = grad * self.alpha;
        let mut optimizing_val = Vector::new(start.to_vec()) - &delta_w * self.mu;

        for _ in 0..self.iters {
            for i in 1..data.rows() {
                let (_, vec_data) = model.compute_grad(&optimizing_val.data()[..],
                                                       &data.select_rows(&[i]),
                                                       &outputs.select_rows(&[i]));

                delta_w = Vector::new(vec_data) * self.mu + &delta_w * self.alpha;
                optimizing_val = &optimizing_val - &delta_w * self.mu;
            }
        }
        optimizing_val.into_vec()
    }
}
