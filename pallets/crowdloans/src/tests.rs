use super::*;
use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn create_new_vault_should_work() {
    new_test_ext().execute_with(|| {
        let crowdloan = 1337;
        let project_shares = 420;
        let currency_shares = 113;
        let token = 1;

        let contribution_strategy =
            ContributionStrategy::Placeholder(crowdloan, currency_shares, crowdloan as u128);

        assert_ok!(Crowdloan::create_vault(
            frame_system::RawOrigin::Root.into(), // origin
            token,                                // token
            crowdloan,                            // crowdloan
            project_shares,                       // project_shares
            currency_shares,                      // currency_shares
            contribution_strategy,                // contribution_strategy
        ));

        if let Some(just_created_vault) = Crowdloan::vaults(crowdloan) {
            assert_eq!(
                just_created_vault,
                Vault {
                    project_shares: project_shares,
                    currency_shares: currency_shares,
                    currency: currency_shares,
                    phase: VaultPhase::CollectingContributions,
                    contribution_strategy: ContributionStrategy::Placeholder(
                        crowdloan,
                        currency_shares,
                        0,
                    ),
                    contributed: 0,
                }
            );
        }
    });
}