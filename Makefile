.PHONY: testdata tools apollo apollo-release synchs synchs-release sink-exp sink-exp-release

tools:
	cargo build --package=genconfig

testdata:
	@./target/debug/genconfig \
	-n 3 \
	-d 50 \
	--blocksize 400 \
	--base_port 4000 \
	--client_base_port 10000 \
	--target testdata/b400-n3

# ============= APOLLO =================================================
apollo-release: 
	cargo build --package=node-apollo --package=client-apollo --release

apollo:
	cargo build --package=node-apollo --package=client-apollo

# ============== SINK-EXP ===============================================
sink-exp-release:
	cargo build --package=streamer --package=sinker --package=relay --release

sink-exp:
	cargo build --package=streamer --package=sinker --package=relay

# ============== SYNC HOTSTUFF ==========================================
synchs-release: 
	cargo build --package=node-synchs --package=client-synchs --release

synchs:
	cargo build --package=node-synchs --package=client-synchs