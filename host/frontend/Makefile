all:
	make js
	make css

js:
	elm-make src/Main.elm --output=output/index.js

css:
	elm-css src/Style.elm --module=Style
