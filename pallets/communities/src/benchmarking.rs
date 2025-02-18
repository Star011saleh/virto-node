//! Benchmarking setup for pallet-communities
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use self::{
	origin::DecisionMethod,
	types::{
		AccountIdOf, CommunityIdOf, DecisionMethodFor, MembershipIdOf, NativeBalanceOf, PalletsOriginOf, PollIndexOf,
		Vote,
	},
	Event, HoldReason, Pallet as Communities,
};
use fc_traits_memberships::{Inspect, Rank};
use frame_benchmarking::v2::*;
use frame_support::{
	traits::{
		fungible::{InspectFreeze, Mutate},
		OriginTrait,
	},
	BoundedVec,
};
use frame_system::{
	pallet_prelude::{BlockNumberFor, OriginFor, RuntimeCallFor},
	RawOrigin,
};
use sp_runtime::traits::StaticLookup;
use sp_std::{vec, vec::Vec};

fn assert_has_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

fn setup_account<T: Config>(name: &'static str, index: u32, seed: u32) -> Result<AccountIdOf<T>, BenchmarkError> {
	let who = frame_benchmarking::account(name, index, seed);

	let initial_balance: NativeBalanceOf<T> = 1_000_000_000_000_000u128
		.try_into()
		.map_err(|_| BenchmarkError::Stop("could not mint balance for a new account"))?;

	T::Balances::mint_into(&who, initial_balance)?;

	Ok(who)
}

fn setup_accounts<T: Config>() -> Result<Vec<AccountIdOf<T>>, BenchmarkError> {
	let size = T::BenchmarkHelper::community_desired_size();
	let mut accounts = vec![];

	for i in 0..size {
		let who = setup_account::<T>("community_benchmarking", i, 0)?;
		accounts.push(who);
	}

	Ok(accounts)
}

fn community_params<T: Config>(
	maybe_decision_method: Option<DecisionMethodFor<T>>,
) -> (
	CommunityIdOf<T>,
	DecisionMethodFor<T>,
	T::RuntimeOrigin,
	PalletsOriginOf<T>,
) {
	let community_id = T::BenchmarkHelper::community_id();

	let decision_method = maybe_decision_method.unwrap_or(DecisionMethod::Rank);
	let admin_origin: T::RuntimeOrigin = T::BenchmarkHelper::community_origin(decision_method.clone());
	let admin_origin_caller: PalletsOriginOf<T> = admin_origin.clone().into_caller();

	(community_id, decision_method, admin_origin, admin_origin_caller)
}

/// Creates a community, setting a [DecisionMethod], returning
/// its ID as well as the caller origin, and origin caller.
fn create_community<T: Config>(
	origin: OriginFor<T>,
	maybe_decision_method: Option<DecisionMethodFor<T>>,
) -> Result<(CommunityIdOf<T>, OriginFor<T>), BenchmarkError> {
	T::BenchmarkHelper::initialize_memberships_collection()?;
	let (community_id, decision_method, admin_origin, admin_origin_caller) =
		community_params::<T>(maybe_decision_method);

	Pallet::<T>::create(origin.clone(), admin_origin_caller, community_id)?;
	Pallet::<T>::set_decision_method(origin, community_id, decision_method)?;

	Ok((community_id, admin_origin))
}

/// Initializes the memberships of a community built for benchmarking
/// purposes.
///
/// Then, returns a list of tuples, each one containing a member's
/// [AccountId] and their corresponding
fn setup_members<T>(
	origin: OriginFor<T>,
	community_id: CommunityIdOf<T>,
) -> Result<Vec<(AccountIdOf<T>, MembershipIdOf<T>)>, BenchmarkError>
where
	T: Config,
	T::MembershipId: From<u32>,
{
	let members_with_memberships = setup_accounts::<T>()?
		.into_iter()
		.enumerate()
		.map(|(i, account_id)| (account_id, MembershipIdOf::<T>::from(i as u32)));

	for (who, membership_id) in members_with_memberships.clone() {
		T::BenchmarkHelper::issue_membership(community_id, membership_id)?;

		let who = T::Lookup::unlookup(who.clone());
		Pallet::<T>::add_member(origin.clone(), who.clone())?;
		Pallet::<T>::promote(origin.clone(), membership_id)?;
	}

	Ok(members_with_memberships.collect())
}

fn prepare_track_and_prepare_poll<T: Config>(
	track_origin: PalletsOriginOf<T>,
	submitter: AccountIdOf<T>,
) -> Result<PollIndexOf<T>, BenchmarkError>
where
	RuntimeCallFor<T>: From<crate::Call<T>>,
{
	T::BenchmarkHelper::prepare_track(track_origin.clone())?;

	let new_member = T::Lookup::unlookup(frame_benchmarking::account("community_benchmarking", 0, 0));
	T::BenchmarkHelper::prepare_poll(
		RawOrigin::Signed(submitter).into(),
		track_origin.clone(),
		crate::Call::<T>::add_member { who: new_member }.into(),
	)
}

#[benchmarks(
	where
		T: frame_system::Config + crate::Config,
		RuntimeCallFor<T>: From<crate::Call<T>>,
		MembershipIdOf<T>: From<u32>,
		BlockNumberFor<T>: From<u32>
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create() {
		// setup code
		let (id, _, _, origin) = community_params::<T>(None);

		#[extrinsic_call]
		_(RawOrigin::Root, origin.clone(), id);

		// verification code
		assert_has_event::<T>(Event::CommunityCreated { id, origin }.into());
	}

	#[benchmark]
	fn set_metadata(n: Linear<1, 64>, d: Linear<1, 256>, u: Linear<1, 256>) -> Result<(), BenchmarkError> {
		// setup code
		let (id, _, _, admin_origin) = community_params::<T>(None);
		Communities::<T>::create(RawOrigin::Root.into(), admin_origin, id)?;

		let name = Some(BoundedVec::truncate_from(vec![0u8; n as usize]));
		let description = Some(BoundedVec::truncate_from(vec![0u8; d as usize]));
		let url = Some(BoundedVec::truncate_from(vec![0u8; u as usize]));

		#[extrinsic_call]
		_(RawOrigin::Root, id, name.clone(), description.clone(), url.clone());

		// verification code
		assert_has_event::<T>(Event::MetadataSet { id, name }.into());

		Ok(())
	}

	#[benchmark]
	fn set_decision_method() -> Result<(), BenchmarkError> {
		// setup code
		let (id, _, _, admin_origin) = community_params::<T>(Default::default());
		Communities::<T>::create(RawOrigin::Root.into(), admin_origin, id)?;
		crate::pallet::CommunityDecisionMethod::<T>::set(id, DecisionMethod::Rank);

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			id,
			DecisionMethod::CommunityAsset(T::BenchmarkHelper::community_asset_id()),
		);

		// verification code
		assert_has_event::<T>(Event::DecisionMethodSet { id }.into());

		Ok(())
	}

	#[benchmark]
	fn add_member() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin) = create_community::<T>(RawOrigin::Root.into(), None)?;

		let who: AccountIdOf<T> = frame_benchmarking::account("community_benchmarking", 0, 0);
		let membership_id = MembershipIdOf::<T>::from(0);

		T::BenchmarkHelper::issue_membership(id, membership_id)?;

		#[extrinsic_call]
		_(origin.into_caller(), T::Lookup::unlookup(who.clone()));

		// verification code
		assert_has_event::<T>(
			Event::MemberAdded {
				who: who.clone(),
				membership_id,
			}
			.into(),
		);
		assert!(T::MemberMgmt::check_membership(&who, &membership_id).is_some());

		Ok(())
	}

	#[benchmark]
	fn remove_member() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin): (CommunityIdOf<T>, OriginFor<T>) = create_community::<T>(RawOrigin::Root.into(), None)?;

		let who: AccountIdOf<T> = frame_benchmarking::account("community_benchmarking", 0, 0);
		let membership_id = MembershipIdOf::<T>::from(0);

		T::BenchmarkHelper::issue_membership(id, membership_id)?;

		Communities::<T>::add_member(origin.clone(), T::Lookup::unlookup(who.clone()))?;

		#[extrinsic_call]
		_(origin.into_caller(), T::Lookup::unlookup(who.clone()), membership_id);

		// verification code
		assert_has_event::<T>(
			Event::MemberRemoved {
				who: who.clone(),
				membership_id,
			}
			.into(),
		);
		assert!(T::MemberMgmt::check_membership(&who, &membership_id).is_none());

		Ok(())
	}

	#[benchmark]
	fn promote() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin): (CommunityIdOf<T>, OriginFor<T>) = create_community::<T>(RawOrigin::Root.into(), None)?;

		let who: AccountIdOf<T> = frame_benchmarking::account("community_benchmarking", 0, 0);
		let membership_id = MembershipIdOf::<T>::from(0);

		T::BenchmarkHelper::issue_membership(id, membership_id)?;

		Communities::<T>::add_member(origin.clone(), T::Lookup::unlookup(who.clone()))?;

		#[extrinsic_call]
		_(origin.into_caller(), membership_id);

		// verification code
		let (_, m) = T::MemberMgmt::user_memberships(&who, Some(id))
			.next()
			.ok_or::<frame_support::pallet_prelude::DispatchError>(Error::<T>::NotAMember.into())?;
		let rank = T::MemberMgmt::rank_of(&id, &m).expect("has rank");

		assert_has_event::<T>(Event::MembershipRankUpdated { membership_id, rank }.into());

		assert_eq!(Communities::<T>::member_rank(&id, &membership_id), rank);

		Ok(())
	}

	#[benchmark]
	fn demote() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin): (CommunityIdOf<T>, OriginFor<T>) = create_community::<T>(RawOrigin::Root.into(), None)?;

		let who: AccountIdOf<T> = frame_benchmarking::account("community_benchmarking", 0, 0);
		let membership_id = MembershipIdOf::<T>::from(0);

		T::BenchmarkHelper::issue_membership(id, membership_id)?;

		Communities::<T>::add_member(origin.clone(), T::Lookup::unlookup(who.clone()))?;

		Communities::<T>::promote(origin.clone(), membership_id)?;

		#[extrinsic_call]
		_(origin.into_caller(), membership_id);

		// verification code
		assert_eq!(Communities::<T>::member_rank(&id, &membership_id), 0.into());

		Ok(())
	}

	#[benchmark]
	fn vote() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin) = create_community::<T>(RawOrigin::Root.into(), None)?;
		let members = setup_members::<T>(origin.clone(), id)?;

		let (who, membership_id) = members
			.first()
			.expect("desired size of community to be equal or greather than 1")
			.clone();

		prepare_track_and_prepare_poll::<T>(origin.into_caller(), who.clone())?;

		#[extrinsic_call]
		_(
			RawOrigin::Signed(who.clone()),
			membership_id,
			0u32,
			Vote::Standard(true),
		);

		// verification code
		assert_has_event::<T>(
			Event::VoteCasted {
				who: who.clone(),
				poll_index: 0u32,
				vote: Vote::Standard(true),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn remove_vote() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin) = create_community::<T>(RawOrigin::Root.into(), None)?;
		let members = setup_members::<T>(origin.clone(), id)?;

		let (who, membership_id) = members
			.first()
			.expect("desired size of community to be equal or greather than 1")
			.clone();

		prepare_track_and_prepare_poll::<T>(origin.into_caller(), who.clone())?;

		Communities::<T>::vote(
			RawOrigin::Signed(who.clone()).into(),
			membership_id,
			0u32,
			Vote::Standard(true),
		)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(who.clone()), membership_id, 0u32);

		// verification code
		assert_has_event::<T>(
			Event::VoteRemoved {
				who: who.clone(),
				poll_index: 0u32,
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn unlock() -> Result<(), BenchmarkError> {
		// setup code
		let (id, origin) = create_community::<T>(RawOrigin::Root.into(), Some(DecisionMethod::NativeToken))?;
		let members = setup_members::<T>(origin.clone(), id)?;

		let (who, membership_id) = members
			.first()
			.expect("desired size of community to be equal or greather than 1")
			.clone();

		let index = prepare_track_and_prepare_poll::<T>(origin.into_caller(), who.clone())?;

		Communities::<T>::vote(
			RawOrigin::Signed(who.clone()).into(),
			membership_id,
			0u32,
			Vote::NativeBalance(true, 1u32.into()),
		)?;

		assert_eq!(
			T::Balances::balance_frozen(&HoldReason::VoteCasted(0u32).into(), &who),
			1u32.into()
		);

		T::BenchmarkHelper::finish_poll(index)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(who.clone()), 0u32);

		// verification code
		assert_eq!(
			T::Balances::balance_frozen(&HoldReason::VoteCasted(0u32).into(), &who),
			0u32.into()
		);

		Ok(())
	}

	impl_benchmark_test_suite!(
		Communities,
		sp_io::TestExternalities::new(Default::default()),
		crate::mock::Test
	);
}
