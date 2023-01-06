# rust_os
following the blog posts on https://os.phil-opp.com/ to write an OS in Rust

## First Build

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
cargo bootimage
```
you may need to install llvm-tools before it works

```
rustup component add llvm-tools-preview
```
