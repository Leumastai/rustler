
### Building Quell OS in Rust. `These are my Notes on how I built is.`

#### Stage One::
Build a Free Standing Rust Binary

Features::
-
- A rust free standing binary that doesn't need any STD library 
  or any specific system architecture or system dependent
- A **custom panic handler**, the panic handler is an important part of rust as it allow us to panic when an error occurs. There are two ways to panic in Rust, `Unwinding` and `Abort`. In this implementation, we used an `Abort` panic method
- A **custom start/entrypoint** for the code rather than using `main` entrypoint, a `_start` method is written to be the entry point of the free standing binary. Why we can't use the `main` entrypoint is beacause in most programming language, the entrypoint of the code is not always main. It usually creates a runtime routine before entring the main. So in rust, the runtime routine is a `crt0` (C runtime Zero) which build the routine before the rust runtime then the main code. 
And since this should be a free standing binary in rust, we don't need to be using a crt0 before our code runs.
- enabling **no_mangle**, mangling is the process in which the compiler renames the fuction so as not to cause conflicts during function overloading. No mangle now tries to disable that and just use the fuction name, usually useful when you want to interfatye with low level fucntion or connect with uther  language with your fuction.

Drawback / Additions::
- 
- Implement a robust Panic method in free standing Rust binary
- Implement a more relaible start method.

#### Stage Two::
Building a Minimal OS

Featuesres::
-
- **Target specification** for the OS to understand the CPU architecture, vendor, OS, and the ABI. e.g:: `x86_64-unknown-linux-gnu` the specificaiton has a x86_64 CPU architecturem and unknown vendor, a linux os and a GNU ABI.
Rust allows us to define our own specification system without needing the STL in a json configuarations. 
```
{
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float",
    "os": "none",
    "executables": true
}
```

- **Building the Kernel** to build the kernel we would run `cargo build --target x85_64-quell_os.json` but the line would throw an error as our target specification is not installed within the pre-compiled rust core library. So we'll have to re-compile the core library on our own using rust nigtly and creating a folder called `.cargo/cargo.toml` and run `rustup component add rust-src`

- **Enabling the compiler buitins** to enable memory intrinsics, some fuction in C is needed to manage memory such as memcpy, memcmp, memeset, etc. Writing our own might be a bit of a hassle. we use the written function from the compiled core library.

- **Creating a bootimage**  to turn our compiled kernel into a bootable disk image, as we'll link it to a bootloader. we use a deoendency directly fr5om rust to assembly without the need for any C.
first, install `cargo install bootimage` in your home directory, then run `rustup component add llvm-tools-preview .` Then create a bootable disk by going back to your working folder and run `cargo bootimage`

- **Boot the image on a machine** using the QEUM vm with this line `qemu-system-x86_64 -drive format=raw,file=target/x86_64-quell_os/debug/bootimage-quell_os.bin`