#####
## CONFIG
#####
NAME=minimalist
TYPE=debug
OUT=target/riscv64gc-unknown-none-elf/$(TYPE)/$(NAME)

#####
## QEMU
#####
QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=128M
DRIVE=hdd.dsk

all:
	cargo build

run: all
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) -nographic -serial mon:stdio -bios none -kernel $(OUT) -drive if=none,format=raw,file=$(DRIVE),id=foo -device virtio-blk-device,drive=foo

.PHONY: clean
clean:
	cargo clean