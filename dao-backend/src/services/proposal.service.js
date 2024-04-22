const nearClient = require('../api/near.client');

async function getAllProposals() {
    const contract = await nearClient.initNear();
    return contract.list_proposals();
}

async function getProposalById(proposalId) {
    const contract = await nearClient.initNear();
    return contract.get_proposal({ proposal_id: parseInt(proposalId) });
}

module.exports = {
    getAllProposals,
    getProposalById,
};