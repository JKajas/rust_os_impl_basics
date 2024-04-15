LD_SCRIPT_PATH 		= $(shell pwd)/src/bsp/raspberrypi/
export LD_SCRIPT_PATH
TARGET            = aarch64-unknown-none-softfloat
KERNEL_BIN        = kernel8.img
QEMU_BINARY 			= qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi4b
CPU_TARGET        = cortex-a72
RUSTC_MISC_ARGS 	= -C target-cpu=$(CPU_TARGET)
KERNEL_MANIFEST = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel.ld
LAST_BUILD_CONFIG = target/rpi4.build_config
KERNEL_ELF      = target/$(TARGET)/release/kernel
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST) $(LAST_BUILD_CONFIG)
LAST_BUILD_CONFIG    = target/rpi4.build_config
RUSTFLAGS = $(RUSTC_MISC_ARGS) \
						-C link-arg=--library-path=$(LD_SCRIPT_PATH) \
						-C link-arg=--script=$(KERNEL_LINKER_SCRIPT)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) \
    -D warnings                   \
    -D missing_docs
FEATURES = --features bsp_rpi4
COMPILER_ARGS = --target=$(TARGET)\
								$(FEATURES)\
								--release

RUSTC_CMD = cargo rustc $(COMPILER_ARGS)
EXEC_QEMU = $(QEMU_BINARY) -M $(OEMU_MACHINE_TYPE)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

build: $(KERNEL_BIN)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	@echo $(KERNEL_BIN)



$(KERNEL_ELF):$(KERNEL_ELF_DEPS)
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)

qemu: $(KERNEL_BIN)
	qemu-system-aarch64 -M $(QEMU_MACHINE_TYPE) -cpu $(CPU_TARGET) -kernel $(KERNEL_BIN) -monitor telnet:127.0.0.1:55555,server,nowait -serial stdio
	#-serial telnet:127.0.0.1:55554,server,nowait -display none

monitor:
	telnet 127.0.0.1 55555


objdump: $(KERNEL_ELF)
	@aarch64-none-elf-objdump -d $(KERNEL_ELF)

readelf: $(KERNEL_ELF)
	@aarch64-none-elf-readelf -s  $(KERNEL_ELF)
