all:
	marp --engine engine.js --html slides\ copy.md
	firefox slides\ copy.html

# --allow-local-files