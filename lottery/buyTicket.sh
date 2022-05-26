
for i in {3..10}
do
near call lottery_ft.testnet ft_transfer_call '{"receiver_id":"'$ID'","amount":"1000000000000000000000000","memo":"Buying Lottery Ticker","msg":"Lottery ticket"}' --accountId nabeelahmed.testnet --depositYocto=1 --gas=300000000000000
done