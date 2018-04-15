.PHONY : all clean run gdb

all :
	xargo build --target=thumbv7m-none-eabi --verbose

clean :
	xargo clean

run :
	qemu-system-arm -monitor stdio -serial pty -machine lm3s811evb -cpu cortex-m3 -s -S -kernel target/thumbv7m-none-eabi/debug/laurel

gdb :
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/laurel -ex "target remote localhost:1234"

