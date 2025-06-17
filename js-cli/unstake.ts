import {
    Args,
    HttpHandler,
    KeyAlgorithm,
    PrivateKey,
    RpcClient,
    ContractCallBuilder,
    CLValue
} from "casper-js-sdk";
import * as fs from 'fs/promises';
import {BigNumber} from "@ethersproject/bignumber";
import {getSenderKey} from "./utils";

const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--network_name [value]', 'network_name', 'casper-net-1')
    .requiredOption('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .requiredOption('--contract_package_hash [value]', 'staking contract package address')
    .requiredOption('--amount [value]', 'amount to unstake')
    .option('--paymentAmount [value]', 'motes to cover gas costs', '12000000000');

program.parse();

const options = program.opts();

const unstake = async () => {

    const sender = await getSenderKey(options.owner_keys_path, options.keys_algo);

    const args = Args.fromMap({
        scspr_amount: CLValue.newCLUInt256(options.amount),
    });

    const transaction = new ContractCallBuilder()
        .from(sender.publicKey)
        .byPackageHash(options.contract_package_hash)
        .entryPoint('unstake')
        .runtimeArgs(args)
        .payment(Number.parseInt(options.paymentAmount, 10)) // Amount in motes
        .chainName(options.network_name)
        .build();

    transaction.sign(sender);
    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);
    const result = await rpcClient.putTransaction(transaction);
    console.log("Transaction hash: ", result.transactionHash.toHex());
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
unstake();
