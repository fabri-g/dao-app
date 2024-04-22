import React, { useEffect, useState } from 'react';
import { Card, Space, Spin } from 'antd';
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

  return (
    <Space direction="vertical" size="middle" style={{ width: '100%', padding: 20 }}>
      {loading ? (
        <Spin size="large" />
      ) : (
        proposals.map(proposalId => (
          <Card
            key={proposalId}
            title={<Link href={`/proposals/${proposalId}`}><a>Proposal #{proposalId}</a></Link>}
            style={{ width: '100%' }}
          >
            <p>Details about proposal will go here. Click title to view more.</p>
          </Card>
        ))
      )}
    </Space>
  );
};

export default HomePage;
