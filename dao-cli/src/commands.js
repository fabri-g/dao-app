const yargs = require("yargs");
const nearClient = require("./near.client");

function setupCommands() {
    yargs
        .scriptName("dao-cli")
        .usage('$0 <cmd> [args]')
        .command('create-proposal', 'Create a new proposal', (yargs) => {
            return yargs.options({
                title: { type: 'string', describe: 'Title of the proposal', demandOption: true },
                description: { type: 'string', describe: 'Description of the proposal', demandOption: true },
                deadline: { type: 'string', describe: 'Deadline for the proposal as timestamp', demandOption: true },
                options: { type: 'string', describe: 'Comma-separated options for the proposal', demandOption: true },
                minimumVotes: { type: 'string', describe: 'Minimum votes required', demandOption: true }
            });
        }, async (argv) => {
            await nearClient.createProposal(argv);
        })
        .command('finalize-proposal', 'Finalize a proposal', (yargs) => {
            return yargs.option('proposalId', { type: 'string', describe: 'ID of the proposal to finalize', demandOption: true });
        }, async (argv) => {
            await nearClient.finalizeProposal(argv.proposalId);
        })
        .help()
        .alias('help', 'h')
        .argv;
}

module.exports = { setupCommands };
