push:
	git add .
	git commit -m "push"
	git push
build:
	cargo clean && cargo build
	rm src/db/todos.db
	
