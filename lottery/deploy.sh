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
near deploy --accountId dev-1653124157396-95960748670756 --wasmFile target/wasm32-unknown-unknown/release/lottery.wasm

