import React, { createContext, useContext, useEffect, useState } from 'react';
import { initNear } from '../utils/near.config';
import { Contract } from 'near-api-js';

const NearContext = createContext(null);

export const NearProvider = ({ children }) => {
  const [wallet, setWallet] = useState(null);
  const [contract, setContract] = useState(null);

  useEffect(() => {
    initNear().then(({ near, wallet }) => {
      setWallet(wallet);
      if (wallet.isSignedIn()) {
        const contractInstance = new Contract(wallet.account(), NEXT_PUBLIC_PROPOSAL_CONTRACT_ID, {
          viewMethods: ['list_proposals', 'get_proposal', 'get_votes'],
          changeMethods: ['vote'],
        });
        setContract(contractInstance);
      }
    });
  }, []);

  return <NearContext.Provider value={{ wallet }}>
    {children}
  </NearContext.Provider>;
};

export const useNearWallet = () => useContext(NearContext);
