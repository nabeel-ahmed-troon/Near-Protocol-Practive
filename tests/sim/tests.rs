use near_sdk::json_types::U128;
use near_sdk_sim::{call, to_yocto, transaction::ExecutionStatus, view};
use std::{thread, time};
use near_sdk::AccountId;
use near_sdk_sim::UserAccount;

use crate::utils::{init, register_user};


#[test]
fn simulate_total_supply() {
    let initial_balance = to_yocto("100");

    let (_, ftt, _, _,_,_) = init(initial_balance);
    println!("{:#?}",ftt.user_account);
    let total_supply: U128 = view!(ftt.ft_total_supply()).unwrap_json();
    assert_eq!(initial_balance, total_supply.0);
}


#[test]
fn buy_lottery_ticket(){
    let initial_balance=to_yocto("10000000000");
    let (root, ftt, lottery, nft,alice,nabeel) = init(initial_balance);
    //<=================================================================>
    //             DEMO ACCOUNT CREATION FOR GETTING AIRDROP
    //<=================================================================>
    let demo=root.create_user("demo_account".to_string().parse().unwrap(), to_yocto("1000"));
    register_user(&demo);
    //
    //<=================================================================>
    //                      STARTING THE LOTTERY
    //<=================================================================>
    call!(
        alice,
        lottery.start_new_lottery(5,U128::from(1000000000000000000000000),"ft".to_string().parse().unwrap(),"nft".to_string().parse().unwrap())
    ).assert_success();
    //<=================================================================>
    //                  STORAGE DEPOSIT FOR LOTTERY CONTRACT
    //            IF NOT THEN WE CANNOT SEND TOKENS TO LOTTERY CONTRACT
    //<=================================================================>
    call!(
        demo,
        ftt.storage_deposit(Some("lottery".to_string().parse().unwrap()),None),
        deposit=to_yocto("3")
    ).assert_success();
    //
    //<=================================================================>
    //                  TRANSFERING FUNGIBLE TOKEN 
    //              TO LOTTERY CONTRACT FOR AIRDROPING
    //<=================================================================>
    let _amount= to_yocto("1000");
    call!(
        root,
        ftt.ft_transfer(lottery.account_id(),U128::from(_amount),Some("Testing Fungible Token".to_string())),
        deposit=1 
    ).assert_success();

    //CHECKING FT RECEIVED OR NOT
    let check= view!(ftt.ft_balance_of(alice.account_id())).unwrap_json_value();
    println!("balance of alice is : {}",check);

    //GETTING APPROVED FT
    let res=view!(lottery.get_approved_ft()).unwrap_json_value();
    println!("Approved Ft is  : {:#?}",res);
    //GETTING LOTTERY OWNER
    let res= view!(lottery.get_lottery_owner()).unwrap_json_value();
    println!("Lottery Owner: {}",res);

    let root_balance=view!(ftt.ft_balance_of(root.account_id())).unwrap_json_value();
    println!("Balance befor buying ticket : {}",root_balance);
    
    //checking balance of lottery account before buying
    //<=================================================================>
    //              GETTING BALANCE OF LOTTERY ACCOUNT
    //                     AFTER SENDING FT TOKENS
    //<=================================================================>
    let check= view!(ftt.ft_balance_of(lottery.account_id())).unwrap_json_value();
    println!("balance of lottery before buy : {}",check);
    //<========================================================================================>

    //<========================================================================================>
    // let res = call!(
    //     root,
    //     ftt.ft_transfer_call(lottery.account_id(),U128::from(1000000000000000000000000),None,"lottery buying hurah!".to_string()),
    //     deposit=1
    // );
    //<=================================================================>
    //                      BUYING LOTTERY TICKETS
    //<=================================================================>
    for i in 1..11{
        call!(
            root,
    
            ftt.ft_transfer_call(lottery.account_id(),U128::from(1000000000000000000000000),None,"lottery buying hurah!".to_string()),
            deposit=1
        );
    }

    //<=================================================================>
    //                      PICKING RANDOM WINNER
    //<=================================================================>
    let res=call!(
        alice,
        lottery.pick_winner()
    );

    // println!("Pick winner promise : {:#?}",res.promise_errors());

    // let root_balance=view!(ftt.ft_balance_of(root.account_id())).unwrap_json_value();
    // println!("Balance after buying ticket : {}",root_balance);

    //<=================================================================>
    //              GETTING ALL PLAYERS
    //<=================================================================>
    let res= view!(lottery.get_players()).unwrap_json_value();
    println!("player in lottery : {}",res);

    //<=================================================================>
    //                      WINNER IS
    //<=================================================================>
    let res = view!(lottery.get_winner_of_lottery());
    // let winner:String= res.unwrap_json();
    // println!("winnner of lottery is ; {}",winner);
    // let s= UserAccount::;

    //NFT CHECKING OF ROOT

    let res=view!(nft.nft_supply_for_owner(root.account_id())).unwrap_json_value();
    println!(
        "Nft supply before claim reward{}",res
    );
     //<=================================================================>
    //                       CLAIM REWARD
    //<=================================================================>

    // call!(
    //     root,
    //     nft.storage_deposit(Some("lottery".to_string().parse().unwrap()),None),
    //     deposit=to_yocto("3")
    // ).assert_success();

    let res= call!(root,
                    lottery.claim_reward("Pakistan".to_string())
        );
    // println!("Promis result of claim reward :::::: {:#?}",res.promise_results());
    println!("Promis error of claim reward :::::: {:#?}",res.promise_errors());

    let res=view!(nft.nft_supply_for_owner(root.account_id())).unwrap_json_value();
    println!(
        "Nft supply after claim reward{}",res
    );

    // println!("Promis error of claim reward :::::: {:#?}",res.status());



    //GETTING EXECUTIONRESULT
    // println!("result:::{:#?}",res.promise_results());
    //<=================================================================>
    //              GETTING BALANCE OF LOTTERY ACCOUNT
    //              ` AFTER SELLING TICKETS FT TOKENS
    //<=================================================================>
    let check= view!(ftt.ft_balance_of(lottery.account_id())).unwrap_json_value();
    println!("balance of lottery after buy : {}",check);
    //<========================================================================================>
    //<========================================================================================>
    
    //STORAGE DEPOSIT
    // call!(
    //     demo,
    //     ftt.storage_deposit(Some("lottery".to_string().parse().unwrap()),None),
    //     deposit=to_yocto("3")
    // ).assert_success();
    //GETTING BALANCE OF DEMO ACCOUNT before airdrop
    //<=================================================================>
    //              AIRDROP FUNCTIONALITY CHECKING
    //<=================================================================>
    let check= view!(ftt.ft_balance_of(demo.account_id())).unwrap_json_value();
    println!("balance of DEMO befor airdrop : {}",check);
    //GETTING AIRDROP FOR LOTTERY PARTICIPATIOIN
    call!(
        demo,
        lottery.ft_airdrop()
    ).assert_success();
    //<=================================================================>
    //              GETTING BALANCE OF AFTER AIRDROP
    //<=================================================================>
    let check= view!(ftt.ft_balance_of(demo.account_id())).unwrap_json_value();
    println!("balance of DEMO after airdrop : {}",check);

// ==============================================================================================
//===============================================================================================
//===============================================================================================
//<=================================================================>
    //                      STARTING THE LOTTERY
    //<=================================================================>
    call!(
        alice,
        lottery.start_new_lottery(5,U128::from(1000000000000000000000000),"ft".to_string().parse().unwrap(),"nft".to_string().parse().unwrap())
    ).assert_success();
    //<=================================================================>
    //                  STORAGE DEPOSIT FOR LOTTERY CONTRACT
    //            IF NOT THEN WE CANNOT SEND TOKENS TO LOTTERY CONTRACT
    //<=================================================================>
    call!(
        demo,
        ftt.storage_deposit(Some("lottery".to_string().parse().unwrap()),None),
        deposit=to_yocto("3")
    ).assert_success();
    //
    //<=================================================================>
    //                  TRANSFERING FUNGIBLE TOKEN 
    //              TO LOTTERY CONTRACT FOR AIRDROPING
    //<=================================================================>
    let _amount= to_yocto("1000");
    call!(
        root,
        ftt.ft_transfer(lottery.account_id(),U128::from(_amount),Some("Testing Fungible Token".to_string())),
        deposit=1 
    ).assert_success();

    //CHECKING FT RECEIVED OR NOT
    let check= view!(ftt.ft_balance_of(alice.account_id())).unwrap_json_value();
    println!("balance of alice is : {}",check);

    //GETTING APPROVED FT
    let res=view!(lottery.get_approved_ft()).unwrap_json_value();
    println!("Approved Ft is  : {:#?}",res);
    //GETTING LOTTERY OWNER
    let res= view!(lottery.get_lottery_owner()).unwrap_json_value();
    println!("Lottery Owner: {}",res);

    let root_balance=view!(ftt.ft_balance_of(root.account_id())).unwrap_json_value();
    println!("Balance befor buying ticket : {}",root_balance);
    
    //checking balance of lottery account before buying
    //<=================================================================>
    //              GETTING BALANCE OF LOTTERY ACCOUNT
    //                     AFTER SENDING FT TOKENS
    //<=================================================================>
    let check= view!(ftt.ft_balance_of(lottery.account_id())).unwrap_json_value();
    println!("balance of lottery before buy : {}",check);
    //<========================================================================================>

    //<========================================================================================>
    // let res = call!(
    //     root,
    //     ftt.ft_transfer_call(lottery.account_id(),U128::from(1000000000000000000000000),None,"lottery buying hurah!".to_string()),
    //     deposit=1
    // );
    //<=================================================================>
    //                      BUYING LOTTERY TICKETS
    //<=================================================================>
    for i in 1..11{
        call!(
            root,
    
            ftt.ft_transfer_call(lottery.account_id(),U128::from(1000000000000000000000000),None,"lottery buying hurah!".to_string()),
            deposit=1
        );
    }

    //<=================================================================>
    //                      PICKING RANDOM WINNER
    //<=================================================================>
    let res=call!(
        alice,
        lottery.pick_winner()
    );

    // println!("Pick winner promise : {:#?}",res.promise_errors());

    // let root_balance=view!(ftt.ft_balance_of(root.account_id())).unwrap_json_value();
    // println!("Balance after buying ticket : {}",root_balance);

    //<=================================================================>
    //              GETTING ALL PLAYERS
    //<=================================================================>
    let res= view!(lottery.get_players()).unwrap_json_value();
    println!("player in lottery : {}",res);

    //<=================================================================>
    //                      WINNER IS
    //<=================================================================>
    let res = view!(lottery.get_winner_of_lottery());
    // let winner:String= res.unwrap_json();
    // println!("winnner of lottery is ; {}",winner);
    // let s= UserAccount::;

    //NFT CHECKING OF ROOT

    let res=view!(nft.nft_supply_for_owner(root.account_id())).unwrap_json_value();
    println!(
        "Nft supply before claim reward{}",res
    );
     //<=================================================================>
    //                       CLAIM REWARD
    //<=================================================================>

    // call!(
    //     root,
    //     nft.storage_deposit(Some("lottery".to_string().parse().unwrap()),None),
    //     deposit=to_yocto("3")
    // ).assert_success();

    let res= call!(root,
                    lottery.claim_reward("Pakistan".to_string())
                
                //    gas= near_sdk_sim::DEFAULT_GAS
        );
    // println!("Promis result of claim reward :::::: {:#?}",res.promise_results());
    println!("Promis error of claim reward :::::: {:#?}",res.promise_errors());

    let res=view!(nft.nft_supply_for_owner(root.account_id())).unwrap_json_value();
    println!(
        "Nft supply after claim reward{}",res
    );



}