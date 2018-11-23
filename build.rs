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

use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let args = vec![
"./examples/add.cu",
        "-std=c++11",
        "-fatbin",
        "-gencode", "arch=compute_30,code=sm_30", // Tesla K40
        "-gencode", "arch=compute_50,code=sm_50", // GTX 940M
        "-gencode", "arch=compute_52,code=sm_52", // GTX 980
        "-gencode", "arch=compute_61,code=sm_61", // GTX 1080
        "-gencode", "arch=compute_70,code=sm_70", // Tesla V100
        "-o"
    ];

    let output = Command::new("nvcc").args(args.as_slice())
        .arg(&format!("{}/add.fatbin", out_dir))
        .output().expect("Couldn't execute nvcc");

    if !output.status.success() {
        eprintln!("status: {}", output.status);
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!();
    }

    println!("cargo:rustc-env=ADD_PATH={}/add.fatbin", out_dir);
    println!("cargo:rustc-link-search=native=/opt/cuda/lib64");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
}
