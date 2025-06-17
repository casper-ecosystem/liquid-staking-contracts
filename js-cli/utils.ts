import {KeyAlgorithm, PrivateKey, PublicKey, RpcClient, URef} from "casper-js-sdk";
import {BigNumber} from "@ethersproject/bignumber";
import {blake2b} from "@noble/hashes/blake2";
import fs from "fs/promises";

export const getSenderKey = async (filePath: string, algo: string) => {
    const pem = await fs.readFile(filePath);
    const keyAlgo = algo == 'ed25519' ? KeyAlgorithm.ED25519 : KeyAlgorithm.SECP256K1;
    return PrivateKey.fromPem(pem.toString(), keyAlgo);
}

let stateKey: string | undefined;

export async function getContractNamedKey(rpcClient: RpcClient, contractHash: string, namedKey: string): Promise<string> {
    const result = await rpcClient.queryLatestGlobalState("hash-" + contractHash, []);
    const _namedKey = result.storedValue.contract.namedKeys.find((k: any) => k.name === namedKey);
    if (!_namedKey) {
        throw new Error("state key not found");
    }
    return _namedKey.key.toString();
}

export async function getOdraStateNamedKey(rpcClient: RpcClient, contractHash: string): Promise<string> {
    return getContractNamedKey(rpcClient, contractHash, "state");
}

export async function getContractMainPurse(rpcClient: RpcClient, contractHash: string): Promise<string> {
    return getContractNamedKey(rpcClient, contractHash, "__contract_main_purse");
}

export async function getStateItem(rpcClient: RpcClient, contract_hash: string, index: number, itemBytes: Uint8Array|null) {
    if (!stateKey) {
        stateKey = await getOdraStateNamedKey(rpcClient, contract_hash);
    }

    const bytes: number[] = [];

    // Push u32 integer 4 in big-endian format
    bytes.push((index >> 24) & 0xff);
    bytes.push((index >> 16) & 0xff);
    bytes.push((index >> 8) & 0xff);
    bytes.push(index & 0xff);

    if (itemBytes) {
        bytes.push(...Array.from(itemBytes));
    }

    const hash = blake2b(new Uint8Array(bytes), {dkLen: 32});

    const dictionaryItemKey = Buffer.from(hash).toString('hex');

    const result = await rpcClient.getDictionaryItem(null, stateKey, dictionaryItemKey);

    return result.storedValue.clValue.bytes().slice(4);
}

export const getStakeInfo = async (
    rpcClient: RpcClient,
    validator: PublicKey,
    delegator: PublicKey | URef
) : Promise<([string, BigNumber])> => {

    const bidAddress = "bid-addr-03" +
        validator.accountHash().toHex() +
        (delegator instanceof PublicKey
            ? (delegator as PublicKey).accountHash().toHex()
            : (delegator as URef).toString().replace("-007", ""));
    const result = await rpcClient.queryLatestGlobalState(bidAddress, []);
    return [
        result.storedValue.bidKind.delegator.bondingPurse.toPrefixedString(),
        result.storedValue.bidKind.delegator.stakedAmount.getValue()
    ];
}
