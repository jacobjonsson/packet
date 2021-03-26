packet:
	cargo build --release

angular: packet
	./target/release/cli ./fixtures/angular-1.2.5.js

jquery: packet
	./target/release/cli ./fixtures/jquery-1.9.1.js

################################################################################
# This runs the test262 official JavaScript test suite through packet

github/test262:
	mkdir -p github
	git clone --depth 1 https://github.com/tc39/test262-parser-tests.git github/test262

test262: packet
	node ./scripts/test-262.js

github/three:
	mkdir -p github
	git clone --depth 1 --branch r108 https://github.com/mrdoob/three.js.git github/three
