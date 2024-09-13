all:
	marp --engine engine.js --html slides.md --allow-local-files
	firefox slides.html
