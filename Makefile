run:
	poetry run python main.py

config:
	poetry run python main.py --configure
install:
	poetry install
wasm:
	poetry run pygbag --html --build .
