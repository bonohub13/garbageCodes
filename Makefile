COMPILER := $(shell which cargo)

clean:
	$(COMPILER) clean

build: clean
	$(COMPILER) build

run: build
	$(COMPILER) run
