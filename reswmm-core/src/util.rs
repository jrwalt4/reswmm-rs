//! internal utilities

mod root {

    use num_traits::{Float, Signed};

    use std::ops::{Add, ControlFlow, Div, Sub};

    #[derive(Debug)]
    pub enum Reason {
        Ok,
        MaxIterations,
        InvalidBounds,
        Warning(String),
    }

    pub struct Iteration<T, V> {
        trial: (T, T),
        value: V,
        count: usize,
    }

    pub trait Tolerance<T, V> {
        /// The Reason for breaking
        type Reason;

        fn check_domain(&self, domain: (T, T)) -> ControlFlow<Result<(), Self::Reason>>;

        fn check_range(&self, range: V) -> ControlFlow<Result<(), Self::Reason>>;

        fn check_iteration(&self, count: usize) -> ControlFlow<Result<(), Self::Reason>>;
    }

    pub struct BoundedTolerance<T, V> {
        convergence: Option<T>,
        tolerance: Option<V>,
        max_iterations: usize,
    }

    impl<T, V> BoundedTolerance<T, V> {
        pub fn new() -> Self {
            Self {
                convergence: None,
                tolerance: None,
                max_iterations: usize::MAX / 2,
            }
        }

        pub fn with_values(convergence: T, tolerance: V, max_iterations: usize) -> Self {
            Self {
                convergence: Some(convergence),
                tolerance: Some(tolerance),
                max_iterations,
            }
        }

        pub fn with_convergence(mut self, convergence: T) -> Self {
            self.convergence.replace(convergence);
            self
        }

        pub fn with_tolerance(mut self, tolerance: V) -> Self {
            self.tolerance.replace(tolerance);
            self
        }

        pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
            self.max_iterations = max_iterations;
            self
        }
    }

    impl<T: Float, V: Float> Default for BoundedTolerance<T, V> {
        fn default() -> Self {
            Self {
                convergence: Some(T::epsilon()),
                tolerance: Some(V::epsilon()),
                max_iterations: usize::MAX / 2,
            }
        }
    }

    impl<T, V> Tolerance<T, V> for BoundedTolerance<T, V>
    where
        T: Sub<Output = T> + Signed + PartialOrd,
        V: Signed + PartialOrd,
    {
        type Error = Error<T>;
        fn check(&self, it: Iteration<T, V>) -> ControlFlow<Result<(), Self::Error>> {
            let Iteration {
                trial,
                value,
                count,
            } = it;

            if count > self.max_iterations {
                return ControlFlow::Break(Err(Error::MaxIterations));
            }

            if let Some(c) = &self.convergence && (trial.1 - trial.0).abs() < *c {
                return ControlFlow::Break(Ok(()));
            }

            if let Some(tol) = &self.tolerance && value.abs() < *tol {
                return ControlFlow::Break(Ok(()))
            }
            ControlFlow::Continue(())
        }
    }

    pub fn bisect<T, V, F: Fn(T) -> V, Tol: Tolerance<T, V>>(
        bounds: (T, T),
        f: F,
        tol: Tol,
    ) -> Result<T, Tol::Error>
    where
        T: Copy + Add<Output = T> + Div<f64, Output = T>,
        V: Copy + Signed,
    {
        let (mut low, mut high) = bounds;
        let mut f_low = f(low);
        let mut n = 0;
        loop {
            use ControlFlow::*;
            let mid = (low + high) / 2.0;
            let f_mid = f(mid);
            match tol.check(Iteration {
                trial: (low, high),
                value: f_mid,
                count: n,
            }) {
                Break(Ok(_)) => return Ok(mid),
                Break(Err(e)) => return Err(e),
                Continue(()) => {
                    n += 1;
                    if f_mid.signum() == f_low.signum() {
                        low = mid;
                        f_low = f_mid;
                    } else {
                        high = mid;
                    }
                }
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_root() {
        let f = |x: f64| x.powi(2) - 2.0;
        let sqrt_2 = bisect(
            (-1., 10.0),
            f,
            BoundedTolerance::with_values(0.01, 0.001, 5),
        )
        .unwrap();
        assert!((sqrt_2 - f64::sqrt(2.0)).abs() < 0.01);
    }

    pub fn newton<T, V, F, Tol>(guess: T, f: F, tol: Tol) -> Result<T, Tol::Error>
    where
        Tol: Tolerance<T, V>
    {
        Ok(guess)
    }
}
