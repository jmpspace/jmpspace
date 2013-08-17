
ELM=elm --runtime elm-runtime.js

all: build/index.html

fresh: clean all

SOURCES=*.elm

ELM_VER=$(shell ghc-pkg latest Elm)

RUNTIME="build/elm-runtime.js"

${RUNTIME}: build
	cp ${HOME}/.cabal/share/${ELM_VER}/elm-runtime.js build/

build/Main.html: ${RUNTIME} ${SOURCES}
	${ELM} --make Main.elm 

build/index.html: build/Main.html
	(cd build; ln -s Main.html index.html)

clean:
	rm -rf cache
	rm -rf build/*

