import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";

describeSuite({
    id: "COMM0205",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const paraId2000 = 2000;
        const paraId2001 = 2001;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;
        const blocksPerSession = 10n;
        let collatorAssignmentAlias: any;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            collatorAssignmentAlias = runtimeName.includes("light")
                ? polkadotJs.query.tanssiCollatorAssignment
                : polkadotJs.query.collatorAssignment;
        });
        it({
            id: "E01",
            title: "Removing credits should make chains not get collators",
            test: async () => {
                const removeFreeCredits = polkadotJs.tx.utility.batch([
                    polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2000, 0n),
                    polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2001, 0n),
                    polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2000, 0n),
                    polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0n),
                ]);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(removeFreeCredits).signAsync(alice)]);
                // Check that after 2 sessions, chain is deregistered
                await jumpSessions(context, 2);

                await context.createBlock();
                // Should not have assigned collators
                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();

                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2001.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E02",
            title: "Buying credits only for collator-assignment is not enough",
            test: async () => {
                const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
                // Now, buy some credits for container chain 2000. we only buy ones session -1
                const purchasedCredits = costPerSession + existentialDeposit;
                // Check that after 2 sessions, container chain 2000 has not collators
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);
                await context.createBlock([await tx.signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has 0 collators and is not producing blocks
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E03",
            title: "Additionally buying credits only for block-credits makes it assigned",
            test: async () => {
                // Now, buy some credits for container chain 2000. we only buy ones session -1
                const purchasedCredits = blocksPerSession * costPerBlock * 2n;
                // Check that after 2 sessions, container chain 2000 has not collators
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);
                await context.createBlock([await tx.signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has collators
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())].length,
                    `Container chain ${paraId2000} has 0 collators`
                ).toBeGreaterThan(0);
            },
        });
        it({
            id: "E04",
            title: "Just one session later they should be unassinged",
            test: async () => {
                // Check that after 1 sessions
                await jumpSessions(context, 1);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
    },
});
