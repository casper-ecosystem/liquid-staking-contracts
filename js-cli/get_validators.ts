import {
    CLTypeList,
    CLTypePublicKey,
    CLValueList,
    HttpHandler,
    RpcClient, URef,
} from "casper-js-sdk";
import {getContractMainPurse, getStakeInfo, getStateItem} from "./utils";
import {BigNumber} from "@ethersproject/bignumber";


const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .requiredOption('--contract_hash [value]', 'staking contract address')

program.parse();

const options = program.opts();

const VALIDATORSSTATE_IDX = 0x07;

export async function getValidatorsDictionaryItemKey(rpcClient: RpcClient, contract_hash: string): Promise<Uint8Array> {
    return getStateItem(rpcClient, contract_hash, VALIDATORSSTATE_IDX, null);
}

export async function getSCSPRTotalSupply(rpcClient: RpcClient, contract_hash: string): Promise<BigNumber> {
    const totalSupply = await rpcClient.queryLatestGlobalState("hash-" + contract_hash, ["total_supply"]);
    return totalSupply.storedValue.clValue.ui256.getValue();
}

const get_validators = async () => {

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);

    const contractMainPurse = await getContractMainPurse(rpcClient, options.contract_hash);
    console.log("Contract main purse:", contractMainPurse);

    const bytes = await getValidatorsDictionaryItemKey(rpcClient, options.contract_hash);
    const list = CLValueList.fromBytes(bytes, new CLTypeList(CLTypePublicKey)).result;

    let totalStaked: BigNumber = BigNumber.from(0);

    for (const el of list.elements) {
        const validatorPk = el.publicKey;

        console.log("Validator:",  validatorPk.toHex());

        const [bondingPurse, stakedAmount] = await getStakeInfo(rpcClient, validatorPk, URef.fromString(contractMainPurse));
        console.log("  Bonding purse:", "https://testnet.cspr.live/uref/" + bondingPurse);
        console.log("  Staked amount:", stakedAmount.toString());

        totalStaked = totalStaked.add(stakedAmount);
    }

    console.log("Total CSPR staked:", totalStaked.toString());

    const totalSupply = await getSCSPRTotalSupply(rpcClient, options.contract_hash);
    console.log("Total sCSPR supply:", totalSupply.toString());

    const totalStakedDecimal = parseFloat(totalStaked.toString());
    const totalSupplyDecimal = parseFloat(totalSupply.toString());

    const ratio = totalStakedDecimal / totalSupplyDecimal;
    console.log("CSPR/sCSPR rate:", ratio);

};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
get_validators();
