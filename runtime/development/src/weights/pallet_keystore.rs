//! Autogenerated weights for pallet_keystore
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION
//! 4.0.0-dev DATE: 2022-06-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH
//! RANGE: `[]` EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet-keystore
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/development/src/weights/pallet_keystore.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_keystore using the Substrate node and recommended
/// hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_keystore::weights::WeightInfo for WeightInfo<T> {
	fn add_keys(n: u32) -> Weight {
		Weight::from_ref_time(26_305_000) // Standard Error: 166_000
			.saturating_add(Weight::from_ref_time(33_062_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}

	fn revoke_keys(n: u32) -> Weight {
		Weight::from_ref_time(12_542_000) // Standard Error: 105_000
			.saturating_add(Weight::from_ref_time(18_740_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}

	fn set_deposit() -> Weight {
		Weight::from_ref_time(20_999_000).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
