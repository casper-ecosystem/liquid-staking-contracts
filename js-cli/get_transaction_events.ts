import {
    HttpHandler,
    RpcClient,
} from "casper-js-sdk";
import {Parser, Event} from "@make-software/ces-js-parser";

const {program} = require('commander');

program
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}', 'http://localhost:11101/rpc')
    .option('--contract_hash [value]', 'staking contract address')
    .option('--transaction_hash [value]', 'unstake transaction hash')

program.parse();

const options = program.opts();

const get_transaction_events = async () => {

    const rpcHandler = new HttpHandler(options.node_url);
    const rpcClient = new RpcClient(rpcHandler);

    const parser = await Parser.create(rpcClient, [
        options.contract_hash,
    ]);

    const result = await rpcClient.getTransactionByTransactionHash(options.transaction_hash);

    console.log(`Transaction Hash: ${result.transaction.hash.toHex()}`);

    const events = parser.parseExecutionResult(result.executionInfo.executionResult);

    const  printEvent = (event: Event) => {
        console.log('Event:');
        console.log(`  Name: ${event.name}`);
        console.log(`  Contract Hash: ${event.contractHash.toHex()}`);
        console.log(`  Contract Package Hash: ${event.contractPackageHash.toHex()}`);
        console.log(`  Event ID: ${event.eventId}`);
        console.log('  Data:');
        for (const [key, value] of Object.entries(event.data)) {
            console.log(`    ${key}: ${value.toString()}`);
        }
    }

    events.forEach((e) => printEvent(e.event));
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
get_transaction_events();
