use codec::Encode;
pub use crate::utils::AccountId32;
// Because Era is one of the args to our extrinsic params.
pub use super::extrinsic_params::Era;
use sp_runtime::traits::ConstU32;

#[derive(Eq, PartialEq, Default, Encode, Debug)]
pub struct ChargeSystemToken {
	// tip to be added for the block author
	#[codec(compact)]
	tip: u128,
	// Asset to pay the fee with
	system_token_id: Option<SystemTokenId>,
	// whom to vote for
	vote_candidate: Option<VoteAccountId>,
}

impl ChargeSystemToken {
	pub fn new(
		tip: u128, 
		system_token_id: Option<SystemTokenId>, 
		vote_candidate: Option<VoteAccountId>
	) -> Self {
		Self {
			tip,
			system_token_id,
			vote_candidate,
		}
	}
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

impl SystemTokenId {
	pub fn new(para_id: ParaId, pallet_id: PalletId, asset_id: AssetId) -> Self {
		Self {
			para_id,
			pallet_id,
			asset_id,
		}
	}
}

pub type StringLimit = ConstU32<128>;