
all: site/js/celestia.js

site/js/celestia.js: *.elm
	elm make Demo.elm --output site/js/celestia.js

clean:
	rm -rf site/js
