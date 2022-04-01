elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// @todo: not sure about removing this.
// use elrond_wasm::elrond_codec::TopEncode;

// Example of defining integers.
// const NFT_AMOUNT: u32 = 1;
// const ROYALTIES_MAX: u32 = 10_000;

// const rounds while whitelisting is open.
// contract datevalue that marks the opening of the whitesale.
//  updateable only be owner.

#[elrond_wasm::module]
pub trait WhitelistModule {
  
  // registered to whitelist
  #[view(getWhitelistStatus)]
  fn is_white_listed(&self, bech32address: ManagedAddress,) -> BigUint {
    // let caller = self.blockchain().get_caller();
    // let whitelist = self.whitelist(&caller).get();
    // if whitelist == BigUint::from(1u32) {
    //   OptionalValue::Some(whitelist) 
    // } else {
    //   OptionalValue::None 
    // }
    self.whitelist(&bech32address).get()

    //BigUint::from(1u32) + BigUint::from(1u32)
  }

  #[endpoint(setWhitelist)]
  fn set_whitelist(&self) {
      // let (payment, token) = self.call_value().payment_token_pair();
      let caller = self.blockchain().get_caller();
      let one = BigUint::from(1u32);
      self.whitelist(&caller).update(|whitelist| *whitelist += one);
  }

  #[endpoint(setWhitelistStart)]
  fn set_whitelist_start(&self) {
    self.whitelist_start().set(self.blockchain().get_block_timestamp() + 3600 * 24);
  }


  #[view(getWhitelist)]
  #[storage_mapper("whitelist")]
  fn whitelist(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;
  
  #[view(getWhitelistStart)]
  #[storage_mapper("whitelist_start")]
  fn whitelist_start(&self) -> SingleValueMapper<u64>;
}