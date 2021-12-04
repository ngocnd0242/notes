SOURCE_DOCS := $(shell find src -type f -name "*.md")

HTML_FILES=$(SOURCE_DOCS:src/%.md=site/%.html)

all: html fix_links
	miniserve site

deploy: html build_index
	ntl deploy --prod

html: $(HTML_FILES)

site/%.html: src/%.md templates/site.html
	pandoc -f markdown+fenced_divs -s $< -o $@ --table-of-contents --template templates/site.html

build_index: $(SOURCE_DOCS)
	./build_index.js

fix_links: $(SOURCE_DOCS)
	sed -i 's/\.md/\.html/g' site/**/*.html
	sed -i 's/\.md/\.html/g' site/*.html

clean:
	rm -r site/*

# .PHONY: mkdirs
# mkdirs:
# 	mkdir site
# 	find src -type d -print0 >dirs.txt
# 	xargs -0 mkdir -p <dirs.txt
# 	rm dirs.txt
