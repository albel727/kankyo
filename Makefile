.PHONY : all test_env test_lib

all : test_env test_lib

test_env :
	@cd test_app && cargo update && cargo test && cd ..;

test_lib :
	echo "A=B\nC=D" > .env;
	cargo test;
	rm .env;
