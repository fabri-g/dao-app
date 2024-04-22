import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import { Card, Spin, message } from 'antd';
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
            message.error('Failed to load proposal details');
          }
          setLoading(false);
        }).catch(error => {
          message.error('Failed to load proposal details');
          console.error('Error fetching proposal:', error);
          setLoading(false);
        });
      }
    }, [id]);
  
    if (loading) {
      return <Spin size="large" />;
    }
  
    if (!proposal) {
      return <p>Proposal not found</p>;
    }
  
    return (
      <Card title={`Proposal #${id} - ${proposal[0]}`}>
        <p>Description: {proposal[1]}</p>
        <p>Deadline: {new Date(parseInt(proposal[2])).toLocaleString()}</p>
        <p>Minimum Votes: {proposal[3]}</p>
        <p>Options: {proposal[4]}</p>
        <p>Status: {proposal[5]}</p>
        {options.map((option, index) => (
            <VoteComponent key={index} proposalId={id} option={option.trim()} />
        ))}
      </Card>
    );
  };

export default ProposalPage;
