echo ==========================================
echo
echo "BUILDING SMART CONTRACT"
echo
echo ==========================================

cargo build --target wasm32-unknown-unknown --release

echo ==========================================
echo
echo "Deploying Smart Contract"
echo 
near deploy --accountId lotteri.testnet --wasmFile target/wasm32-unknown-unknown/release/lottery.wasm

echo ==========================================
echo
echo "Buying Lottery Ticket"
echo
echo ==========================================
# near call lotteri.testnet buy_ticket '' --accountId lotteri.testnet