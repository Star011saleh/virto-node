
//! Autogenerated weights for `pallet_payments`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `virto-builder`, CPU: `Intel(R) Xeon(R) Silver 4216 CPU @ 2.10GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("kreivo-local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/virto-node
// benchmark
// pallet
// --chain
// kreivo-local
// --pallet
// pallet_payments
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/kreivo/src/weights/pallet_payments.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_payments`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_payments::WeightInfo for WeightInfo<T> {
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(1182), added: 3657, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `Payments::PaymentParties` (r:0 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `q` is `[1, 50]`.
	fn pay(_q: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510`
		//  Estimated: `8517`
		// Minimum execution time: 158_440_000 picoseconds.
		Weight::from_parts(261_788_690, 0)
			.saturating_add(Weight::from_parts(0, 8517))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(1182), added: 3657, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1119`
		//  Estimated: `8856`
		// Minimum execution time: 389_717_000 picoseconds.
		Weight::from_parts(411_183_000, 0)
			.saturating_add(Weight::from_parts(0, 8856))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(1182), added: 3657, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1086`
		//  Estimated: `8517`
		// Minimum execution time: 287_452_000 picoseconds.
		Weight::from_parts(304_880_000, 0)
			.saturating_add(Weight::from_parts(0, 8517))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn request_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `159279`
		// Minimum execution time: 77_629_000 picoseconds.
		Weight::from_parts(83_710_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:1 w:1)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(1182), added: 3657, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn dispute_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1187`
		//  Estimated: `159279`
		// Minimum execution time: 143_872_000 picoseconds.
		Weight::from_parts(211_428_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(1182), added: 3657, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn resolve_dispute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1189`
		//  Estimated: `8856`
		// Minimum execution time: 555_143_000 picoseconds.
		Weight::from_parts(685_098_000, 0)
			.saturating_add(Weight::from_parts(0, 8856))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Payments::PaymentParties` (r:0 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn request_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `8517`
		// Minimum execution time: 63_928_000 picoseconds.
		Weight::from_parts(73_374_000, 0)
			.saturating_add(Weight::from_parts(0, 8517))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5052), added: 7527, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(223), added: 2698, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn accept_and_pay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `980`
		//  Estimated: `8856`
		// Minimum execution time: 229_573_000 picoseconds.
		Weight::from_parts(352_516_000, 0)
			.saturating_add(Weight::from_parts(0, 8856))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
