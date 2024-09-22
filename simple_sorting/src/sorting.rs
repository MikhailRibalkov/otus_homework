pub use crate::sorting;
use rand::Rng;
use std::{
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

    fn binary_search(&mut self, key: i32, min: i64, max: i64) -> usize {
        if max <= min {
            if key >= self.arr[min as usize] {
                return (min + 1) as usize;
            } else {
                return min as usize;
            }
        }

        let mid = (min + max) / 2;
        if key > self.arr[mid as usize] {
            self.binary_search(key, mid + 1, max)
        } else {
            self.binary_search(key, min, mid - 1)
        }
    }

    pub fn insert_binary_sort(&mut self) {
        let start = Instant::now();

        let arr_len = self.arr.len();

        for j in 1..arr_len {
            let t = self.arr[j];
            let p = self.binary_search(t, 0, j as i64 - 1);
            for index in (p..j).rev() {
                self.arr[index + 1] = self.arr[index];
            }
            self.arr[p] = t;
        }
        self.duration = start.elapsed();
        println!("N: {arr_len}, duration: {:?}", self.duration);
    }

    pub fn insert_shift_sort(&mut self) {
        let n: usize = self.arr.len();

        let start = Instant::now();
        for index in 1..n {
            let mut j: usize = index - 1;
            let t = self.arr[index];
            while self.arr[j] > t {
                self.arr[j + 1] = self.arr[j];
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            self.arr[j + 1] = t;
        }
        self.duration = start.elapsed();
        println!("N: {n}, duration: {:?}", self.duration);
    }

    pub fn insert_sort(&mut self) {
        let n: usize = self.arr.len();

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
        println!("N: {n}, duration: {:?}", self.duration);
    }

    pub fn bubble_sort(&mut self) {
        let n = self.arr.len();
        let start = Instant::now();
        for index in (0..n).rev() {
            for j in 0..index {
                if self.arr[index] < self.arr[j] {
                    self.arr.swap(index, j);
                }
            }
        }
        self.duration = start.elapsed();
        println!("N: {n}, duration: {:?}", self.duration);
    }

    pub fn get_array(&mut self, n: usize) {
        self.arr.resize(n, 0);
        let mut rng = rand::thread_rng();
        for v in 0..n {
            self.arr[v] = rng.gen_range(1..1000);
        }
    }
}
