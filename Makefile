unexport CARGO_INCREMENTAL

TARGET_NAME=thumbv7em-none-eabi

debug:
	make build
	make gdb

release:
	xargo build --release
	arm-none-eabi-gdb target/${TARGET_NAME}/release/rust-bluepill-quickstart

build:
	xargo build

gdb:
	arm-none-eabi-gdb target/${TARGET_NAME}/debug/rust-bluepill-quickstart


openocd:
	# openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
	# openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -f bluepill.cfg
	openocd -f bluepill.cfg
