import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { fetchIssuance, filterRewardFromContainer, filterRewardFromOrchestrator, getAuthorFromDigest } from "utils";
import { PARACHAIN_BOND } from "utils";

describeSuite({
    id: "COMMO0201",
    title: "Invulnerable reward test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let charlie: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            charlie = context.keyring.charlie;
        });
        it({
            id: "E01",
            title: "Every block created should reward the appropriate amount to orchestrator",
            test: async () => {
                await context.createBlock();
                const author = await getAuthorFromDigest(polkadotJs);
                // Fetch current session
                const currentSession = await polkadotJs.query.session.currentIndex();
                const keys = await polkadotJs.query.authorityMapping.authorityIdMapping(currentSession);
                const account = keys.toJSON()[author];
                // 70% is distributed across all rewards
                // But we have 2 container chains, so it should get 1/3 of this
                // Since it is an invulnerable, it receives all payment
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const expectedOrchestratorReward = chainRewards / 3n;
                const reward = await filterRewardFromOrchestrator(events, account);
                expect(reward).to.equal(expectedOrchestratorReward);
            },
        });

        it({
            id: "E02",
            title: "Parachain bond receives 30% of the inflation and pending rewards plus division dust",
            test: async () => {
                let expectedAmountParachainBond = 0n;

                const pendingChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.length);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const parachainBondBalanceBefore = (
                    await polkadotJs.query.system.account(PARACHAIN_BOND)
                ).data.free.toBigInt();
                await context.createBlock();

                const currentChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                let dust = 0n;
                if (currentChainRewards.isSome) {
                    const currentRewardPerChain = currentChainRewards.unwrap().rewardsPerChain.toBigInt();
                    dust = (issuance * 7n) / 10n - 3n * currentRewardPerChain;
                }
                const parachainBondBalanceAfter = (
                    await polkadotJs.query.system.account(PARACHAIN_BOND)
                ).data.free.toBigInt();
                expectedAmountParachainBond += (issuance * 3n) / 10n + dust;
                await context.createBlock();

                // Not sure where this one comes from, looks like a rounding thing
                expect(parachainBondBalanceAfter - parachainBondBalanceBefore).to.equal(
                    expectedAmountParachainBond + 1n
                );
            },
        });

        it({
            id: "E03",
            title: "Charlie receives the reward from container-chain block proposal",
            test: async () => {
                const balacharlieBalanceBeforenceBefore = (
                    await polkadotJs.query.system.account(charlie.address)
                ).data.free.toBigInt();

                await context.createBlock();

                const currentChainRewards = (await polkadotJs.query.inflationRewards.chainsToReward()).unwrap();
                const events = await polkadotJs.query.system.events();
                const receivedRewardCharlie = filterRewardFromContainer(events, charlie.address, 2000);

                const balacharlieBalanceBeforenceAfter = (
                    await polkadotJs.query.system.account(charlie.address)
                ).data.free.toBigInt();

                // Not sure where this one comes from, looks like a rounding thing
                expect(balacharlieBalanceBeforenceAfter - balacharlieBalanceBeforenceBefore).to.equal(
                    currentChainRewards.rewardsPerChain.toBigInt()
                );

                // Not sure where this one comes from, looks like a rounding thing
                expect(balacharlieBalanceBeforenceAfter - balacharlieBalanceBeforenceBefore).to.equal(
                    receivedRewardCharlie
                );
            },
        });
    },
});
