use crate::{mock::*, Error, Proposals};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_system::ensure_signed;
use pallet_dao_core::Error as DaoError;

#[test]
fn it_creates_a_proposal() {
	new_test_ext().execute_with(|| {
		let dao_id = b"TEST".to_vec();
		let dao_name = b"TEST DAO".to_vec();
		let prop_id = b"TEST_proposal".to_vec();
		let origin = RuntimeOrigin::signed(1);

		// cannot create a proposal without a DAO
		assert_noop!(
			DaoVotes::create_proposal(origin.clone(), dao_id.clone(), prop_id.clone()),
			DaoError::<Test>::DaoDoesNotExist
		);

		// preparation: create a DAO
		assert_ok!(DaoCore::create_dao(origin.clone(), dao_id.clone(), dao_name));

		// cannot create a proposal without DAO tokens existing (because they need to be reserved)
		assert_noop!(
			DaoVotes::create_proposal(origin.clone(), dao_id.clone(), prop_id.clone()),
			Error::<Test>::DaoTokenNotYetIssued
		);

		// preparation: issue token
		assert_ok!(DaoCore::issue_token(origin.clone(), dao_id.clone(), 1000));

		let dao = pallet_dao_core::Pallet::<Test>::load_dao(dao_id.clone()).unwrap();
		let asset_id = dao.asset_id.unwrap();

		assert_eq!(
			pallet_dao_assets::pallet::Pallet::<Test>::reserved(
				asset_id,
				ensure_signed(origin.clone()).unwrap()
			),
			Default::default()
		);

		// test creating a proposal
		assert_ok!(DaoVotes::create_proposal(origin.clone(), dao_id, prop_id.clone()));

		// check that a proposal exists with the given id
		let bounded_prop_id: BoundedVec<_, _> = prop_id.try_into().unwrap();
		assert!(<Proposals<Test>>::contains_key(bounded_prop_id));

		// creating a proposal should reserve DAO tokens
		assert_eq!(
			pallet_dao_assets::pallet::Pallet::<Test>::reserved(
				asset_id,
				ensure_signed(origin).unwrap()
			),
			1
		);
	});
}

#[test]
fn it_creates_a_vote() {
	new_test_ext().execute_with(|| {
		let dao_id = b"DAO1".to_vec();
		let dao_name = b"TEST DAO".to_vec();
		let prop_id = b"proposal1".to_vec();
		let origin = RuntimeOrigin::signed(1);

		// cannot create a vote without a proposal
		assert_noop!(
			DaoVotes::create_vote(origin.clone(), prop_id.clone(), true),
			Error::<Test>::ProposalDoesNotExist
		);

		// preparation: create a DAO
		assert_ok!(DaoCore::create_dao(origin.clone(), dao_id.clone(), dao_name));
		// preparation: issue token
		assert_ok!(DaoCore::issue_token(origin.clone(), dao_id.clone(), 1000));
		// preparation: create a proposal
		assert_ok!(DaoVotes::create_proposal(origin.clone(), dao_id, prop_id.clone()));

		// test creating a vote
		assert_ok!(DaoVotes::create_vote(origin, prop_id, true));
	});
}
