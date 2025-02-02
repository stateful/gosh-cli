/*
 * Copyright 2018-2021 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
 */

pub const PARTICIPANT_ABI: &str = r#"
{
	"ABI version": 2,
	"header": ["time", "expire"],
	"functions": [
		{
			"name": "onRoundComplete",
			"inputs": [
				{"name":"roundId","type":"uint64"},
				{"name":"reward","type":"uint64"},
				{"name":"ordinaryStake","type":"uint64"},
				{"name":"vestingStake","type":"uint64"},
				{"name":"lockStake","type":"uint64"},
				{"name":"reinvest","type":"bool"},
				{"name":"reason","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "receiveAnswer",
			"inputs": [
				{"name":"errcode","type":"uint32"},
				{"name":"comment","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "onTransfer",
			"inputs": [
				{"name":"source","type":"address"},
				{"name":"amount","type":"uint128"}
			],
			"outputs": [
			]
		},
		{
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		}
	],
	"data": [
	],
	"events": [
	]
}
"#;

pub const DEPOOL_ABI: &str = r#"
{
	"ABI version": 2,
	"header": ["time", "expire"],
	"functions": [
		{
			"name": "constructor",
			"inputs": [
				{"name":"minStake","type":"uint64"},
				{"name":"validatorAssurance","type":"uint64"},
				{"name":"proxyCode","type":"cell"},
				{"name":"validatorWallet","type":"address"},
				{"name":"participantRewardFraction","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "addOrdinaryStake",
			"inputs": [
				{"name":"stake","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "withdrawFromPoolingRound",
			"inputs": [
				{"name":"withdrawValue","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "addVestingStake",
			"inputs": [
				{"name":"stake","type":"uint64"},
				{"name":"beneficiary","type":"address"},
				{"name":"withdrawalPeriod","type":"uint32"},
				{"name":"totalPeriod","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "addLockStake",
			"inputs": [
				{"name":"stake","type":"uint64"},
				{"name":"beneficiary","type":"address"},
				{"name":"withdrawalPeriod","type":"uint32"},
				{"name":"totalPeriod","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "withdrawPart",
			"inputs": [
				{"name":"withdrawValue","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "withdrawAll",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "cancelWithdrawal",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "setVestingDonor",
			"inputs": [
				{"name":"donor","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "setLockDonor",
			"inputs": [
				{"name":"donor","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "transferStake",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"amount","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "participateInElections",
			"id": "0x4E73744B",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"validatorKey","type":"uint256"},
				{"name":"stakeAt","type":"uint32"},
				{"name":"maxFactor","type":"uint32"},
				{"name":"adnlAddr","type":"uint256"},
				{"name":"signature","type":"bytes"}
			],
			"outputs": [
			]
		},
		{
			"name": "ticktock",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "completeRoundWithChunk",
			"inputs": [
				{"name":"roundId","type":"uint64"},
				{"name":"chunkSize","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "completeRound",
			"inputs": [
				{"name":"roundId","type":"uint64"},
				{"name":"participantQty","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "onStakeAccept",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"comment","type":"uint32"},
				{"name":"elector","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "onStakeReject",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"comment","type":"uint32"},
				{"name":"elector","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "onSuccessToRecoverStake",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"elector","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "onFailToRecoverStake",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"elector","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "terminator",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "setValidatorRewardFraction",
			"inputs": [
				{"name":"fraction","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "receiveFunds",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "getLastRoundInfo",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "getParticipantInfo",
			"inputs": [
				{"name":"addr","type":"address"}
			],
			"outputs": [
				{"name":"total","type":"uint64"},
				{"name":"withdrawValue","type":"uint64"},
				{"name":"reinvest","type":"bool"},
				{"name":"reward","type":"uint64"},
				{"name":"stakes","type":"map(uint64,uint64)"},
				{"components":[{"name":"remainingAmount","type":"uint64"},{"name":"lastWithdrawalTime","type":"uint64"},{"name":"withdrawalPeriod","type":"uint32"},{"name":"withdrawalValue","type":"uint64"},{"name":"owner","type":"address"}],"name":"vestings","type":"map(uint64,tuple)"},
				{"components":[{"name":"remainingAmount","type":"uint64"},{"name":"lastWithdrawalTime","type":"uint64"},{"name":"withdrawalPeriod","type":"uint32"},{"name":"withdrawalValue","type":"uint64"},{"name":"owner","type":"address"}],"name":"locks","type":"map(uint64,tuple)"},
				{"name":"vestingDonor","type":"address"},
				{"name":"lockDonor","type":"address"}
			]
		},
		{
			"name": "getDePoolInfo",
			"inputs": [
			],
			"outputs": [
				{"name":"poolClosed","type":"bool"},
				{"name":"minStake","type":"uint64"},
				{"name":"validatorAssurance","type":"uint64"},
				{"name":"participantRewardFraction","type":"uint8"},
				{"name":"validatorRewardFraction","type":"uint8"},
				{"name":"balanceThreshold","type":"uint64"},
				{"name":"validatorWallet","type":"address"},
				{"name":"proxies","type":"address[]"},
				{"name":"stakeFee","type":"uint64"},
				{"name":"retOrReinvFee","type":"uint64"},
				{"name":"proxyFee","type":"uint64"}
			]
		},
		{
			"name": "getParticipants",
			"inputs": [
			],
			"outputs": [
				{"name":"participants","type":"address[]"}
			]
		},
		{
			"name": "getDePoolBalance",
			"inputs": [
			],
			"outputs": [
				{"name":"value0","type":"int256"}
			]
		},
		{
			"name": "getRounds",
			"inputs": [
			],
			"outputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"supposedElectedAt","type":"uint32"},{"name":"unfreeze","type":"uint32"},{"name":"stakeHeldFor","type":"uint32"},{"name":"vsetHashInElectionPhase","type":"uint256"},{"name":"step","type":"uint8"},{"name":"completionReason","type":"uint8"},{"name":"stake","type":"uint64"},{"name":"recoveredStake","type":"uint64"},{"name":"unused","type":"uint64"},{"name":"isValidatorStakeCompleted","type":"bool"},{"name":"participantReward","type":"uint64"},{"name":"participantQty","type":"uint32"},{"name":"validatorStake","type":"uint64"},{"name":"validatorRemainingStake","type":"uint64"},{"name":"handledStakesAndRewards","type":"uint64"}],"name":"rounds","type":"map(uint64,tuple)"}
			]
		}
	],
	"data": [
	],
	"events": [
		{
			"name": "DePoolClosed",
			"inputs": [
			],
			"outputs": [
			]
		},
		{
			"name": "RoundStakeIsAccepted",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"comment","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "RoundStakeIsRejected",
			"inputs": [
				{"name":"queryId","type":"uint64"},
				{"name":"comment","type":"uint32"}
			],
			"outputs": [
			]
		},
		{
			"name": "ProxyHasRejectedTheStake",
			"inputs": [
				{"name":"queryId","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "ProxyHasRejectedRecoverRequest",
			"inputs": [
				{"name":"roundId","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "RoundCompleted",
			"inputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"supposedElectedAt","type":"uint32"},{"name":"unfreeze","type":"uint32"},{"name":"stakeHeldFor","type":"uint32"},{"name":"vsetHashInElectionPhase","type":"uint256"},{"name":"step","type":"uint8"},{"name":"completionReason","type":"uint8"},{"name":"stake","type":"uint64"},{"name":"recoveredStake","type":"uint64"},{"name":"unused","type":"uint64"},{"name":"isValidatorStakeCompleted","type":"bool"},{"name":"participantReward","type":"uint64"},{"name":"participantQty","type":"uint32"},{"name":"validatorStake","type":"uint64"},{"name":"validatorRemainingStake","type":"uint64"},{"name":"handledStakesAndRewards","type":"uint64"}],"name":"round","type":"tuple"}
			],
			"outputs": [
			]
		},
		{
			"name": "StakeSigningRequested",
			"inputs": [
				{"name":"electionId","type":"uint32"},
				{"name":"proxy","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "TooLowDePoolBalance",
			"inputs": [
				{"name":"replenishment","type":"uint256"}
			],
			"outputs": [
			]
		},
		{
			"name": "RewardFractionsChanged",
			"inputs": [
				{"name":"validator","type":"uint8"},
				{"name":"participants","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "InternalError",
			"inputs": [
				{"name":"ec","type":"uint16"}
			],
			"outputs": [
			]
		}
	]
}
"#;
