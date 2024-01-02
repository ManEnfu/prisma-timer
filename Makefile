fenv-clean:
	rm -rf .fenv

fenv-setup: fenv-clean
	fenv gen ./build-aux/io.github.manenfu.PrismaTimer.json
	fenv exec -- meson --prefix=/app _build

fenv-build:
	fenv exec -- ninja -C _build
	fenv exec -- ninja -C _build install

fenv-run:
	fenv exec -- prisma-timer

