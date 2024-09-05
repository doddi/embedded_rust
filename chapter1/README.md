# Chapter 1

This is the first chapter the introduces you to what is needed to get a
basic build of a hello world program.

## Set up / Tooling

This project documents how to get a micro:bit v1 controller set up.

# Step 1 - Installing Rust toolchain

Follow the guide at rust.org to install the rust toolchain at
https://www.rust-lang.org/tools/install

At this point you should be able to build and run a program on your host
machine.

# Step 2 - Installing the cross compiler

Installing the cross compiler for the micro:bit v1 controller.

```bash
rustup target add thumbv6m-none-eabi
```

Note: This is targeting the v1 version of the micro:bit controller.

# Step 3 - Installing the tooling

```bash
rustup component add llvm-tools
```

This is component is useful for getting information on the generated images.

```bash
cargo install cargo-binstall
cargo binstall probe-rs-tools
cargo install cargo-probe-rs
cargo bininstall probe-rs-tools
```

At this point you should have all the necessary tools to build and flash the
micro:bit controller.

## Embedded project notes

After installing all the tooling, an important to use is `cargo embed`. It is
this command that will flash the micro:bit controller with the generated image.

To limit the number of arguments needed `cargo embed` looks for an Embed.toml
file at the root of the project which determines the target to use.

### Embed.toml

```toml
[default.general]
chip = "nRF51822_xxAA"
```

Additionally the option to use RTTI for logging purposes is also enabled

```toml
[default.rtt]
enabled = true
```

### cargo.toml

```toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv7m-none-eabi]
rustflags = "link-arg=-Tlink.x"
```

located in the local .cargo directory. Is what tells cargo to cross compile the

### memory.x

Is your linker script that tells the compiler where to place the generated
RAM and ROM sections.

## Build and run

```bash
cargo size -- -Ax
```

Will give information on the size of the generated image.

Ensure you micro:bit controller is connected to your computer via the USB

If all looks good run the command:

```bash
cargo embed
```

that should build and flash you micro:bit controller.

With RTTI enabled if all went well you should see text output on the console.
