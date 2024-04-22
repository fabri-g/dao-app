const nearAPI = require("near-api-js");
const { connect, keyStores, Contract, KeyPair } = nearAPI;

const keyStore = new keyStores.InMemoryKeyStore();
const keyPair = KeyPair.fromString(process.env.ADMIN_PRIVATE_KEY);
keyStore.setKey(process.env.NETWORK_ID, process.env.ADMIN_ACCOUNT_ID, keyPair);

let contractInstance;

async function initNear() {
    if (contractInstance) return contractInstance;

    try {
        const nearConfig = {
            networkId: process.env.NETWORK_ID,
            keyStore: keyStore,
            nodeUrl: process.env.NODE_URL,
            walletUrl: process.env.WALLET_URL,
            helperUrl: process.env.HELPER_URL,
            explorerUrl: process.env.EXPLORER_URL,
        };

        const near = await connect(nearConfig);
        const adminAccount = await near.account(process.env.ADMIN_ACCOUNT_ID);

        contractInstance = new Contract(adminAccount, process.env.PROPOSAL_CONTRACT_ID, {
            viewMethods: ['list_proposals', 'get_proposal'],
            changeMethods: ['create_proposal', 'update_status', 'vote'],
        });

        return contractInstance;
    } catch (error) {
        console.error('Failed to initialize NEAR client:', error);
        throw error;
    }
}

module.exports = {
    initNear
};