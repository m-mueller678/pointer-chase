use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::ptr;

pub struct PointerCycle(Vec<*const u8>);

impl PointerCycle {
    pub fn build(size: usize) -> Self {
        let mut vec = vec![ptr::null(); size];
        for x in &mut vec {
            *x = x as *mut *const u8 as *const u8;
        }
        // Sattolo's algorithm
        let mut rng = SmallRng::from_entropy();
        for i in (1..size).rev() {
            let j = rng.gen_range(0..i);
            vec.swap(i, j);
        }
        PointerCycle(vec)
    }

    pub fn walk_loop(&self, mut f: impl FnMut()) -> ! {
        let start = self.0[0];
        let len = self.0.len();
        loop {
            let mut p = start;
            for _ in (0..len - 1).rev() {
                p = unsafe { *(p as *const *const u8) };
                debug_assert!(p != start);
                f();
            }
            p = unsafe { *(p as *const *const u8) };
            assert_eq!(p, start);
            f();
        }
    }
}
