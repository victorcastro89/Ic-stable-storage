# BtreeMap working with canister stable memory

## How to Use:
dfx start --clean  
make deploy  
dfx canister call stablestorage insert_string '("ping","pong")'  
dfx canister call stablestorage get_string_by_key '("ping")'  
