use crate::*;

#[near_bindgen]
impl Lottery{
    #[private]
pub fn callback_promise_result(&mut self) -> bool {
    assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(_val) => true,
        PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
    }
}
}