SOURCE_DOCS := $(shell find src -type f -name "*.md")

HTML_FILES=$(SOURCE_DOCS:src/%.md=site/%.html)

all: html books fix_links
	miniserve site --index index.html

deploy: html build_index
	ntl deploy --prod

html: mkdirs $(HTML_FILES)

site/%.html: src/%.md templates/site.html
	pandoc -f markdown+fenced_divs -s $< -o $@ --table-of-contents --template templates/site.html

build_index: $(SOURCE_DOCS)
	./build_index.js

fix_links: $(HTML_FILES)
	./bin/convert-html.sh

clean:
	rm -r site/**/*.html

books:
	./bin/generate_books_md.py

.PHONY: mkdirs
mkdirs:
	rsync -a --include='*/' \
	--include="*.png" \
	--include="*.jpg" \
	--include="*.jpeg" \
	--exclude='*' src/ site/
