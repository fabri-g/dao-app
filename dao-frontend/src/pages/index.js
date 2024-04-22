import React, { useEffect, useState } from 'react';
/* import Card from 'antd/lib/card';
import Space from 'antd/lib/space';
import Spin from 'antd/lib/spin'; */
import Link from 'next/link';
import { getProposals } from '../hooks/api';

const HomePage = () => {
  const [proposals, setProposals] = useState([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchProposals = async () => {
      setLoading(true);
      const data = await getProposals();
      setProposals(data || []);
      setLoading(false);
    };

    fetchProposals();
  }, []);

  if (loading) {
    return <div>Loading...</div>; 
  }

  if (proposals.length === 0) {
    return <div>No proposals available at the moment.</div>; 
  }

  return (
    <div style={{ width: '100%', padding: '20px' }}>
      {proposals.map(proposalId => (
        <div key={proposalId}>
          <Link href={`/proposals/${proposalId}`}>
            Proposal #{proposalId}
          </Link>
          <p>Details about proposal will go here. Click title to view more.</p>
        </div>
      ))}
    </div>
  );
};
export default HomePage;
