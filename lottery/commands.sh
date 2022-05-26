#BUILDING OF CONTRACT
cargo build --target wasm32-unknown-unknown --release
#DEPLOYING TO DEV ACCOUNT
near dev-deploy target/wasm32-unknown-unknown/release/lottery.wasm
#SET ID TO DEPOYED ACCOUNT ADDRESS
ID=
#INITIALIZATION OF CONTRACT
near call $ID new '' --accountId $ID
#STORAGE DEPOSIT
near call lottery_ft.testnet storage_deposit '' --accountId $YOUR_ACCOUNT_ID --amount 0.0025
#SENDING FT TO LOTTERY CONTRACT SO THEY CAN AIRDROP
#AIRDROP LOTTERY TOKEN
near call $ID ft_airdrop '' --accountId nabeelahmed.testnet
#BELOW IS COMMAND OF STORAGE DEPOSIT IN FT
#near call lottery_ft.testnet storage_deposit '' --accountId nabeeel.testnet --amount 0.0025
#START LOTTERY
near call $ID start_new_lottery '{"ticket_limit":5,"ticket_price":"1000000000000000000000000","approved_ft":"lottery_ft.testnet"}' --accountId $ID
#BUY TICKET
near call lottery_ft.testnet ft_transfer_call '{"receiver_id":"'$ID'","amount":"1000000000000000000000000","memo":"Buying Lottery Ticker","msg":"Lottery ticket"}' --accountId nabeelahmed.testnet --depositYocto=1 --gas=300000000000000
#NOW PICK WINNER ONLY OWNER CAN CALL THIS
near call $ID pick_winner '' --accountId $ID
#CLAIM REWARD ONLY WINNER


