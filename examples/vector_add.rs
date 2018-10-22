/*
 * Copyright 2018 German Research Center for Artificial Intelligence (DFKI)
 * Author: Clemens Lutz <clemens.lutz@dfki.de>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate accel_native;
extern crate accel;

use accel::module::Module;
use accel::device::sync;
use accel::kernel::{Block,Grid};
use accel::uvec::UVec;

use std::path::Path;


fn main() {

    // Fetch path from build.rs
    let module_path = Path::new(env!("ADD_PATH"));
    println!(
        "Module found at {}",
        module_path.to_str().expect("Cannot get path string"));

    // Force CUDA to init device and create context
    sync().unwrap();

    // Load CUDA module that contains kernel
    let module = Module::load_file(module_path)
        .expect("Cannot load CUDA module");

    // Create arguments
    let len = 128_u64;
    let mut a = UVec::new(len as usize).unwrap();
    let mut b = UVec::new(len as usize).unwrap();

    // Initialize arguments
    a
        .iter_mut()
        .enumerate()
        .map(|(i, elem)| {
            *elem = i as i64;
        })
    .collect::<()>();

    b
        .iter_mut()
        .enumerate()
        .map(|(i, elem)| {
            *elem = i as i64;
        })
    .collect::<()>();

    // Call CUDA kernel
    cuda!( vector_add<<[module, Grid::x(1), Block::x(1)]>>(len, a, b) )
        .expect("Cannot launch CUDA kernel");

    // Wait for completion
    sync().unwrap();
    println!("Kernel returned: {}", a[1]);
}

