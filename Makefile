COMPONENTS := target/comp.wasm

TARGET=wasm32-unknown-unknown
#TARGET=wasm32-wasi
ADAPT=wasi_snapshot_preview1.reactor.wasm
components: $(COMPONENTS)

components_unstripped: $(COMPONENTS:.wasm=.unstripped.wasm)

target/%.unstripped.wasm: components/% components/%/src/*.rs components/%/wit/*.wit
	cd $< && cargo build --release --target=$(TARGET)
	wasm-tools component new components/*/target/$(TARGET)/release/$*.wasm --adapt $(ADAPT) -o target/$*.unstripped.wasm

%.wasm: %.unstripped.wasm
	wasm-tools strip $< -o $@
