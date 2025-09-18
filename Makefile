push:
	cargo clean
	git add .
	git commit -m "push"
	git push
build:
	cargo clean && cargo build
	rm src/db/todos.db
	
clean:
	cargo clean
	rm src/db/todos.db
