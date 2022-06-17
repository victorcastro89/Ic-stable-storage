.PHONY: init candid build local stop-replica test format lint clean

LOCAL_CUSTODIAN_PRINCIPAL=$(shell dfx identity get-principal)
TEST_CUSTODIAN_PRINCIPAL=$(shell cat test/custodian-test-principal)

init:
	# npm --prefix test i
	cargo check

# build:
# 	cargo run -p manager > ./manager/manager.did       
# 	# didc bind -t ts nft.did > test/factory/idl.d.ts
# 	# echo "// @ts-nocheck" > test/factory/idl.ts
# 	# didc bind -t js nft.did >> test/factory/idl.ts

deploy:
	cargo run  > storage.did      
	dfx deploy
# build: candid
# 	dfx ping local || dfx start --clean --background
# 	dfx canister create nft
# 	dfx build nft

# local: build
# 	dfx deploy nft --argument 'opt record{name=opt "Vmonster"; logo= opt "Vmonster Logo"; symbol=opt "VM"; custodians=opt vec {principal "$(LOCAL_CUSTODIAN_PRINCIPAL)"}}'


# stop-replica:
# 	dfx stop

# test: stop-replica build
# 	dfx canister install nft --argument '(opt record{custodians=opt vec{principal"$(TEST_CUSTODIAN_PRINCIPAL)"}})'
# 	npm --prefix test t
# 	dfx stop

# format:
# 	npm --prefix test run prettier
# 	npm --prefix test run lint
# 	cargo fmt --all

# lint:
# 	npm --prefix test run prebuild
# 	cargo fmt --all -- --check
# 	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all

# clean:
# 	cargo clean
# 	npm --prefix test run clean


# mint:
# 	 dfx canister call nft mint "(principal\"i5jdn-42uan-vrsev-iphlu-3zcrf-vxpjv-sl3td-ucvdm-jqy26-rbivz-qqe\", record{  blob_content=blob\"Fotinha do milton\" ; name=\"Milton Na Estrada\" ; key_val_data=vec{record{\"contentType\";variant{TextContent=\"text/plain\"};}}})"
 
