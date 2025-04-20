# RustyCompose-App
A Jetpack Compose (android) app with Rust!

### Prerequisites
The Android NDK, Rust 1.85 or higher and Linux.

### Usage
Configure the (absolute) path of the NDK-clang in `rust/.cargo/config.toml` file. This config is applicable for all crates in `rust` folder.

You can add native android dependencies to crates individually using `build.rustflags` key.

Use this command to compile a rust crate:
```
cargo b --release --target <arch>
```


Move the generated libs from `target` to the `jniLibs/<target_abi>` folders

Finally build the app!
