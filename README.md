# NEAR DAO
This project is a full-stack dApp built on the NEAR blockchain that enables DAO governance. The DAO allows token holders to vote on proposals created by the admin using DAO tokens. The application provides a complete ecosystem, including smart contracts, a CLI, a backend with REST APIs, and a responsive frontend for user interaction.

## Features
### Smart Contracts
- **Proposals:** Create proposals with a title, description, deadline, minimum votes, and two voting options (A and B).
- **Voting:** Token holders can cast votes for Option A or Option B.
- **Status Tracking:** Automatically tracks proposal status (e.g., Open, Closed, Option A Won, Option B Won, Rejected).

### Command-Line Interface
- **Admin Features:**
    - Create new proposals on-chain.
    - View all proposals and their statuses.
    - Finalize proposals once the deadline is met.
 
### Backend
- Sync with Blockchain: Syncs and tracks all proposals on the blockchain.
- API Endpoints: Serves proposal data, including details, vote counts, and statuses.
- Real-time Updates: Keeps proposal data up-to-date as changes occur on-chain.

### Frontend
- **Proposal Listing:** Displays all proposals with their statuses (Pending, Closed, Option A Won, Option B Won, etc.).
- **Voting Interface:** Allows token holders to cast their votes for Option A or Option B.
- **Responsive Design:** Fully responsive UI for seamless interaction across devices.

## Technologies Used
### Smart Contracts
- **NEAR Blockchain:** All logic runs on the NEAR blockchain for decentralized governance.
- **Rust:** Used for writing secure and efficient smart contracts.
### CLI
- **Rust:** Enables interaction with smart contracts via the NEAR CLI and custom scripts.
### Backend
- **Node.js:** Provides a RESTful API to expose proposal data and statuses.
- **NEAR SDK:** Used to interact with the NEAR blockchain and sync proposal data.
### Frontend
- **React.js:** For building the user interface.
- **Next.js:** For server-side rendering and improved performance.
- **Ant Design:** A React-based UI library for a consistent and professional look.

## Getting Started
### Prerequisites
- Install NEAR CLI: Follow the official guide to install the NEAR CLI.
- Install Rust: Install Rust for compiling smart contracts.
- Node.js: Install Node.js for running the backend and frontend.
### Clone the Repository
```
git clone https://github.com/fabri-g/dao-app.git
```
## Environment Variables
Make sure to create .env files in the backend and frontend directories.

## Authors
[fabri-g](https://github.com/fabri-g)
