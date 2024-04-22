import axios from 'axios';


export const getProposals = async () => {
  try {
    const { data } = await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/proposals`);
    return data;
  } catch (error) {
    console.error('Error fetching proposals', error);
    return [];
  }
};

export const getProposalById = async (id) => {
  try {
    const response = await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/proposals/${id}`);
    return response.data;
  } catch (error) {
    console.error('Error fetching proposal details', error);
  }
};
