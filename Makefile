build: yarn rustbuild

rustbuild: distcopy fmt
ifdef TARGET
	cd src/backend; cargo build --target $(TARGET)
	mv src/backend/target/$(TARGET)/debug/dietpi-dashboard .
else
	cd src/backend; cargo build
	mv src/backend/target/debug/dietpi-dashboard .
endif

release: yarn compress
ifdef TARGET
	cd src/backend; cargo build --target $(TARGET) --release --features compression
	mv src/backend/target/$(TARGET)/release/dietpi-dashboard .
else
	cd src/backend; cargo build --release --features compression
	mv src/backend/target/release/dietpi-dashboard .
endif

	rm -r src/backend/dist

# There may be a better, more 'make'y way of doing this, but find works for now
compress: distcopy
	find src/backend/dist ! -name '*.png' -type f -exec gzip -9 {} \; -exec mv {}.gz {} \;

yarn:
	cd src/frontend; yarn build

ifdef TARGET
	rm -f src/backend/target/$(TARGET)/debug/deps/dietpi_dashboard-*
else
	rm -f src/backend/target/debug/deps/dietpi_dashboard-*
endif

fmt:
	cd src/backend; cargo fmt
ifdef TARGET
	cd src/backend; cargo clippy --target $(TARGET)
else
	cd src/backend; cargo clippy
endif

distcopy:
	rm -rf src/backend/dist
	cp -r src/frontend/dist src/backend
