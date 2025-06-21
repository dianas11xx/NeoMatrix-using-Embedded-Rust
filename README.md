# Programming Animations on NeoMatrix Display Using Embedded Rust 

## Description

This project uses the Rust programming language to create an embedded system that displays 4 seperate animations on the NeoMatrix display based on the orientation of the device using the LIS3DH accelerometer over I2C. 

The boards used in this project are:
- Adafruit Feather RP2040 
- Adafruit Propmaker FeatherWing
- Adafruit 8x8 NeoMatrix

## Project Structure 
---
### Src Folder
The source folder contains the `main.rs` and `animations.rs` files, which serve as the application code. 

#### Main.rs
The code in this file instantiates the application by importing the crates used to access the on-chip peripherals. This allows the use of the accelerometer over i2c and the NeoMatrix via ws2812 pio. 

The main creates used in this file were: 
- `adafruit_feather_rp2040`: Used to access pre-defined functions/traits from the rp2040_hal and pre-configured pins for the Feather RP2040.
- `rp2040_hal`: Used to access on-chip peripherals like clock, sio, gpio, i2c, and timer in order to initialize/define them for application use. 
- `embedded_hal`: Used to enable a common abstraction accross all peripheral drivers for specific devices like LIS3DH.
- `ws2812_pio`: Used to set up the pio state machines to drive the ws2812 LED's on the NeoMatrix. 
- `smart_leds`: Used to access RGB8 type for declaring pixel values and access the SmartLedsWrite trait to drive a string of LED's using a list of RGB8 values created in the animations.rs file.

#### Psuedocode for main.rs
- Initialize pins to default state
- Set up propmaker power enable pin and drive it high for the NeoMatrix
- Import animations module and instantiate each one
- Initialize the i2c and accelerometor by configuring scl and sda pins 
- Create i2c driver
- Initialize device orientation
- Loop:
    - Get accelerometer vector from LIS3DH
    - Update the trackers internal state using the accelerometer vector
    - Set the animation mode based on the current orientation of the device
    - Iterate through the animation modes by calling thier `next()` methods
    - Select the animation list to read based on the current mode using the `to_list()` method.
    - Write the list to the LED's 

 #### Animations.rs
 The code in this module implements four different animations as their own structs for the NeoMatrix display. Each animation has the four following methods:
 - `new()`: Instantiates the animation for use in the application code.
 - `set()`: Sets each pixel to a specific color based on the current frame in the animation 
 - `to_list()`: Converts the current frame, that was set in the set() method, to a list of sixty-four 32-bit color values so it could be sent to the NeoMatrix.
 - `next()`: Updates the animation to the next frame

 I implemented the following 4 animations:
 - A color-changing spiral that draws different colors across the display
 - A multi-pulsing heart animation that increases/decreases the color intensity of the heart
 - A color-changing pac-man ghost animation that moves its 'legs'
 - A color-changing firework animation

 ### Configuration Files
 In order to configure the compiler, build, and load the execuatable, several files are needed such as:
 - `Cargo.toml`: Defines the name and version of the package along with the dependencies (crates) used in the application.
 - `.cargo/config `: Defines the target architecture, compiler/linker flags, and runner of the chip for the builds system.
 - `memory.x`: Linker script that defines relevant memory addresses.

 ## How to Build
 ---
 Since this project uses the Rust toolchain, you need to install the `rustup` tool and `cargo` runner before you build.

 - Download and `extract` the ZIP archive
 - `cd` into the folder and run `cargo run` 

 This will `compile` and `load` the application to the board.

 ## How to Load Code 
 --- 
 After building the code, if you are on a Windows or native Linux system, it should automatically load the program to your board. 
 
 If you're on WSL2, please follow the following instructions to load the code to your board:
 - Remove the `-d flag` from the runner at line 28 in the `.config/cargo` file
 - Run `cargo run` in your terminal
 - Reboot the board by holding down the `BOOLSEL` button on the Adafruit Feather rp2040 while clicking the `RESET` button or powering it on.
 - Drag or copy the `Embedded-Rust_Lab.uf2` files from the `target/thumbv6m-none-eabi/debug` folder onto the filesystem

 This will flash the board with the compiled code via the built-in ROM UF2 bootloader.
