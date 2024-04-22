const express = require('express');
const router = express.Router();
const proposalRoutes = require('./proposal.route.js');

// Routes
router.use('/proposals', proposalRoutes);

module.exports = router;
