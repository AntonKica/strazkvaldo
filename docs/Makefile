specification:
		plantuml -tsvg specification.md -o build/images
		./markdown-transpiler < specification.md > build/specification-transpiled.md
	pandoc --template eisvogel -V lang=sk -V titlepage=true --toc -V toc-own-page=true \
		-f markdown -t pdf build/specification-transpiled.md -o specification.pdf \
		--resource-path build/images
clean:
	rm -rf build/ specification.pdf
