CC=clang
FLAGS=-O3 --target=wasm32 -nostartfiles -nostdlib -mbulk-memory -Wl,--no-entry -Wl,--allow-undefined
dir_guard=@mkdir -p $(@D)
WASM_DIR=wasm
WAT_DIR=wat
HEADER_DIR=headers

C_BENCHES:=prime murmur3 mandelbrot nbody
ASC_BENCHES:= quicksort
BENCHES=$(C_BENCHES) $(ASC_BENCHES)

WASM=$(foreach bench, $(BENCHES), $(WASM_DIR)/$(bench).wasm)
WAT=$(foreach bench, $(BENCHES), $(WAT_DIR)/$(bench).wat)
HEADERS=$(foreach bench, $(BENCHES), $(HEADER_DIR)/$(bench).wasm.h)
C_SRCS=$(addsuffix .c, $(C_BENCHES))
ASC_SRCS=$(addsuffix .ts, $(ASC_BENCHES))

all: $(WAT) $(WASM) $(HEADERS)

$(WAT_DIR)/%.wat: $(WASM_DIR)/%.wasm
	$(dir_guard)
	wasm2wat $< -o $@

$(HEADER_DIR)/%.wasm.h: $(WASM_DIR)/%.wasm
	$(dir_guard)
	xxd -n $(basename $(notdir $<))_wasm -iC $< > $@

$(WASM_DIR)/%.wasm: %.c
	$(dir_guard)
	$(CC) $(FLAGS) -o $@ $<

$(WASM_DIR)/%.wasm: %.ts
	$(dir_guard)
	asc $< -O3 -o $@ --runtime stub --initialMemory=2 --maximumMemory=2
clean:
	rm -rf $(WASM_DIR) $(WAT_DIR) $(HEADER_DIR)
