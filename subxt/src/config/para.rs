// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use super::{
    extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder},
    common::ChargeSystemToken,
	Config,	
};

pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
use crate::SubstrateConfig;
pub use primitive_types::{H256, U256};

/// Default set of commonly used types by InfraBlockspace nodes.
pub enum ParaConfig {}

impl Config for ParaConfig {
    type Index = <SubstrateConfig as Config>::Index;
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = ParaExtrinsicParams<Self>;
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type ParaExtrinsicParams<T> = BaseExtrinsicParams<T, ChargeSystemToken>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type ParaExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, ChargeSystemToken>;

pub struct AssetInfo<AssetId, AccountId, Balance> {
    asset_id: codec::Compact<AssetId>,
    owner: AccountId,
    is_sufficient: bool,
    min_balance: codec::Compact<Balance>,
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
    is_frozen: bool,
}

impl<AssetId, AccountId, Balance> AssetInfo<AssetId, AccountId, Balance> {

    pub fn new(
        asset_id: codec::Compact<AssetId>,
        owner: AccountId,
        is_sufficient: bool,
        min_balance: codec::Compact<Balance>,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
        is_frozen: bool,
    ) -> Self {
        Self {
            asset_id,
            owner,
            is_sufficient,
            min_balance,
            name,
            symbol,
            decimals,
            is_frozen
        }
    }
}

