//use rand::Rng;
use rand::distributions::{Normal, Distribution};

pub trait Process {
    fn devide_width(&self) -> f64;
    fn generate_process<F>(&self, data: Vec<(f64, f64)>, f: F) -> Vec<(f64, f64)> where F: Fn(f64, Vec<(f64, f64)>) -> Vec<(f64, f64)>;
} 

pub struct StochasticProcess {
    pub term: f64,
    pub devide_number: i32
}

impl Process for StochasticProcess {
    fn devide_width(&self) -> f64 {
        self.term / (self.devide_number as f64)
    }
    fn generate_process<F>(&self, data: Vec<(f64, f64)>, f: F) -> Vec<(f64, f64)> where F: Fn(f64, Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        f(self.devide_width(), data)
    }
}

impl StochasticProcess {
    pub fn brownian_motion(&self) -> Vec<(f64, f64)> {
        let norm_rand = norm_dist_vec(self.devide_number);
        self.generate_process(norm_rand, |devide_width, norm_rand_vec| {
            norm_rand_vec.into_iter().scan(0.0, |acc, x| {
                let bm = if x.0 == 0.0 { 0.0 } else { *acc + x.1 * devide_width.sqrt()};
                *acc = bm;
                Some((x.0 / (self.devide_number as f64), bm))
            }).collect()
        })
    }

    pub fn geometric_brownian_motion(&self, mu: f64, vola: f64, init: f64) -> Vec<(f64, f64)> {
        let norm_rand = norm_dist_vec(self.devide_number);
        self.generate_process(norm_rand, |devide_width, norm_rand_vec| {
            norm_rand_vec.into_iter().scan((0.0, init), |acc, x| {
                let t = x.0 / (self.devide_number as f64);
                let bm = if x.0 == 0.0 { 0.0 } else { acc.0 + x.1 * devide_width.sqrt()};
                let gbm = if x.0 == 0.0 { init } else { init * ((mu - 0.5 * vola * vola) * &t + vola * bm).exp()};
                *acc = (bm, gbm);
                Some((t, gbm))
            }).collect()
        })
    }
}

pub fn norm_dist_vec(size: i32) -> Vec<(f64, f64)> {
    let normal = Normal::new(0.0,1.0);
    let range = (0..(size - 1)).collect::<Vec<i32>>();
    range.into_iter().map(|i| (i as f64, normal.sample(&mut rand::thread_rng()))).collect::<Vec<(f64, f64)>>()
}
