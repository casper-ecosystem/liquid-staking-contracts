import {
    Args,
    CLValue,
    CLValueUInt64,
    HttpHandler,
    KeyAlgorithm,
    PrivateKey,
    PublicKey,
    RpcClient,
    CLValueBool,
    CLValueString, SessionBuilder
} from "casper-js-sdk";
import * as fs from 'fs/promises';

const {program} = require('commander');

program
    .option('--wasm [value]', 'path to LS contract wasm file')
    .option('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--network_name [value]', 'network_name', 'casper-net-1')
    .option('--validator [value]', 'validator public key')

program.parse();

const options = program.opts();

export const getSenderKey = async (filePath: string) => {
    const pem = await fs.readFile(filePath);
    return PrivateKey.fromPem(pem.toString(),
        KeyAlgorithm.ED25519
    );
}

const install = async () => {

    const paymentAmount = 300_000_000_000;
    const owner = await getSenderKey(options.owner_keys_path);
    const contractWasm = await fs.readFile(options.wasm);

    const args = Args.fromMap({
            odra_cfg_is_upgradable: CLValueBool.newCLValueBool(false),
            odra_cfg_allow_key_override: CLValueBool.newCLValueBool(true),
            odra_cfg_package_hash_key_name: CLValueString.newCLString("StakedCSPR_package_hash"),
            validator_address: CLValue.newCLPublicKey(PublicKey.fromHex(options.validator)),
            claim_time: CLValueUInt64.newCLUint64(7*60*60),
        });

    const sessionTransaction = new SessionBuilder()
        .from(owner.publicKey)
        .installOrUpgrade()
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
install();
