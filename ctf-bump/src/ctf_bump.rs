#![no_std]

use dharitri_sc::imports::*;

#[dharitri_sc::contract]
pub trait CtfBump: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn bump(&self) {
        let caller = self.blockchain().get_caller();
        self.perform_bump(&caller);
    }
}
