# STM32F411 Hello World in Rust

Based on https://dev.to/apollolabsbin/stm32f4-embedded-rust-at-the-pac-uart-communication-djp

## Installing the prerequisites

- Install the Rust tools as described in the Rust manual

- Configure the target for the STM32F411 microcontroller
```bash
rustup target add thumbv7em-none-eabihf
```

- Install the tools for this target
```bash
cargo install cargo-binutils
rustup component add llvm-tools
```

- Install GDB
```bash
# For Ubuntu-based distro
sudo apt install gdb-multiarch
```

## Building the binary

```bash
cargo build
cargo objcopy --bin stm32f411_helloworld -- -O binary stm32f411_helloworld.bin
```

## Building QEMU for STM32F411

```bash
git clone https://github.com/juliencombattelli/qemu-st-nucleo-f411
mkdir qemu-st-nucleo-f411-build
cd qemu-st-nucleo-f411-build
sudo apt install libglib2.0-dev libpixman-1-dev
../qemu-st-nucleo-f411/configure --target-list=arm-softmmu
make -j$(nproc)
export PATH=$(pwd):$PATH
```

## Testing on QEMU

```bash
qemu-system-arm -machine st-nucleo-f411 -kernel stm32f411_helloworld.bin -nographic -monitor none -serial null -serial stdio
```

## Debugging on QEMU with GDB
```bash
# In one terminal
qemu-system-arm -machine st-nucleo-f411 -kernel stm32f411_helloworld.bin -nographic -monitor none -serial null -serial stdio -s -S
# In a second terminal
gdb-multiarch stm32f411_helloworld -ex "target remote :1234" -ex "b main"
```
