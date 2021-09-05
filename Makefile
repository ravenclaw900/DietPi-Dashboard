default: yarn publiccopy fmt 

	cd src/backend; go build -o dietpi-dashboard

	$(MAKE) publicdelete

	mv src/backend/dietpi-dashboard .

go: publiccopy fmt

	cd src/backend; go build -o dietpi-dashboard
	
	$(MAKE) publicdelete

	mv src/backend/dietpi-dashboard .

yarn:
	cd src/frontend; yarn build

fmt:
	cd src/backend; go fmt

publiccopy:
	cp -r src/frontend/public src/backend

publicdelete:
	rm -r src/backend/public

gobuild: publiccopy fmt
	mkdir -p build/

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-w" -o dietpi-dashboard-amd64
	cd src/backend; upx dietpi-dashboard-amd64

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm go build -ldflags "-w" -o dietpi-dashboard-armhf
	cd src/backend; upx dietpi-dashboard-armhf

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -ldflags "-w" -o dietpi-dashboard-arm64
	cd src/backend; upx dietpi-dashboard-arm64

	$(MAKE) publicdelete

	mv src/backend/dietpi-dashboard-* build

build: yarn publiccopy fmt
	mkdir -p build/

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-w" -o dietpi-dashboard-amd64
	cd src/backend; upx dietpi-dashboard-amd64

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm go build -ldflags "-w" -o dietpi-dashboard-armhf
	cd src/backend; upx dietpi-dashboard-armhf

	cd src/backend; CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -ldflags "-w" -o dietpi-dashboard-arm64
	cd src/backend; upx dietpi-dashboard-arm64

	$(MAKE) publicdelete

	mv src/backend/dietpi-dashboard-* build