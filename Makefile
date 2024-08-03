MAKEFLAGS += --no-print-directory

.SILENT:
.PHONY: src, run, run-devel, target, build, rc, rs, r, b, bc, watch, w, clean, c, install, i, watchexec, we, help

# don't chang this shit

SRC_DIR := ./lib
TARGET_DIR := $(SRC_DIR)/target
BIN_DIR := $(TARGET_DIR)/release
RELEASE_BINARY_NAME := autovirt
RELEASE_BINARY_PATH := $(BIN_DIR)/$(RELEASE_BINARY_NAME)
RELEASE_RELATIVE_PATH := target/release/$(RELEASE_BINARY_NAME)
ORIGIN_DIR := $(shell pwd)

# shorthandns  cos im lazy af and fast

h: help
r: run
b: build
bc: buildcopy
w: watch
we: watchexec
c: clean
i: install
build: b

# full help list with proper formatting
help:
	@echo "Makefile Help list"
	@echo ""
	@echo "Commands:"
	@printf "  run        [r] \tRuns the rust code\n"
	@printf "  build      [b] \tBuilds rust code\n"
	@printf "  buildcopy  [bc]\tBuilds & copies binary to root dir\n"
	@printf "  watch      [w] \tCargo watch-es the code for hot reloads\n"
	@printf "  watchexec  [we]\tWatchexec's the code for hot reloads\n"
	@printf "  clean      [c] \tCleans all leftover build targets & others\n"
	@printf "  install    [i] \tInstalls AutoVirt via the install script\n"
	@printf "  bci        []  \tBulid copy and install\n"
	@echo ""
	@printf "  help       [h] \tShows this thing\n"



# rUn

run:
	@echo "Running..."
	cd $(SRC_DIR); \
		printf "Entering Directory\t>\t$$PWD\n" ; \
		printf "Running CMD\t\t>\t'cargo run'\n\n" ; \
		cargo run;

# bld

build:
	@echo "Building..."
	cd $(SRC_DIR); \
		printf "Entering Directory\t>\t$$PWD\n" ; \
		printf "Running CMD\t\t>\t'cargo build'\n\n" ; \
		cargo build --release;

# testing the binary (testing commands)

test:
	./$(BIN_DIR)/$(RELEASE_BINARY_NAME) --help


# test the binary with hot reloading
watchtest:
	watchexec -w $(SRC_DIR) -r "cd $(SRC_DIR) && cargo build --release && ./$(RELEASE_RELATIVE_PATH) --help"
	# ./$(BIN_DIR)/$(RELEASE_BINARY_NAME) --help

