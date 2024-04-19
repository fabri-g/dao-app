// src/config.js
require('dotenv').config();
const nearAPI = require("near-api-js");
const { keyStores } = nearAPI;
const homedir = require("os").homedir();
const credentialsPath = require("path").join(homedir, process.env.CREDENTIALS_DIR);

const myKeyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

module.exports = {
    networkId: process.env.NETWORK_ID,
    keyStore: myKeyStore,
    nodeUrl: process.env.NODE_URL,
    walletUrl: process.env.WALLET_URL,
    helperUrl: process.env.HELPER_URL,
    explorerUrl: process.env.EXPLORER_URL,
    adminAccountId: process.env.ADMIN_ACCOUNT_ID,
    daoContractId: process.env.DAO_CONTRACT_ID
};
