
for i in {1..10}
do
   near call ft_bkrt.testnet ft_transfer_call '{"receiver_id":"dev-1653427954223-52287514539272","amount":"1","memo":"Buying Lottery Ticker","msg":"Lottery ticket"}' --accountId nabeelahmed.testnet --depositYocto=1 --gas=300000000000000
done