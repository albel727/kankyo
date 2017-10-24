.PHONY : all test_env test_lib

all : test_env test_lib

test_env :
	@cd test_app && cargo test && cd ..;

test_lib :
	cargo test;
