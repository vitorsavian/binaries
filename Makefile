build-grep:
	cargo build --bin grep

build-ls:
	cargo build --bin ls

build-cat:
	cargo build --bin cat

build-pwd:
	cargo build --bin pwd

grep: build-grep
	./target/debug/grep $(ARGS)

ls: build-ls
	./target/debug/ls $(ARGS)

cat: build-cat
	./target/debug/cat $(ARGS)		

pwd: build-pwd
	./target/debug/pwd $(ARGS)