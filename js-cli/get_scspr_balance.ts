import {
    HttpHandler,
    ParamDictionaryIdentifier,
    ParamDictionaryIdentifierContractNamedKey,
    RpcClient,
} from "casper-js-sdk";


const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .requiredOption('--contract_hash [value]', 'staking contract address')
    .requiredOption('--account_hash [value]', 'account hash')

program.parse();

const options = program.opts();

const get_scspr_balance = async () => {

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);

    const accountHashBytes = Uint8Array.from(Buffer.from("00"+options.account_hash, 'hex'));
    const accountHashBase64 = Buffer.from(accountHashBytes).toString('base64');

    const paramDictionaryIdentifier = new ParamDictionaryIdentifier(
        undefined,
        new ParamDictionaryIdentifierContractNamedKey(
            "hash-" + options.contract_hash,
            "balances",
            accountHashBase64),
        undefined,
        undefined);

    const result = await rpcClient.getDictionaryItemByIdentifier(null, paramDictionaryIdentifier);
    const scsprBalance = result.storedValue.clValue.ui256.toNumber();

    console.log("sCSPR balance:", scsprBalance);
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
get_scspr_balance();
