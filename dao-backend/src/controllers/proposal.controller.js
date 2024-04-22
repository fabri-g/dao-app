const proposalService = require('../services/proposal.service');

exports.getAllProposals = async (req, res) => {
    try {
        const proposals = await proposalService.getAllProposals();
        res.json(proposals);
    } catch (error) {
        console.error('Failed to fetch proposals:', error);
        res.status(500).json({ error: 'Failed to fetch proposals' });
    }
};

exports.getProposalById = async (req, res) => {
    const { id } = req.params;
    try {
        const proposal = await proposalService.getProposalById(id);
        if (!proposal) {
            return res.status(404).json({ error: 'Proposal not found' });
        }
        res.json(proposal);
    } catch (error) {
        console.error('Failed to fetch proposal:', error);
        res.status(500).json({ error: 'Failed to fetch proposal' });
    }
};