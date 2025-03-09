specification:
		plantuml -tsvg Špecifikácia.md -o build/images
		./markdown-transpiler < Špecifikácia.md > build/Špecifikácia-transpiled.md
		pandoc  --template eisvogel -V lang=sk -f markdown -t pdf build/Špecifikácia-transpiled.md -o Špecifikácia.pdf --resource-path build/images

clean:
	rm -rf build/ Špecifikácia.pdf
