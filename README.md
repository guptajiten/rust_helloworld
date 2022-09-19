## Rust Hello World
### Setup and Debug Rust HelloWorld(MacOS) in VSCODE
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. brew tap messense/macos-cross-toolchains
3. brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu
    a. x86_64-unknown-linux-gnu should work for both intel and M1 chips
4. Enable 'Debug: Allow Breakpoints Everywhere' in VSCODE 
5. You'll need to install an extension. Which one depends on your platform.
    a. C/C++ (Windows)
    b. CodeLLDB (OS X / Linux)
6. rustc --version
7. cargo build
8. Open .vscode/ws.code-workspace
9. Add a breakpoint in (src/main.rs)
10. Select your debug launch config
11. Press F5

### How to debug UnitTest HelloWorld(MacOS) in VSCODE



### FAQ in RUST
1. [Difference between if-else and if let](https://www.geeksforgeeks.org/rust-if-let-operator/#:~:text=The%20basic%20difference%20between%20conventional,is%20assigned%20according%20to%20it.)
2. Cargo - Cargo is package manager for Rust
3. Breakpoint not hitting - whenever you make changes, do 'cargo build' again and then hit F5.