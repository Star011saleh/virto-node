#![cfg_attr(not(feature = "std"), no_std)]
//! # Communities Pallet
//!
//! This pallet enables people to form dynamic collectives refered to as
//! communities. In simpler terms, it can be considered a DAO Factory.
//!
//! - [`Call`]
//! - [`Config`]
//!
//! ## Overview
//!
//! The Communities pallet provides functionality for managing communities,
//! facilitating its participants to have governance over the community entity
//! (and its associated account) which can interect with other systems:
//!
//! - Community registration and removal.
//! - Enrolling/removing members from a community.
//! - Promoting/demoting members within the community.
//! - Voting on proposals to enable community governance.
//!
//! ## Terminology
//!
//! - **Community:** An entity comprised of _members_ —each one defined by their
//!   [`AccountId`][1]— with a given _description_ who can vote on _proposals_
//!   and actively take decisions on behalf of it. Communities are given a
//!   _treasury account_ they can use to hold assets.
//! - **Community Description:** A set of metadata used to identify a community
//!   distinctively. Typically, a name, a description and a URL.
//! - **Community Status:** A community can be either `active` or `blocked`.
//! - **Member:** An [`AccountId`][1] registered into the community as such. Can
//!   have a rank within it and vote in the community's polls.
//! - **Member Rank:** Members could have a rank within the community. This can
//!   determine a voting weight depending on the community's voting mechanism.
//! - **Proposal:** A poll that executes a [call][2] dispatch if approved when
//!   it's closed.
//! - **Community Account:** A keyless [`AccountId`][1] generated on behalf of
//!   the community. Like any regular account can hold balances. It can transfer
//!   funds via a privileged call executed by the community _admin_ or a call
//!   dispatched from a proposal.
//! - **Decision Method:** Can be either rank weighed, member-counted, or
//!   asset-weighed and determines how the votes of proposals will be tallied.
//!
//! ## Lifecycle
//!
//! ```ignore
//! [       ] --> [Pending]               --> [Active]            --> [Blocked]
//! create        set_metadata                set_metadata            unblock
//!                                           block                   
//!                                           add_member              
//!                                           remove_member
//!                                           promote
//!                                           demote
//!                                           set_voting_mechanism
//! ```
//!
//! ## Implementations
//!
//! > TODO: Define which traits we are defining/implementing.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! - [`apply_for`][c00]: Registers an appliation as a new community, taking an
//!   [existential deposit][3] used to create the community account.
//!
//! ### Permissioned Functions
//!
//! Calling these functions requires being a member of the community.
//!
//! - [`add_member`][c02]: Enroll an account as a community member. In theory,
//!   any community member should be able to add a member. However, this can be
//!   changed to ensure it is a privileged function.
//! - `vote`: Adds a vote into a community proposal.
//!
//! ### Privileged Functions
//!
//! These functions can be called either by the community _admin_ or
//! dispatched through an approved proposal. !
//! - [`set_metadata`][c01]: Sets some [`CommunityMetadata`][t01] to describe
//!   the
//! community.
//! - [`remove_member`][c03]: Removes an account as a community member. While
//!   enrolling a member into the community can be an action taken by any
//!   member, the decision to remove a member should not be taken arbitrarily by
//!   any community member. Also, it shouldn't be possible to arbitrarily remove
//!   the community admin, as some privileged calls would be impossible execute
//!   thereafter.
//! - `promote`: Increases the rank of a member in the community.
//! - `demote`: Decreases the rank of a member in the community.
//! - `set_decision_method`: Means for a community to make decisions.
//!
//! ### Public Functions
//!
//! - [`community`][g00]: Stores the basic information of the community. If a
//!   value exists for a specified [`ComumunityId`][t00], this means a community
//!   exists.
//! - [`metadata`][g01]: Stores the metadata regarding a community.
//!
//! <!-- References -->
//! [1]: `frame_system::Config::AccountId`
//! [2]: https://docs.substrate.io/reference/glossary/#call
//! [3]: https://docs.substrate.io/reference/glossary/#existential-deposit
//!
//! [t00]: `Config::CommunityId`
//! [t01]: `types::CommunityMetadata`
//!
//! [c00]: `crate::Pallet::create`
//! [c01]: `crate::Pallet::set_metadata`
//! [c02]: `crate::Pallet::add_member`
//! [c03]: `crate::Pallet::remove_member`
//!
//! [g00]: `crate::Pallet::community`
//! [g01]: `crate::Pallet::metadata`
//! [g02]: `crate::Pallet::membership`
//! [g03]: `crate::Pallet::members_count`
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(feature = "runtime-benchmarks")]
pub use types::BenchmarkHelper;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod functions;
mod impls;

pub mod types;
pub mod weights;
pub use weights::*;
pub mod origin;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::num::NonZeroU8;
	use fc_traits_memberships::{self as membership, Inspect, Manager, Rank};
	use frame_support::{
		pallet_prelude::*,
		traits::{fungible, fungibles, EnsureOrigin, Polling},
		Blake2_128Concat, Parameter,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use sp_runtime::traits::StaticLookup;
	use types::{PollIndexOf, *};
	const ONE: NonZeroU8 = NonZeroU8::MIN;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// This type represents an unique ID for the community
		type CommunityId: Parameter + MaxEncodedLen + Copy;

		/// This type represents an unique ID to identify a membership within a
		/// community
		type MembershipId: Parameter + MaxEncodedLen + Copy;

		/// Means to manage memberships of a community
		type MemberMgmt: membership::Inspect<Self::AccountId, Group = CommunityIdOf<Self>, Membership = MembershipIdOf<Self>>
			+ membership::Manager<Self::AccountId, Group = CommunityIdOf<Self>, Membership = MembershipIdOf<Self>>
			+ membership::Rank<Self::AccountId, Group = CommunityIdOf<Self>, Membership = MembershipIdOf<Self>>;

		/// Origin authorized to manage the state of a community
		type CommunityMgmtOrigin: EnsureOrigin<OriginFor<Self>>;

		/// Origin authorized to manage memeberships of an active community
		type MemberMgmtOrigin: EnsureOrigin<OriginFor<Self>, Success = Self::CommunityId>;

		type Polls: Polling<
			Tally<Self>,
			Class = CommunityIdOf<Self>,
			Index = u32,
			Votes = VoteWeight,
			Moment = BlockNumberFor<Self>,
		>;

		/// Type represents interactions between fungibles (i.e. assets)
		type Assets: fungibles::Inspect<Self::AccountId>
			+ fungibles::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>;

		/// Type represents interactions between fungible tokens (native token)
		type Balances: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::freeze::Inspect<Self::AccountId, Id = Self::RuntimeHoldReason>
			+ fungible::freeze::Mutate<Self::AccountId, Id = Self::RuntimeHoldReason>;

		/// The overarching hold reason.
		type RuntimeHoldReason: From<HoldReason>;

		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// The pallet id used for deriving sovereign account IDs.
		#[pallet::constant]
		type PalletId: Get<frame_support::PalletId>;

		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper: BenchmarkHelper<Self>;
	}

	/// The origin of the pallet
	#[pallet::origin]
	pub type Origin<T> = origin::RawOrigin<T>;

	/// A reason for the pallet communities placing a hold on funds.
	#[pallet::composite_enum]
	pub enum HoldReason {
		// A vote has been casted on a poll
		VoteCasted(u32),
	}

	/// Stores the basic information of the community. If a value exists for a
	/// specified [`ComumunityId`][`Config::CommunityId`], this means a
	/// community exists.
	#[pallet::storage]
	#[pallet::getter(fn community)]
	pub(super) type Info<T> = StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, CommunityInfo>;

	/// List of origins and how they map to communities
	#[pallet::storage]
	pub(super) type CommunityIdFor<T> = StorageMap<_, Blake2_128Concat, PalletsOriginOf<T>, CommunityIdOf<T>>;

	/// Stores the metadata regarding a community.
	#[pallet::storage]
	#[pallet::getter(fn metadata)]
	pub(super) type Metadata<T: Config> =
		StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, CommunityMetadata, ValueQuery>;

	/// Stores the decision method for a community
	#[pallet::storage]
	pub(super) type CommunityDecisionMethod<T> =
		StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, DecisionMethodFor<T>, ValueQuery>;

	/// Stores the list of votes for a community.
	#[pallet::storage]
	#[pallet::getter(fn community_vote_of)]
	pub(super) type CommunityVotes<T> =
		StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Blake2_128Concat, PollIndexOf<T>, VoteOf<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A [`Commmunity`][`types::Community`] has been created.
		CommunityCreated {
			id: T::CommunityId,
			origin: PalletsOriginOf<T>,
		},
		/// Some [`CommmuniMetadata`][`types::CommunityMetadata`] has been set
		/// for a community.
		MetadataSet {
			id: T::CommunityId,
			name: Option<ConstSizedField<64>>,
		},
		DecisionMethodSet {
			id: T::CommunityId,
		},
		MemberAdded {
			who: AccountIdOf<T>,
			membership_id: MembershipIdOf<T>,
		},
		MemberRemoved {
			who: AccountIdOf<T>,
			membership_id: MembershipIdOf<T>,
		},
		MembershipRankUpdated {
			membership_id: MembershipIdOf<T>,
			rank: membership::GenericRank,
		},
		VoteCasted {
			who: AccountIdOf<T>,
			poll_index: PollIndexOf<T>,
			vote: VoteOf<T>,
		},
		VoteRemoved {
			who: AccountIdOf<T>,
			poll_index: PollIndexOf<T>,
		},
	}

	// Errors inform users that something worked or went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The community doesn't exist in storage, nor they have members.
		CommunityDoesNotExist,
		/// A community with the same [`CommunityId`][`Config::CommunityId`]
		/// already exists, therefore cannot be applied for.
		CommunityAlreadyExists,
		/// The community can't introduce new members at the moment
		CommunityAtCapacity,
		/// The specified [`AccountId`][`frame_system::Config::AccountId`] is
		/// not a member of the community
		NotAMember,
		/// The indicated index corresponds to a poll that is already ongoing
		AlreadyOngoing,
		/// The indicated index corresponds to a poll that is not ongoing
		NotOngoing,
		/// The track for the poll voted for does not correspond to the
		/// community ID
		InvalidTrack,
		/// The vote type does not correspond with the community's selected
		/// [`DecisionMethod`][`origin::DecisionMethod`]
		InvalidVoteType,
		/// The signer tried to remove a vote from a poll they haven't
		/// casted a vote yet, or they have already removed it from.
		NoVoteCasted,
		/// The poll
		NoLocksInPlace,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke
	// state changes. These functions materialize as "extrinsics", which are often
	// compared to transactions. Dispatchable functions must be annotated with a
	// weight and must return a DispatchResult.
	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		/// Creates a new community managed by the given origin
		#[pallet::call_index(0)]
		pub fn create(
			origin: OriginFor<T>,
			admin_origin: PalletsOriginOf<T>,
			community_id: T::CommunityId,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;

			Self::do_register_community(&admin_origin, &community_id)?;
			Self::deposit_event(Event::CommunityCreated {
				id: community_id,
				origin: admin_origin,
			});
			Ok(())
		}

		/// Sets some [`CommunityMetadata`][11] to describe the
		/// community.
		///
		/// [11]: `types::CommunityMetadata`
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::set_metadata(
				name.as_ref().map(|x| x.len() as u32).unwrap_or(0),
				description.as_ref().map(|x| x.len() as u32).unwrap_or(0),
				url.as_ref().map(|x| x.len() as u32).unwrap_or(0),
		))]
		pub fn set_metadata(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: Option<ConstSizedField<64>>,
			description: Option<ConstSizedField<256>>,
			url: Option<ConstSizedField<256>>,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;

			Self::do_set_metadata(&community_id, &name, description, url);
			Self::deposit_event(Event::MetadataSet { id: community_id, name });

			Ok(())
		}

		// === Memberships management ===

		/// Enroll an account as a community member that receives a membership
		/// from the available pool of memberships of the community.
		#[pallet::call_index(3)]
		pub fn add_member(origin: OriginFor<T>, who: AccountIdLookupOf<T>) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;

			let account = Self::community_account(&community_id);
			// assume the community has memberships to give out to the new member
			let (_, membership_id) = T::MemberMgmt::user_memberships(&account, None)
				.next()
				.ok_or(Error::<T>::CommunityAtCapacity)?;

			T::MemberMgmt::assign(&community_id, &membership_id, &who)?;

			Self::deposit_event(Event::MemberAdded { who, membership_id });
			Ok(())
		}

		/// Removes an account as a community member. While
		/// enrolling a member into the community can be an action taken by any
		/// member, the decision to remove a member should not be taken
		/// arbitrarily by any community member. Also, it shouldn't be possible
		/// to arbitrarily remove the community admin, as some privileged calls
		/// would be impossible to execute thereafter.
		#[pallet::call_index(4)]
		pub fn remove_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			membership_id: MembershipIdOf<T>,
		) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;

			ensure!(T::MemberMgmt::is_member_of(&community_id, &who), Error::<T>::NotAMember);

			T::MemberMgmt::release(&community_id, &membership_id)?;

			Self::deposit_event(Event::MemberRemoved { who, membership_id });
			Ok(())
		}

		/// Increases the rank of a member in the community
		#[pallet::call_index(5)]
		pub fn promote(origin: OriginFor<T>, membership_id: MembershipIdOf<T>) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;

			let rank = T::MemberMgmt::rank_of(&community_id, &membership_id)
				.ok_or(Error::<T>::NotAMember)?
				.promote_by(ONE);
			T::MemberMgmt::set_rank(&community_id, &membership_id, rank)?;

			Self::deposit_event(Event::MembershipRankUpdated { membership_id, rank });
			Ok(())
		}

		/// Decreases the rank of a member in the community
		#[pallet::call_index(6)]
		pub fn demote(origin: OriginFor<T>, membership_id: MembershipIdOf<T>) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;

			let rank = T::MemberMgmt::rank_of(&community_id, &membership_id).ok_or(Error::<T>::NotAMember)?;
			T::MemberMgmt::set_rank(&community_id, &membership_id, rank.demote_by(ONE))?;

			Self::deposit_event(Event::MembershipRankUpdated { membership_id, rank });
			Ok(())
		}

		// === Governance ===

		/// Decide the method used by the community to vote on proposals
		#[pallet::call_index(7)]
		pub fn set_decision_method(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			decision_method: DecisionMethodFor<T>,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;
			CommunityDecisionMethod::<T>::set(community_id, decision_method);

			Self::deposit_event(Event::DecisionMethodSet { id: community_id });
			Ok(())
		}

		/// Cast a vote on an on-going referendum
		#[pallet::call_index(8)]
		pub fn vote(
			origin: OriginFor<T>,
			membership_id: MembershipIdOf<T>,
			#[pallet::compact] poll_index: PollIndexOf<T>,
			vote: VoteOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_vote(&who, membership_id, poll_index, &vote)?;
			Self::deposit_event(Event::<T>::VoteCasted {
				who: who.clone(),
				poll_index,
				vote,
			});
			Ok(())
		}

		/// Remove any previous vote on a given referendum
		#[pallet::call_index(9)]
		pub fn remove_vote(
			origin: OriginFor<T>,
			membership_id: MembershipIdOf<T>,
			#[pallet::compact] poll_index: PollIndexOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_remove_vote(&who, membership_id, poll_index)?;
			Self::deposit_event(Event::<T>::VoteRemoved {
				who: who.clone(),
				poll_index,
			});
			Ok(())
		}

		/// Make previously held or locked funds from a vote available
		// if the refereundum  has finished
		#[pallet::call_index(10)]
		pub fn unlock(origin: OriginFor<T>, #[pallet::compact] poll_index: PollIndexOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(T::Polls::as_ongoing(poll_index).is_none(), Error::<T>::AlreadyOngoing);
			let vote = Self::community_vote_of(&who, poll_index).ok_or(Error::<T>::NoLocksInPlace)?;

			Self::do_unlock_for_vote(&who, &poll_index, &vote)
		}
	}
}
