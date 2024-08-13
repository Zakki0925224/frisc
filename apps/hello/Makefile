SRC_FILES := hello.c
OBJ_FILES := $(SRC_FILES:.c=.o)
OUT_FILE := hello.elf

CC := clang
CFLAGS := --target=riscv32 -march=rv32i -mabi=ilp32 -nostdlib

LD := ld.lld
LDFLAGS := -z norelro --static

%.o: %.c
	$(CC) -c $< -o $@ $(CFLAGS)

$(OUT_FILE): $(OBJ_FILES)
	$(LD) $(OBJ_FILES) -o $@ $(LDFLAGS)

clean:
	rm -f $(OBJ_FILES) $(OUT_FILE)

all: $(OUT_FILE)

.PHONY: clean all
