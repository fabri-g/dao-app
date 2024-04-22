import { connect, keyStores, WalletConnection } from 'near-api-js';

const nearConfig = {
  networkId: process.env.NEXT_PUBLIC_NEAR_NETWORK_ID,
  nodeUrl: process.env.NEXT_PUBLIC_NEAR_NODE_URL,
  walletUrl: process.env.NEXT_PUBLIC_NEAR_WALLET_URL,
  helperUrl: process.env.NEXT_PUBLIC_NEAR_HELPER_URL,
  explorerUrl: process.env.NEXT_PUBLIC_NEAR_EXPLORER_URL,
};

export async function initNear() {
  if (typeof window !== "undefined") {
    const { keyStores } = await import('near-api-js');
    nearConfig.keyStore = new keyStores.BrowserLocalStorageKeyStore();

    const near = await connect(nearConfig);
    const wallet = new WalletConnection(near, 'daoApp');

    return { near, wallet };
  }
  return { near: null, wallet: null };
}
