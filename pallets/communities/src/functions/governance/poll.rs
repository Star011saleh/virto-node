use super::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_initiate_poll(community_id: &CommunityIdOf<T>) -> DispatchResult {
		Poll::<T>::insert(community_id, CommunityPoll::default());

		Ok(())
	}

	pub(crate) fn do_vote_in_poll(
		who: &AccountIdOf<T>,
		community_id: &CommunityIdOf<T>,
		vote: CommunityPollVote,
	) -> DispatchResult {
		Poll::<T>::try_mutate(community_id, |value| {
			let Some(mut poll) = value.clone() else {
        return Err(Error::<T>::PollAlreadyClosed)?;
      };

			match vote {
				CommunityPollVote::Aye(weight) => {
					poll.ayes = poll
						.ayes
						.saturating_add(Self::get_vote_weight(who, community_id, weight)?)
				}
				CommunityPollVote::Nay(weight) => {
					poll.ayes = poll
						.nays
						.saturating_add(Self::get_vote_weight(who, community_id, weight)?)
				}
			}

			*value = Some(poll);

			Ok(())
		})
	}

	#[allow(dead_code)]
	pub(crate) fn do_close_poll(community_id: &CommunityIdOf<T>) -> DispatchResult {
		let poll_outcome = Self::do_calculate_poll_outcome(community_id)?;
		let proposal = Self::do_deequeue_proposal(community_id)?;

		match poll_outcome {
			// Schedule the approved proposal
			PollOutcome::Approved => Self::do_execute_proposal(proposal),
			// Do nothing
			PollOutcome::Rejected => Ok(()),
		}?;

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	fn get_vote_weight(
		who: &AccountIdOf<T>,
		community_id: &CommunityIdOf<T>,
		_input_weight: VoteWeight,
	) -> Result<VoteWeight, DispatchError> {
		let governance_strategy = Self::governance_strategy(community_id).ok_or(Error::<T>::CommunityDoesNotExist)?;

		match governance_strategy {
			CommunityGovernanceStrategy::AdminBased(admin) => {
				if *who == admin {
					Ok(VoteWeight::one())
				} else {
					Ok(VoteWeight::zero())
				}
			}
			CommunityGovernanceStrategy::MemberCountPoll { .. } => Ok(VoteWeight::one()),
			CommunityGovernanceStrategy::AssetWeighedPoll { .. } => todo!(),
			CommunityGovernanceStrategy::RankedWeighedPoll { .. } => {
				// use crate::traits::rank::GetRank;

				// let membership = Self::ensure_member(community_id, who)?;
				// let rank = <MembershipPassportOf<T> as
				// GetRank<T::MembershipRank>>::rank_of(&membership);

				// Ok(input_weight.max(rank.into()))
				todo!()
			}
		}
	}

	fn do_calculate_poll_outcome(community_id: &CommunityIdOf<T>) -> Result<PollOutcome, DispatchError> {
		let governance_strategy = Self::governance_strategy(community_id).ok_or(Error::<T>::CommunityDoesNotExist)?;
		let poll = Self::poll(community_id).ok_or(Error::<T>::PollAlreadyClosed)?;

		let (ayes, nays) = (poll.ayes, poll.nays);

		match governance_strategy {
			CommunityGovernanceStrategy::AdminBased(_) => Ok(PollOutcome::Approved),
			CommunityGovernanceStrategy::MemberCountPoll { min } => {
				if ayes.saturating_add(nays) >= min {
					if ayes > nays {
						Ok(PollOutcome::Approved)
					} else {
						Ok(PollOutcome::Rejected)
					}
				} else {
					Err(Error::<T>::CannotClosePoll.into())
				}
			}
			CommunityGovernanceStrategy::AssetWeighedPoll {
				asset_id: _,
				min_approval,
			}
			| CommunityGovernanceStrategy::RankedWeighedPoll { min_approval } => {
				let approval = ayes / ayes.saturating_add(nays);
				if approval >= min_approval {
					Ok(PollOutcome::Approved)
				} else {
					Ok(PollOutcome::Rejected)
				}
			}
		}
	}
}