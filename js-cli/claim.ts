import {
    Args,
    HttpHandler,
    KeyAlgorithm,
    PrivateKey,
    RpcClient,
    CLValueUInt256,
    ContractCallBuilder,
    ExecutableDeployItem,
    DeployHeader,
    Deploy,
    ModuleBytes,
    StoredVersionedContractByHash,
    ContractHash, CLValueUInt32
} from "casper-js-sdk";
import * as fs from 'fs/promises';
import {BigNumber} from "@ethersproject/bignumber";

const {program} = require('commander');

program
    .option('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--network_name [value]', 'network_name', 'casper-net-1')
    .option('--contract_package_hash [value]', 'staking contract package address')
    .option('--unstake_id [value]', 'id received from unstake transaction')

program.parse();

const options = program.opts();

export const getSenderKey = async (filePath: string) => {
    const pem = await fs.readFile(filePath);
    return PrivateKey.fromPem(pem.toString(),
        KeyAlgorithm.ED25519
    );
}

const claim_deploy = async () => {

    const sender = await getSenderKey(options.owner_keys_path);

    const args = Args.fromMap({
        unstake_id: CLValueUInt32.newCLUInt32(options.unstake_id),
    });

    const session = new ExecutableDeployItem();
    const contractHash = ContractHash.fromJSON(options.contract_package_hash)
    session.storedVersionedContractByHash = new StoredVersionedContractByHash(contractHash, 'claim', args);

    const payment = ExecutableDeployItem.standardPayment("25000000000");

    const deployHeader = DeployHeader.default();
    deployHeader.account = sender.publicKey;
    deployHeader.chainName = options.network_name;
    const deploy = Deploy.makeDeploy(deployHeader, payment, session);
    deploy.sign(sender);

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);
    const result = await rpcClient.putDeploy(deploy);

    console.log(`Deploy Hash: ${result.deployHash.toHex()}`);
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
claim_deploy();
