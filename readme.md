# Nes emulator written in rust


## Prerequisite : 

- Install the rust compiler with [rustup](https://rustup.rs/) (Don't forget to install the GNU toolchain or the MSVC Build Tools for C/C++).
- Install a 6502 assembler, my choice is [ASM6F](https://github.com/freem/asm6f).
- (Optional) Powershell (pwsh on Linux), but if you prefer to not use powershell scripts it's okay :/ 

## Build and test (WIP)

Assemble your ASM code  in the [test](./src/test/) folder under the name `test.nes`.

You can then use the command 
```powershell
cargo build
```

By default `cargo run` will launch a test. It displays informations about the code, the cpu registers and the current operation applied.
Hitting `ENTER` will advance the clock by 1.
