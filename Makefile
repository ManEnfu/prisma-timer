.PHONY: all

all:
	@echo 'Targets: '
	@echo 'clean - clean up build artifacts'
	@echo 'setup - setup flatpak environment'
	@echo 'build - build in flatpak environment'
	@echo 'run   - run in flatpak environment'

clean:
	rm -rf .fenv .flatpak-builder _build

setup: clean
	fenv gen ./build-aux/io.github.manenfu.PrismaTimer.json
	fenv exec -- meson --prefix=/app _build

build:
	fenv exec -- ninja -C _build install

run:
	fenv exec -- prisma-timer

