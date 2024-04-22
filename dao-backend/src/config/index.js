require('dotenv').config();

const config = {
    networkId: process.env.NETWORK_ID,
    nodeUrl: process.env.NODE_URL,
    walletUrl: process.env.WALLET_URL,
    helperUrl: process.env.HELPER_URL,
    explorerUrl: process.env.EXPLORER_URL,
    daoContractId: process.env.DAO_CONTRACT_ID,
    adminAccountId: process.env.ADMIN_ACCOUNT_ID,
}

module.exports = config;