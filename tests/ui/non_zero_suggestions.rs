#![warn(clippy::non_zero_suggestions)]

use std::num::{NonZeroI16, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

fn main() {
    // Positive test cases (lint should trigger)

    // U32 -> U64
    let x: u64 = 100;
    let y = NonZeroU32::new(10).unwrap();
    let r1 = x / u64::from(y.get());
    let r2 = x % u64::from(y.get());

    // U16 -> U32
    let a: u32 = 50;
    let b = NonZeroU16::new(5).unwrap();
    let r3 = a / u32::from(b.get());

    // I8 -> I16
    let c: i16 = 25;
    let d = NonZeroI8::new(3).unwrap();
    let r4 = i16::from(d.get());

    // Different operations
    let m: u64 = 400;
    let n = NonZeroU32::new(20).unwrap();
    let r5 = m / u64::from(n.get());

    // Edge cases

    // Using the max value of a type
    let max_u32 = NonZeroU32::new(u32::MAX).unwrap();
    let r6 = u64::from(max_u32.get());

    // Chained method calls
    let _ = u64::from(NonZeroU32::new(10).unwrap().get());

    // Negative test cases (lint should not trigger)

    // Same size types
    let e: u32 = 200;
    let f = NonZeroU32::new(20).unwrap();
    let r10 = e / f.get();

    // Smaller to larger, but not NonZero
    let g: u64 = 1000;
    let h: u32 = 50;
    let r11 = g / u64::from(h);

    // Using From correctly
    let k: u64 = 300;
    let l = NonZeroU32::new(15).unwrap();
    let r12 = k / NonZeroU64::from(l);
}

// Additional function to test the lint in a different context
fn divide_numbers(x: u64, y: NonZeroU32) -> u64 {
    x / u64::from(y.get())
}

struct Calculator {
    value: u64,
}

impl Calculator {
    fn divide(&self, divisor: NonZeroU32) -> u64 {
        self.value / u64::from(divisor.get())
    }
}
