unexport CARGO_INCREMENTAL

TARGET_NAME=thumbv7m-none-eabi
PROJECT_NAME=monocle


r:
	make rb
	arm-none-eabi-gdb target/${TARGET_NAME}/release/${PROJECT_NAME}

debug:
	make build
	make gdb

rb:
	cargo build --release --target ${TARGET_NAME}

build:
	cargo build --target ${TARGET_NAME}

gdb:
	arm-none-eabi-gdb target/${TARGET_NAME}/debug/${PROJECT_NAME}


openocd:
	# openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
	# openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -f bluepill.cfg
	openocd -f bluepill.cfg

doc:
	cargo doc --target ${TARGET_NAME}

expand:
	cargo expand --target ${TARGET_NAME}
