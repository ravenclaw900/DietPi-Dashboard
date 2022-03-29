default: yarn publiccopy fmt

	rm src/backend/target/debug/deps/dietpi_dashboard-*

	cd src/backend; cargo build

	$(MAKE) publicdelete

	mv src/backend/target/debug/dietpi-dashboard ./dietpi-dashboard

rust: publiccopy fmt 

	cd src/backend; cargo build

	$(MAKE) publicdelete

	mv src/backend/target/debug/dietpi-dashboard ./dietpi-dashboard

yarn:
	cd src/frontend; yarn build

publiccopy:
	cp -r src/frontend/dist src/backend

publicdelete:
	rm -r src/backend/dist

fmt:
	cd src/backend; cargo fmt
ifdef TARGET
	cd src/backend; cargo clippy --target $(TARGET)
else
	cd src/backend; cargo clippy
endif

rustdev: publiccopy fmt
	cd src/backend; cargo build --target $(TARGET)
	mv src/backend/target/$(TARGET)/debug/dietpi-dashboard ./dietpi-dashboard

	$(MAKE) publicdelete

dev: yarn rustdev
