
//! Autogenerated weights for `pallet_migration_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_migration_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_migration_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_migration_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_migration_manager::WeightInfo for WeightInfo<T> {
	// Storage: Migration Status (r:1 w:1)
	fn finalize() -> Weight {
		// Minimum execution time: 29_800 nanoseconds.
		Weight::from_ref_time(30_500_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Migration Status (r:1 w:1)
	// Storage: System Account (r:0 w:1)
	/// The range of component `n` is `[1, 100]`.
	fn migrate_system_account(n: u32, ) -> Weight {
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_ref_time(31_809_116)
			// Standard Error: 11_304
			.saturating_add(Weight::from_ref_time(1_293_803).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	// Storage: Migration Status (r:1 w:1)
	fn migrate_balances_issuance() -> Weight {
		// Minimum execution time: 35_801 nanoseconds.
		Weight::from_ref_time(47_501_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Migration Status (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[1, 10]`.
	fn migrate_vesting_vesting(n: u32, ) -> Weight {
		// Minimum execution time: 121_802 nanoseconds.
		Weight::from_ref_time(98_158_299)
			// Standard Error: 99_624
			.saturating_add(Weight::from_ref_time(38_488_345).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
	}
	// Storage: Migration Status (r:1 w:1)
	// Storage: Proxy Proxies (r:0 w:1)
	/// The range of component `n` is `[1, 10]`.
	fn migrate_proxy_proxies(n: u32, ) -> Weight {
		// Minimum execution time: 71_001 nanoseconds.
		Weight::from_ref_time(71_833_963)
			// Standard Error: 67_845
			.saturating_add(Weight::from_ref_time(9_368_247).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
}
