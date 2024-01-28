// simd.rs
#![feature(portable_simd)]
use packed_simd::i32x4;

pub fn add() {
    // Create SIMD vectors
    let a = i32x4::new(1, 1, 1, 2);
    let b = i32x4::new(5, 6, 7, 8);

    // Add the vectors using SIMD
    let result = a + b;

    // Convert the SIMD result back to a regular array
    let result_array: [i32; 4] = result.into();

    // Print the result
    println!("{:?}", result_array);
}
