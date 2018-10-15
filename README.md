accel-native
========

The purpose of this crate is to provide a simple way to call native CUDA kernels
compiled with NVCC from Rust stable. It is built on top of the awesome ['accel'
crate](https://github.com/rust-accel/accel) by Toshiki Teramura.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
accel = "0.2.0"
accel-native = "0.1.0"
```

Before building, ensure that CUDA is installed on your system. CUDA modules can be
loaded as PTX, CUBIN, or FATBIN formats. These formats are output by the NVCC
compiler. A environment variable containing the path to the module must be exported
to Cargo at build-time.

Then you, can load the module and execute CUDA kernels:

```rust
#[macro_use]
extern crate accel_native;
extern crate accel;

let module_path = Path::new(env!("INSERT_ENV_VAR_HERE"));
let module = Module::load_file(module_path).unwrap();

// argument setup ...

cuda!( module::my_function<<[Grid::x(1), Block::x(1)]>>(x) ).unwrap();
```

Note that CUDA kernels must be exported as C functions to prevent C++ name
mangling. This can be done by annotating the function with 'extern "C"':

```C
extern "C"
__global__
void my_function() { /* ... */ }
```

For a working example, see [the build file](./build.rs) and the [examples
directory](./examples).

## Platforms

The following platforms are currently tested:

* `x86_64-unknown-linux-gnu`
* `powerpc64le-unknown-linux-gnu`
