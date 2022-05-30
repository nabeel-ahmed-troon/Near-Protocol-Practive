use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
// use ft::ContractContract as FungibleTokenContract;
use fungible_token::ContractContract as FungibleTokenContract;
use lottery_contract::LotteryContract as LotteryContract;
use non_fungible_token::ContractContract as NonFungibleTokenContract;
// use ft_staking::ContractContract;
// use ft_staking::{APY,FT};

use near_sdk_sim::{deploy, init_simulator, to_yocto, ContractAccount, UserAccount};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    // CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/ft.wasm",
    // STAKING_WASM_BYTES => "target/wasm32-unknown-unknown/release/ft_staking.wasm",

    LOTTERY_CONTRACT_WASM=>"target/wasm32-unknown-unknown/release/lottery_contract.wasm",
    FUNGIBLE_CONTRACT=>"target/wasm32-unknown-unknown/release/fungible_token.wasm",
    NON_FUNGIBLE_CONTRACT=>"target/wasm32-unknown-unknown/release/non_fungible_token.wasm",

    // CONTRACT_WASM_BYTES => "res/ft_bkrt_contract.wasm",
    // STAKING_WASM_BYTES => "res/staking_bkrt_contract.wasm",

}

const FT_ID: &str = "ft";
// const ST_ID: &str = "staking";
const NFT_ID: &str="nft";
const LOTTERY_ID: &str="lottery";



pub fn register_user(user: &near_sdk_sim::UserAccount) {
    user.call(
        FT_ID.parse().unwrap(),
        "storage_deposit",
        &json!({
            "account_id": user.account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}

// pub fn init(
//     initial_balance: u128,
// ) -> (
//     UserAccount,
//     ContractAccount<FungibleTokenContract>,
//     ContractAccount<ContractContract>,
//     UserAccount,
// ) {
//     let root = init_simulator(None);

//     let ft = deploy!(
//         contract: FungibleTokenContract,
//         contract_id: FT_ID,
//         bytes: &CONTRACT_WASM_BYTES,
//         signer_account: root,
//         init_method: new_default_meta(root.account_id(),initial_balance.into())
//     );

//     let alice = root.create_user("alice".parse().unwrap(), to_yocto("1000"));
//     register_user(&alice);
//     let apy_data: Vec<APY> = [
//         APY {
//             apy_key: "3months".to_string(),
//             interest_rate: 250,
//             min_duration: 6,
//             min_staking_amount: U128::from(500000000000000000000000000),
//         },
//         APY {
//             apy_key: "6months".to_string(),
//             interest_rate: 500,
//             min_duration: 12,
//             min_staking_amount: U128::from(5000000000000000000000000000),
//         },
//     ]
//     .to_vec();
//     let approved_fts: Vec<FT> = [FT {
//         account_id: ft.account_id(),
//         symbol: "BKRT".to_string(),
//         apy_against_duration: None,
//     }]
//     .to_vec();
//     let staking = deploy!(
//         contract: ContractContract,
//         contract_id: ST_ID,
//         bytes: &STAKING_WASM_BYTES,
//         signer_account: alice,
//         init_method: new(root.account_id(),approved_fts,apy_data)
//         // init_method: new(root.account_id(),[{ft.account_id(), "BKRT".to_string(),None}].to_vec(),apy_data)
//     );

//     (root, ft, staking, alice)
// }

pub fn init(
        initial_balance: u128,
    ) -> (
        UserAccount,
        ContractAccount<FungibleTokenContract>,
        ContractAccount<LotteryContract>,
        ContractAccount<NonFungibleTokenContract>,
        UserAccount,
        UserAccount
    ) {
        let root = init_simulator(None);
    
        
        let ft = deploy!(
            contract: FungibleTokenContract,
            contract_id: FT_ID,
            bytes: &FUNGIBLE_CONTRACT,
            signer_account: root,
            init_method: new_default_meta(root.account_id(),initial_balance.into())
        );

        println!("contract id {:?}",ft.account_id());
    
        let alice = root.create_user("alice".parse().unwrap(), to_yocto("1000"));
        register_user(&alice);
        let lottery=deploy!(
            contract:LotteryContract,
            contract_id: LOTTERY_ID,
            bytes: &LOTTERY_CONTRACT_WASM,
            signer_account:alice,
            init_method: new()
        );
        

        let nabeel = root.create_user("nabeel".parse().unwrap(), to_yocto("1000"));
        register_user(&nabeel);
        let non_fungible=deploy! (
            contract:NonFungibleTokenContract,
            contract_id: NFT_ID,
            bytes: &NON_FUNGIBLE_CONTRACT,
            signer_account:nabeel,
            init_method: new_default_meta(lottery.account_id())
        );

        (root,ft,lottery,non_fungible,alice,nabeel)

    }
     