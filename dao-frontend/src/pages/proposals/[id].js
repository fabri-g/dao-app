import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { getProposalById } from '../../hooks/api';
import VoteComponent from '../../components/vote.button';

const ProposalPage = () => {
  const router = useRouter();
  const { id } = router.query;
  const [proposal, setProposal] = useState(null);
  const [loading, setLoading] = useState(true);
  const [options, setOptions] = useState([]);

  useEffect(() => {
    if (id) {
      getProposalById(id).then(proposalData => {
        if (proposalData) {
          setProposal(proposalData);
          setOptions(proposalData[4].split(','));
        } else {
          console.error('Failed to load proposal details');
        }
        setLoading(false);
      }).catch(error => {
        console.error('Failed to load proposal details:', error);
        setLoading(false);
      });
    }
  }, [id]);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (!proposal) {
    return <div>Proposal not found</div>;
  }

  return (
    <div className="max-w-2xl mx-auto px-4 py-6">
      <div className="bg-white shadow-lg rounded-lg p-6">
        <h1 className="text-2xl font-bold mb-4">{`Proposal #${id} - ${proposal[0]}`}</h1>
        <p className="text-gray-700 mb-2"><strong>Description:</strong> {proposal[1]}</p>
        <p className="text-gray-700 mb-2"><strong>Deadline:</strong> {proposal[2]}</p>
        <p className="text-gray-700 mb-2"><strong>Minimum Votes:</strong> {proposal[3]}</p>
        <p className="text-gray-700 mb-2"><strong>Status:</strong> {proposal[5]}</p>
        <h2 className="text-xl font-semibold mt-4 mb-2">Vote on Options</h2>
        {options.map((option, index) => (
          <VoteComponent key={index} proposalId={id} optionIndex={index} optionText={option.trim()} />
        ))}
      </div>
    </div>
  );
};

export default ProposalPage;
