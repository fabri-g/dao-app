import React, { useState } from 'react';
import { useNearWallet } from '../contexts/near.context';
import { useAlert } from '../contexts/alert.context';

const VoteComponent = ({ proposalId, optionIndex, optionText }) => {
  const { wallet, contract } = useNearWallet();
  const { showAlert, hideAlert } = useAlert();

  const handleVote = async () => {
    if (!wallet || !wallet.isSignedIn()) {
      showAlert("Please sign in to vote.", 'error');
      return;
    }

    const accountId = wallet.getAccountId();

    try {
      await contract.vote({ 
        proposal_id: parseInt(proposalId, 10), 
        vote_option: optionIndex,
        voter: accountId 
      });
      showAlert(`Vote successful for option: ${optionText}`, 'success');
    } catch (error) {
      console.error('Failed to vote:', error);
      showAlert('Failed to vote. Please try again.', 'error');
    }
  };

  return (
    <div>
      <button
        onClick={handleVote}
        className="px-4 py-2 text-indigo-600 bg-indigo-50 rounded-lg duration-150 hover:bg-indigo-100 active:bg-indigo-200 focus:outline-none focus:ring focus:ring-indigo-300 mx-2 my-1"
      >
        Vote for {optionText}
      </button>
    </div>
  );
};

export default VoteComponent;
