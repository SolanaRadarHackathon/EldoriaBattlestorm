
# Eldoria Battlestorm

## Overview
This project is a Solana-based NFT minting and transfer system using the Metaplex protocol. It integrates key functionalities such as uploading NFT images, minting NFTs, and transferring ownership using the UMI library and the Metaplex Token Metadata program.

## Features
- **Mint NFTs**: Upload images, set metadata, and mint NFTs on the Solana blockchain.
- **Transfer NFTs**: Easily transfer ownership of NFTs to other users on Solana.
- **Solana Devnet Support**: Tested on the Solana devnet environment.

## Project Structure
```bash
.
├── dist
│   └── src
│       ├── controller
│       │   └── solana.controller.js     # Handles NFT minting requests (JavaScript compiled)
│       ├── router
│       │   └── solana.router.js         # Defines API routes for Solana interaction (JavaScript compiled)
│       ├── utils
│       │   ├── nft_image.js             # Utility for uploading NFT images (JavaScript compiled)
│       │   ├── nft_metadata.js          # Utility for setting NFT metadata (JavaScript compiled)
│       │   ├── nft_mint.js              # Utility for minting NFTs (JavaScript compiled)
│       │   └── nft_transfer.js          # Utility for transferring NFTs (JavaScript compiled)
│       ├── wallet
│       │   └── index.js                 # Wallet configuration (JavaScript compiled)
│       └── server.js                    # Server logic (JavaScript compiled)
├── src
│   ├── controller
│   │   └── solana.controller.ts         # Handles NFT minting requests
│   ├── router
│   │   └── solana.router.ts             # Defines API routes for Solana interaction
│   ├── utils
│   │   ├── nft_image.ts                 # Utility for uploading NFT images
│   │   ├── nft_metadata.ts              # Utility for setting NFT metadata
│   │   ├── nft_mint.ts                  # Utility for minting NFTs
│   │   └── nft_transfer.ts              # Utility for transferring NFTs
│   ├── wallet
│   │   └── wallet.json                  # Solana wallet for signing transactions
│   └── server.ts                        # Starts the server
├── package.json                         # Node project dependencies
├── package-lock.json                    # Lockfile for dependencies
├── tsconfig.json                        # TypeScript configuration
└── rerun.sh                             # Script to rerun the server
```

## Getting Started

### Prerequisites
- Node.js (v16 or higher)
- Typescript
- Solana CLI
- A funded Solana devnet wallet

### Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/eldoria-battlestorm.git
    cd eldoria-battlestorm
    ```

2. Install dependencies:
    ```bash
    npm install
    ```

3. Create a `.env` file and configure it for your RPC URL, wallet, etc.
    ```bash
    SOLANA_RPC_URL=https://api.devnet.solana.com
    ```

4. Ensure your Solana wallet has devnet tokens:
    ```bash
    solana airdrop 1
    ```

### Running the Server
To start the server:
```bash
npm run start
```

The server will be running on `http://localhost:8000`.

### API Endpoints

- **POST /solana/nft**
  - Create and mint an NFT.
  - **Body parameters**:
    - `name` (string): Name of the NFT.
    - `symbol` (string): Symbol for the NFT.
    - `description` (string): Description for the NFT.
    - `image` (string): Base64-encoded image data.
    - `receiver` (string): Solana wallet address of the receiver.

- **POST /solana/transfer**
  - Transfer an NFT to a new owner.
  - **Body parameters**:
    - `mint` (string): Mint address of the NFT.
    - `receiver` (string): New owner's wallet address.

### Example Usage

#### Minting an NFT:
```bash
curl -X POST http://localhost:8000/solana/nft   -H "Content-Type: application/json"   -d '{
    "name": "My NFT",
    "symbol": "MNFT",
    "description": "This is my first NFT",
    "image": "base64-image-data-here",
    "receiver": "receiver-public-key-here"
  }'
```

#### Transferring an NFT:
```bash
curl -X POST http://localhost:8000/solana/transfer   -H "Content-Type: application/json"   -d '{
    "mint": "mint-address-here",
    "receiver": "new-owner-public-key-here"
  }'
```

### Acknowledgements
- Built using the [UMI](https://github.com/metaplex-foundation/umi) and [Metaplex Token Metadata](https://github.com/metaplex-foundation/metaplex-program-library/tree/master/token-metadata) libraries.
- Powered by [Solana](https://solana.com) blockchain.

### License
This project is licensed under the MIT License.
