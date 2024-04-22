const express = require('express');
const router = express.Router();
const proposalController = require('../controllers/proposal.controller');

// Routes
router.get('/', proposalController.getAllProposals);
router.get('/:id', proposalController.getProposalById);

module.exports = router;
