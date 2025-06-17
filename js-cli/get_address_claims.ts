import {
    CLTypeBool,
    CLTypeKey,
    CLTypeList,
    CLTypeUInt32,
    CLTypeUInt512,
    CLTypeUInt64,
    CLValueList,
    CLValueParser,
    HttpHandler,
    RpcClient,
} from "casper-js-sdk";
import {Parser} from "@make-software/ces-js-parser";
import {getStateItem} from "./utils";


const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .requiredOption('--contract_hash [value]', 'staking contract address')
    .requiredOption('--account_hash [value]', 'account hash')

program.parse();

const options = program.opts();

const UNSTAKE_IDS_STATE_IDX = 4;
const UNSTAKE_INFO_STATE_IDX = 80;

export async function getUnstakeIdsDictionaryItemKey(
    rpcClient: RpcClient,
    contract_hash: string,
    accountHashHex: string
): Promise<Array<number>> {
    if (accountHashHex.length !== 64) {
        throw new Error('Account hash must be 32 bytes (64 hex characters)');
    }

    const bytes: number[] = [];
    bytes.push(0x00);

    const accountHashBytes = Uint8Array.from(Buffer.from(accountHashHex, 'hex'));
    bytes.push(...Array.from(accountHashBytes));

    const unstakeIdsBytes =  await getStateItem(rpcClient, contract_hash, UNSTAKE_IDS_STATE_IDX, new Uint8Array(bytes));
    const list = CLValueList.fromBytes(unstakeIdsBytes, new CLTypeList(CLTypeUInt32)).result;

    const unstakeIds = [];
    for (const el of list.elements) {
        const unstakeId = el.ui32.toNumber();
        unstakeIds.push(unstakeId);
    }

    return unstakeIds;
}

interface UnstakeInfoResult {
    id: any;
    owner: any;
    amount: any;
    claimTime: any;
    isClaimed: any;
}
export async function getUnstakeInfoDictionaryItemKey(
    rpcClient: RpcClient,
    contract_hash: string,
    unstakeId: number
): Promise<UnstakeInfoResult> {
    // Step 2: Construct the input byte array
    const bytes: number[] = [];

    // Push the unstake Id
    bytes.push(unstakeId & 0xff);
    bytes.push((unstakeId >> 8) & 0xff);
    bytes.push((unstakeId >> 16) & 0xff);
    bytes.push((unstakeId >> 24) & 0xff);

    const stateBytes =  await getStateItem(rpcClient, contract_hash, UNSTAKE_INFO_STATE_IDX, new Uint8Array(bytes));

    const getCLValue = (bytes: Uint8Array, type: any) => {
        const clValueWithRemainder = CLValueParser.fromBytesByType(
            bytes,
            type,
        );
        if (!clValueWithRemainder.bytes) {
            throw new Error('remainder is empty');
        }
        const remainder = clValueWithRemainder.bytes;
        return {
            value: clValueWithRemainder.result,
            remainder,
        };
    }

    const { value: id, remainder: stateBytes2 } = getCLValue(stateBytes, CLTypeUInt32);
    const { value: owner, remainder: stateBytes3 } = getCLValue(stateBytes2, CLTypeKey);
    const { value: amount, remainder: stateBytes4 } = getCLValue(stateBytes3, CLTypeUInt512);
    const { value: claimTime, remainder: stateBytes5 } = getCLValue(stateBytes4, CLTypeUInt64);
    const { value: isClaimed, remainder: stateBytes6 } = getCLValue(stateBytes5, CLTypeBool);

    return {
        id,
        owner,
        amount,
        claimTime,
        isClaimed,
    }
}

const get_address_claims = async () => {

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);

    const parser = await Parser.create(rpcClient, [
        options.contract_hash,
    ]);

    const unstakeIds = await getUnstakeIdsDictionaryItemKey(rpcClient, options.contract_hash, options.account_hash);

    for (const unstakeId of unstakeIds) {
        const unstakeInfoResult = await getUnstakeInfoDictionaryItemKey(rpcClient, options.contract_hash, unstakeId);

        console.log("unstake id", unstakeInfoResult.id.ui32.toNumber());
        console.log("unstake owner", unstakeInfoResult.owner.key.account.toHex());
        console.log("unstake amount", unstakeInfoResult.amount.ui512.toNumber());
        console.log("unstake claimTime", unstakeInfoResult.claimTime.ui64.toNumber());
        console.log("unstake isClaimed", unstakeInfoResult.isClaimed.bool.getValue());
    }
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
get_address_claims();
