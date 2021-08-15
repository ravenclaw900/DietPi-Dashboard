default:
	mkdir -p build/

	cd src/frontend; yarn build

	cp -r src/frontend/public src/backend

	cd src/backend; go fmt

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o dietpi-dashboard-amd64
	cd src/backend; upx dietpi-dashboard-amd64

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm go build -o dietpi-dashboard-armhf
	cd src/backend; upx dietpi-dashboard-armhf

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -o dietpi-dashboard-arm64
	cd src/backend; upx dietpi-dashboard-arm64

	rm -r src/backend/public

	mv src/backend/dietpi-dashboard-* build