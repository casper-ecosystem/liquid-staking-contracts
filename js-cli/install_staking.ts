import {
    Args,
    CLValue,
    HttpHandler,
    KeyAlgorithm,
    PrivateKey,
    PublicKey,
    RpcClient,
    SessionBuilder
} from "casper-js-sdk";
import {getSenderKey} from "./utils";
import fs from "fs/promises";

const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--network_name [value]', 'network_name', 'casper-net-1')
    .requiredOption('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--wasm [value]', 'path to LS contract wasm file')
    .requiredOption('--validator [value]', 'validator public key')
    .option('--paymentAmount [value]', 'motes to cover gas costs', '600000000000');

program.parse();

const options = program.opts();

const install = async () => {

    const owner = await getSenderKey(options.owner_keys_path, options.keys_algo);
    const contractWasm = await fs.readFile(options.wasm);

    const args = Args.fromMap({
            odra_cfg_is_upgradable: CLValue.newCLValueBool(false),
            odra_cfg_allow_key_override: CLValue.newCLValueBool(true),
            odra_cfg_package_hash_key_name: CLValue.newCLString("StakedCSPR_package_hash"),
            validator_address: CLValue.newCLPublicKey(PublicKey.fromHex(options.validator)),
            claim_time: CLValue.newCLUint64(7*60*60),
        });

    const sessionTransaction = new SessionBuilder()
        .from(owner.publicKey)
        .installOrUpgrade()
        .runtimeArgs(args)
        .wasm(new Uint8Array(contractWasm))
        .payment(options.paymentAmount) // Amount in motes
        .chainName(options.network_name)
        .build();
    await sessionTransaction.sign(owner);

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);
    const result = await rpcClient.putTransaction(sessionTransaction);
    console.log("Transaction hash: ", result.transactionHash.toHex());
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
install();

