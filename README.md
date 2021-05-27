# DMX for i.MX6 using the Rust 'dmx' crate

The example code in this repository will a basic DMX light between Red, Green,
and Blue at full brightness.

## Cross Compiling on Your PC

### Install the armv7-unknown-linux-gnueabihf

```
rustup target add armv7-unknown-linux-gnueabihf
```

### Build for armv7 target

```
 cargo build --target armv7-unknown-linux-gnueabihf --release
```

### Copy to target

```
scp target/armv7-unknown-linux-gnueabihf/release/rust-dmx mytarget:/tmp/
```
