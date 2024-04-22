// components/Header.js
import React from 'react';
import { useNearWallet } from '../contexts/near.context';

const Header = () => {
  const { wallet } = useNearWallet();

  const handleLogin = () => {
    if (!wallet) return;
      wallet.requestSignIn({
        contractId: NEXT_PUBLIC_PROPOSAL_CONTRACT_ID,
        methodNames: [] 
      });
  };

  const handleLogout = () => {
    if (!wallet) return;
      wallet.signOut();
      window.location.reload(); 
  };

  return (
    <header>
      {wallet && wallet.isSignedIn() ? (
        <>
          <span>Signed in as: {wallet.getAccountId()}</span>
          <button onClick={handleLogout}>Sign out</button>
        </>
      ) : (
        <button onClick={handleLogin}>Sign in with NEAR Wallet</button>
      )}
    </header>
  );
};

export default Header;
