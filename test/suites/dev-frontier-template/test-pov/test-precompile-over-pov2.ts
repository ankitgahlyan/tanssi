import "@tanssi/api-augment";

import { beforeAll, deployCreateCompiledContract, describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import { MAX_ETH_POV_PER_TX, createEthersTransaction } from "@moonwall/util";
import { type Abi, encodeFunctionData } from "viem";
import { type HeavyContract, deployHeavyContracts } from "../../../helpers";

const PRECOMPILE_BATCH_ADDRESS = "0x0000000000000000000000000000000000000801";

describeSuite({
    id: "DE1204",
    title: "PoV precompile test - PoV Limit (3.5Mb in Dev)",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let contracts: HeavyContract[];
        let batchAbi: Abi;
        let proxyAbi: Abi;
        let proxyAddress: `0x${string}`;
        let emptyBlockProofSize: bigint;

        beforeAll(async () => {
            // Create an empty block to estimate empty block proof size
            const { block } = await context.createBlock();
            // Empty blocks usually do not exceed 50kb
            emptyBlockProofSize = BigInt(block.proofSize || 50_000);

            const { contractAddress: contractAdd1, abi } = await deployCreateCompiledContract(context, "CallForwarder");
            proxyAddress = contractAdd1;
            proxyAbi = abi;
            contracts = await deployHeavyContracts(context, 6000, Number(6000n + MAX_ETH_POV_PER_TX / 24_000n + 1n));

            // Get the interface for Batch precompile
            batchAbi = fetchCompiledContract("Batch").abi;
        });

        it({
            id: "T01",
            title: "should allow to produce block under the PoV Limit with precompile tx",
            test: async () => {
                const maxContracts = MAX_ETH_POV_PER_TX / 24_000n - 1n;

                const callData = encodeFunctionData({
                    abi: batchAbi,
                    functionName: "batchAll",
                    args: [
                        [proxyAddress],
                        [],
                        [
                            encodeFunctionData({
                                abi: proxyAbi,
                                functionName: "callRange",
                                args: [contracts[0].account, contracts[Number(maxContracts)].account],
                            }),
                        ],
                        [],
                    ],
                });

                const rawSigned = await createEthersTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                    gasLimit: 52_000_000,
                });

                const { result, block } = await context.createBlock(rawSigned);
                expect(block.proofSize).to.be.at.least(Number(30_000));
                expect(block.proofSize).to.be.at.most(Number(50_000n + emptyBlockProofSize));
                expect(result?.successful).to.equal(true);
            },
        });

        it({
            id: "T0",
            title: "should prevent a tx reaching just over the PoV with a precompile tx",
            test: async () => {
                const maxContracts = MAX_ETH_POV_PER_TX / 24_000n;

                const callData = encodeFunctionData({
                    abi: batchAbi,
                    functionName: "batchAll",
                    args: [
                        [proxyAddress],
                        [],
                        [
                            encodeFunctionData({
                                abi: proxyAbi,
                                functionName: "callRange",
                                args: [contracts[0].account, contracts[Number(maxContracts)].account],
                            }),
                        ],
                        [],
                    ],
                });

                const rawSigned = await createEthersTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                    gasLimit: 60_000_000,
                });

                const { result, block } = await context.createBlock(rawSigned);

                // Empty blocks usually do not exceed 10kb, picking 50kb as a safe limit
                expect(block.proofSize).to.be.at.most(50_000);
                expect(result?.successful).to.equal(false);
            },
        });
    },
});
