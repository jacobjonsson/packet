PACKET_PATH = ./target/release/cli

packet:
	cargo build --release

################################################################################
# This runs the test262 official JavaScript test suite through packet

github/test262:
	mkdir -p github
	git clone --depth 1 https://github.com/tc39/test262-parser-tests.git github/test262

test262: packet
	node ./scripts/test-262.js
