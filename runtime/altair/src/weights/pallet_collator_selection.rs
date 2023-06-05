
//! Autogenerated weights for `pallet_collator_selection`
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
// --pallet=pallet_collator_selection
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_collator_selection.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: CollatorAllowlist Allowlist (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 100]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 41_501 nanoseconds.
		Weight::from_ref_time(43_594_397)
			// Standard Error: 7_792
			.saturating_add(Weight::from_ref_time(6_194_704).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 25_600 nanoseconds.
		Weight::from_ref_time(26_401_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 22_301 nanoseconds.
		Weight::from_ref_time(22_601_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: CollatorAllowlist Allowlist (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// The range of component `c` is `[1, 99]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 85_801 nanoseconds.
		Weight::from_ref_time(94_457_751)
			// Standard Error: 3_896
			.saturating_add(Weight::from_ref_time(290_979).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// The range of component `c` is `[6, 100]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 59_501 nanoseconds.
		Weight::from_ref_time(63_375_946)
			// Standard Error: 3_389
			.saturating_add(Weight::from_ref_time(232_088).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	fn note_author() -> Weight {
		// Minimum execution time: 59_301 nanoseconds.
		Weight::from_ref_time(60_101_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:100 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `c` is `[1, 100]`.
	fn new_session(_r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 36_101 nanoseconds.
		Weight::from_ref_time(36_401_000)
			// Standard Error: 466_215
			.saturating_add(Weight::from_ref_time(17_679_837).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}