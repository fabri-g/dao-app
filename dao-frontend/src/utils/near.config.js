import { connect, keyStores, WalletConnection } from 'near-api-js';

const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();

const nearConfig = {
  networkId: process.env.NEXT_PUBLIC_NEAR_NETWORK_ID,
  keyStore: myKeyStore,
  nodeUrl: process.env.NEXT_PUBLIC_NEAR_NODE_URL,
  walletUrl: process.env.NEXT_PUBLIC_NEAR_WALLET_URL,
  helperUrl: process.env.NEXT_PUBLIC_NEAR_HELPER_URL,
  explorerUrl: process.env.NEXT_PUBLIC_NEAR_EXPLORER_URL,
};

export async function initNear() {
  const near = await connect(nearConfig);
  const wallet = new WalletConnection(near);
  return { near, wallet };
}
