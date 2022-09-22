all: run

BUILD = cargo build --release
TARGET = target/release/prj

build:
	${BUILD}

run: 
	${BUILD} && ./${TARGET}

install:
	./install.sh

clean:
	rm -rfv target 
