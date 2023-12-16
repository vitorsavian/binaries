build-grep:
	cargo build --bin grep

build-ls:
	cargo build --bin ls

build-cat:
	cargo build --bin cat

build-pwd:
	cargo build --bin pwd

grep: build-grep
	./target/debug/grep $(filter-out $@,$(MAKECMDGOALS))

ls: build-ls
	./target/debug/ls $(filter-out $@,$(MAKECMDGOALS))

cat: build-cat
	./target/debug/cat $(filter-out $@,$(MAKECMDGOALS))

pwd: build-pwd
	./target/debug/pwd

%:
	@: