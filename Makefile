.PHONY: test clean

all: build

build:
	dune build --release
	cp _build/default/sail_plugin_virt.cmxs plugin.cmxs
	chmod +rwx plugin.cmxs

test:
	./virt-sail arch.sail -o out

clean:
	-dune clean
