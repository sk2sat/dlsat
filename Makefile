build: build-srv build-front

build-srv:
	cargo build

build-front:
	make -C ui build

install-front:
	make -C ui install

run: build
	cargo run

ci: build install-front

clean: clean-srv clean-front

clean-srv:
	cargo clean

clean-front:
	make -C ui clean
