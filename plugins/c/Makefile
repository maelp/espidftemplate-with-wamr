.PHONY: all clean

all: ../../resources/plugins/c-plugin.wasm

../../resources/plugins/c-plugin.wasm: plugin.c
	emcc -Os -s WASM=1 -s SIDE_MODULE=1 -s STANDALONE_WASM -s EXPORTED_FUNCTIONS='["_print_message", "_add"]' -o $@ $<

clean:
	rm -f ../../resources/plugins/c-plugin.wasm
