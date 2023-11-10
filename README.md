## Android Position Estimator
An Android app that performs numerical integration to estimate a device's real-time position.

### Project setup
- Install [Android Studio](https://developer.android.com/studio)
- Install [Rust](https://www.rust-lang.org/learn/get-started)
- Install build toolchains for Android
```bash
rustup target add aarch64-linux-android x86_64-linux-android
```
- Install cargo-ndk
```bash
cargo install cargo-ndk
```
- Specify path to NDK via either setting `ANDROID_NDK_HOME` env variable, or ndk.dir property in `local.properties`

### Build
Android build is managed by [cargo-ndk-android-gradle](https://github.com/willir/cargo-ndk-android-gradle). Simply click `Build`, then `Run` in Android Studio.

Build and run on desktop
```bash
cargo run --features="desktop"
```