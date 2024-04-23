import React, { useEffect, useState } from 'react';
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
    <div className="max-w-screen-xl mx-auto px-4 md:px-8">
      <h3 className="text-gray-800 text-xl font-bold sm:text-2xl">
        Proposal List
      </h3>
      <div className="mt-12 shadow-sm border rounded-lg overflow-x-auto">
        <table className="w-full table-auto text-sm text-left">
          <thead className="bg-gray-50 text-gray-600 font-medium border-b">
            <tr>
              <th className="py-3 px-6">Proposal #</th>
              <th className="py-3 px-6">Description</th>
            </tr>
          </thead>
          <tbody className="text-gray-600 divide-y">
            {proposals.map((proposal, idx) => (
              <tr key={idx}>
                <td className="px-6 py-4 whitespace-nowrap">
                  <Link className="text-blue-500 hover:text-blue-700" href={`/proposals/${proposal[0]}`}>
                    Proposal #{proposal[0]} - {proposal[1]}
                  </Link>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  {proposal[2]}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
export default HomePage;
