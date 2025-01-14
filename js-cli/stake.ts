import {
    Args,
    HttpHandler,
    KeyAlgorithm,
    PrivateKey,
    RpcClient,
    SessionBuilder,
    CLValueString,
    CLValueUInt512,
    CLValueByteArray,
    Hash,
    CLValueList,
    CLTypeUInt8,
    CLValueUInt8
} from "casper-js-sdk";
import * as fs from 'fs/promises';

const {program} = require('commander');

program
    .option('--wasm [value]', 'path to LS contract wasm file')
    .option('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--network_name [value]', 'network_name', 'casper-net-1')
    .option('--proxy_caller [value]', 'proxy caller wasm file')
    .option('--contract_package_hash [value]', 'staking contract address')
    .option('--amount [value]', 'amount to unstake')

program.parse();

const options = program.opts();

export const getSenderKey = async (filePath: string) => {
    const pem = await fs.readFile(filePath);
    return PrivateKey.fromPem(pem.toString(),
        KeyAlgorithm.ED25519
    );
}

const stake = async () => {

    const paymentAmount = 1_000_000_000;
    const owner = await getSenderKey(options.owner_keys_path);
    const contractWasm = await fs.readFile(options.proxy_caller);

    const args_bytes: Uint8Array = new Uint8Array([0x00, 0x00, 0x00, 0x00]);
    const serialized_args = CLValueList.newCLList(CLTypeUInt8,
        Array.from(args_bytes)
            .map(value => CLValueUInt8.newCLUint8(value))
    );


    const args = Args.fromMap({
        amount: CLValueUInt512.newCLUInt512(options.amount),
        attached_value: CLValueUInt512.newCLUInt512(options.amount),
        entry_point: CLValueString.newCLString("stake"),
        contract_package_hash: CLValueByteArray.newCLByteArray(Hash.fromHex(options.contract_package_hash).toBytes()),
        args: serialized_args,
    });

    const sessionTransaction = new SessionBuilder()
        .from(owner.publicKey)
        .runtimeArgs(args)
        .wasm(new Uint8Array(contractWasm))
        .payment(paymentAmount) // Amount in motes
        .chainName(options.network_name)
        .build();

    await sessionTransaction.sign(owner);

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);
    const result = await rpcClient.putTransaction(sessionTransaction);
    console.log("Transaction hash: ", result.transactionHash);
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
stake();
