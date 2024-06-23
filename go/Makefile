.PHONY = \
	all \
	build \
	clean \
	run \
	test \
	test-watch

all: build

build:
	go build -o bin/nes-emulator .

clean:
	go clean -testcache
	rm -rf bin

run: build
	bin/nes-emulator

watch:
	watchexec -r make run

test:
	go test ./...

test-watch:
	watchexec make test