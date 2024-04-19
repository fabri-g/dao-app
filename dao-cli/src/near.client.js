// src/near.client.js
const nearAPI = require("near-api-js");
const config = require("./config");

const { connect } = nearAPI;

const connectionConfig = {
    networkId: config.networkId,
    keyStore: config.keyStore,
    nodeUrl: config.nodeUrl,
    walletUrl: config.walletUrl,
    helperUrl: config.helperUrl,
    explorerUrl: config.explorerUrl,
};

// Function to create a proposal 
async function createProposal({ title, description, deadline, options, minimumVotes }) {
    try {
        const near = await connect(connectionConfig);
        const account = await near.account(config.adminAccountId);

        const args = {
            title,
            description,
            deadline: parseInt(deadline, 10),
            options_vec: options.split(','),
            minimum_votes: parseInt(minimumVotes, 10)
        };

        console.log("Sending createProposal transaction...");
        const result = await account.functionCall({
            contractId: config.daoContractId,
            methodName: "create_proposal",
            args,
            gas: "300000000000000",
            attachedDeposit: "0"
        });

        console.log('Transaction successful:', result.transaction.hash);
    } catch (error) {
        console.error('Failed to create proposal:', error);
        throw error;
    }
}

// Function to finalize a proposal 
async function finalizeProposal(proposalId) {
    try {
        const near = await connectToNear();
        const account = await near.account(config.adminAccountId);

        console.log("Sending finalizeProposal transaction...");
        const result = await account.functionCall({
            contractId: config.daoContractId,
            methodName: "finalize_proposal",
            args: { proposal_id: parseInt(proposalId, 10) },
            gas: "300000000000000",
            attachedDeposit: "0"
        });

        console.log('Transaction successful:', result.transaction.hash);
    } catch (error) {
        console.error('Failed to finalize proposal:', error);
        throw error;
    }
}

module.exports = { createProposal, finalizeProposal };
