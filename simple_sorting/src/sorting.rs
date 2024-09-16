pub use crate::sorting;
use rand::Rng;
use std::{
    isize,
    time::{Duration, Instant},
    usize,
};

#[derive(Clone)]
pub struct SortObject {
    pub arr: Vec<i32>,
    pub duration: Duration,
}

impl Default for SortObject {
    fn default() -> Self {
        Self::new()
    }
}

impl SortObject {
    pub fn new() -> SortObject {
        SortObject {
            arr: vec![],
            duration: Duration::new(0, 0),
        }
    }

    pub fn insert_sort(&mut self) {
        let n: usize = self.arr.len();
        println!("{:?}", self.arr);

        let start = Instant::now();
        for index in 1..n {
            let mut j: usize = index - 1;
            while self.arr[j] > self.arr[j + 1] {
                self.arr.swap(j, j + 1);
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }
        self.duration = start.elapsed();
        println!("{:?}", self.arr);
        println!("duration: {:?}", self.duration);
    }

    pub fn bubble_sort(&mut self) {
        let n = self.arr.len();
        println!("{:?}", self.arr);
        let start = Instant::now();
        for index in (0..n).rev() {
            for j in 0..index {
                if self.arr[index] < self.arr[j] {
                    self.arr.swap(index, j);
                }
            }
        }
        self.duration = start.elapsed();
        println!("{:?}", self.arr);
        println!("duration: {:?}", self.duration);
    }

    pub fn get_array(&mut self, n: usize) {
        self.arr = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            self.arr.push(rng.gen_range(1..10));
        }
    }
}
