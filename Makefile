default: yarn publiccopy fmt 

	cd src/backend; cargo build

	$(MAKE) publicdelete

	mv src/backend/target/debug/dietpi_dashboard ./dietpi-dashboard

rust: publiccopy fmt 

	cd src/backend; cargo build
	
	$(MAKE) publicdelete

	mv src/backend/target/debug/dietpi_dashboard ./dietpi-dashboard

yarn:
	cd src/frontend; yarn build

publiccopy:
	cp -r src/frontend/public src/backend

publicdelete:
	rm -r src/backend/public

fmt:
	cd src/backend; cargo fmt
	cd src/backend; cargo clippy

rustdev: publiccopy fmt
	cd src/backend; cargo build --target $(TARGET)
	mv src/backend/target/$(TARGET)/debug/dietpi_dashboard ./dietpi-dashboard

	$(MAKE) publicdelete

dev: yarn rustdev

rustbuild: publiccopy fmt
	mkdir -p build/

	cd src/backend; cargo build --release --target x86_64-unknown-linux-gnu
	x86_64-linux-gnu-strip src/backend/target/x86_64-unknown-linux-gnu/release/dietpi_dashboard
	upx --lzma src/backend/target/x86_64-unknown-linux-gnu/release/dietpi_dashboard
	mv src/backend/target/x86_64-unknown-linux-gnu/release/dietpi_dashboard build/dietpi-dashboard-amd64

	cd src/backend; cargo build --release --target arm-unknown-linux-gnueabihf
	/opt/rpi/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf-strip src/backend/target/arm-unknown-linux-gnueabihf/release/dietpi_dashboard
	upx --lzma src/backend/target/arm-unknown-linux-gnueabihf/release/dietpi_dashboard
	mv src/backend/target/arm-unknown-linux-gnueabihf/release/dietpi_dashboard build/dietpi-dashboard-armv6

	cd src/backend; cargo build --release --target armv7-unknown-linux-gnueabihf
	arm-linux-gnueabihf-strip src/backend/target/armv7-unknown-linux-gnueabihf/release/dietpi_dashboard
	upx --lzma src/backend/target/armv7-unknown-linux-gnueabihf/release/dietpi_dashboard
	mv src/backend/target/armv7-unknown-linux-gnueabihf/release/dietpi_dashboard build/dietpi-dashboard-armv7

	cd src/backend; cargo build --release --target aarch64-unknown-linux-gnu
	aarch64-linux-gnu-strip src/backend/target/aarch64-unknown-linux-gnu/release/dietpi_dashboard
	upx --lzma src/backend/target/aarch64-unknown-linux-gnu/release/dietpi_dashboard
	mv src/backend/target/aarch64-unknown-linux-gnu/release/dietpi_dashboard build/dietpi-dashboard-armv8

	$(MAKE) publicdelete

build: yarn rustbuild