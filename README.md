# DMX for i.MX6 using rust-dmx

## Cross Compiling on Your PC

### Install the armv7-unknown-linux-gnueabihf

```
rustup target add armv7-unknown-linux-gnueabihf
```

### Build for armv7 target

```
 cargo build --target armv7-unknown-linux-gnueabihf --release
```

### Build for armv7 target

```
scp target/armv7-unknown-linux-gnueabihf/release/rust-dmx mytarget:/tmp/
```
