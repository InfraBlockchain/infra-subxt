// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use super::{
    extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder},
    Config,
};
use codec::Encode;

pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
use crate::SubstrateConfig;
pub use primitive_types::{H256, U256};

/// Default set of commonly used types by InfraBlockspace nodes.
pub enum IbsConfig {}

impl Config for IbsConfig {
    type Index = <SubstrateConfig as Config>::Index;
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = IbsExtrinsicParams<Self>;
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type IbsExtrinsicParams<T> = BaseExtrinsicParams<T, FeePaymentMetadata>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type IbsExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, FeePaymentMetadata>;

// Because Era is one of the args to our extrinsic params.
pub use super::extrinsic_params::Era;

#[derive(Eq, PartialEq, Default, Encode, Debug)]
pub struct FeePaymentMetadata {
	// tip to be added for the block author
	#[codec(compact)]
	tip: u128,
	// Asset to pay the fee with
	system_token_id: Option<SystemTokenId>,
	// whom to vote for
	vote_candidate: Option<VoteAccountId>,
}

pub type ParaId = u32;
pub type PalletId = u32;
pub type AssetId = u32;
pub type VoteAccountId = AccountId32;

#[derive(Eq, PartialEq, Encode, Debug)]
pub struct SystemTokenId {
	#[codec(compact)]
	pub para_id: ParaId,
	#[codec(compact)]
	pub pallet_id: PalletId,
	#[codec(compact)]
	pub asset_id: AssetId,
}