import React from 'react';
import { useNearWallet } from '../contexts/near.context';
import { message } from 'antd';

const VoteComponent = ({ proposalId, option }) => {
  const { wallet, contract } = useNearWallet();

  const handleVote = async () => {
    if (!wallet.isSignedIn()) {
      message.error("Please sign in to vote.");
      return;
    }

    try {
      await contract.vote({ proposal_id: proposalId, vote_option: option });
      message.success('Vote successful');
    } catch (error) {
      console.error('Failed to vote:', error);
      message.error('Failed to vote');
    }
  };

  return <button onClick={handleVote}>Vote for Option {option}</button>;
};

export default VoteComponent;
