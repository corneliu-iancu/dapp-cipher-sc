#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod swap_module;

mod nft_module;

mod whitelist_module;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

const DEFAULT_TOKEN_DECIMALS_VALUE: u64 = 1_000_000_000_000_000_000; // 1 EGLD.

const DEFAULT_ESDT_RATIO_VALUE: u64 = 5_000_000; // this means 50.000 coins.

const DEFAULT_EGLD_MINT_COST_VALUE: u64 = DEFAULT_TOKEN_DECIMALS_VALUE / 100; // 0.01EGLD ~ $2.2

#[elrond_wasm::contract]
pub trait EsdtNftContract: swap_module::EgldEsdtSwap + nft_module::NftModule + whitelist_module::WhitelistModule {
    #[init]
    fn init(&self) {}

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    // #[only_owner]
    #[payable("EGLD")]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        #[var_args] opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_used_as_payment_nonce: OptionalValue<u64>,
    ) {
        let tx_cost = self.get_minting_price();
        
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        require!(payment_token.is_egld(), "Only EGLD accepted");
        // that means a 0.01 EGLD 
        require!(payment_amount == tx_cost, "Payment must be equal to 0.01 EGLD");

        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => token,
            OptionalValue::None => TokenIdentifier::egld(),
        };
        require!(
            token_used_as_payment.is_egld() || token_used_as_payment.is_valid_esdt_identifier(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );

        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();

        self.send()
            .esdt_local_mint(&wrapped_egld_token_id, 0, &(&payment_amount * self.get_mint_ratio()));
        
        let caller = self.blockchain().get_caller();
        
        self.send()
            .direct(
                &caller,
                &wrapped_egld_token_id,
                0,
                &(&payment_amount * self.get_mint_ratio()), // payment amount.
                &[]
            )
        ;
    }

    // The marketplace SC will send the funds directly to the initial caller, i.e. the owner
    // The caller has to know which tokens they have to claim,
    // by giving the correct token ID and token nonce
    #[only_owner]
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress,
        token_id: TokenIdentifier,
        token_nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace_address)
            .claim_tokens(token_id, token_nonce, caller)
            .async_call()
            .call_and_exit()
    }

    #[proxy]
    fn marketplace_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;

    #[view(getMintingPrice)]
    fn get_minting_price(&self) -> u64 {
        DEFAULT_EGLD_MINT_COST_VALUE
    }

    #[view(getMintingRatio)]
    fn get_mint_ratio(&self) -> u64 {
        DEFAULT_ESDT_RATIO_VALUE // Ratio of EGLD to ESDT Token ( which will affect price of the esdt. )
    }
}

mod nft_marketplace_proxy {
  elrond_wasm::imports!();

  #[elrond_wasm::proxy]
  pub trait NftMarketplace {
      #[endpoint(claimTokens)]
      fn claim_tokens(
          &self,
          token_id: TokenIdentifier,
          token_nonce: u64,
          claim_destination: ManagedAddress,
      );
  }
}