
ELM=elm --runtime elm-runtime.js

all: build/Demo.html

fresh: clean all

SOURCES=*.elm

ELM_VER=$(shell ghc-pkg latest Elm)

RUNTIME="build/elm-runtime.js"

${RUNTIME}: build
	cp ${HOME}/.cabal/share/${ELM_VER}/elm-runtime.js build/

build/Demo.html: ${RUNTIME} ${SOURCES}
	${ELM} --make Demo.elm 

clean:
	rm -rf cache
	rm -rf build/*

