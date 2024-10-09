
# Eldoria Battlestorm

Welcome to **Eldoria Battlestorm**, a turn-based strategy game built on the Solana blockchain! In this game, players engage in strategic battles using their unique characters to claim victory. The game leverages Solana's fast and secure blockchain to ensure a seamless and fair gaming experience.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Game Overview](#game-overview)
- [Smart Contracts](#smart-contracts)
- [Building and Running](#building-and-running)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust and Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Solana CLI**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **Node.js and npm**: [Install Node.js](https://nodejs.org/)
- **Unity** (for mobile): [Install Unity](https://unity.com/)

### Installation

Clone the repository:

```sh
git clone https://github.com/yourusername/eldoria-battlestorm.git
cd eldoria-battlestorm
```

Install the dependencies:

```sh
cargo build-bpf
```

## Project Structure

The **Eldoria Battlestorm** repository is organized into several key directories and files, reflecting different aspects of the project:

```
/EldoriaBattlestorm
├── client/                       # Client code for the Unity-based mobile game
│   └── ...                       # Unity assets, scripts, and configuration
│
├── echoes_of_eldoria_api/         # API built using Flask for backend logic
│   ├── app.py                    # Main Flask server
│   ├── Dockerfile                # Docker setup for the API
│   ├── requirements.txt          # Dependencies for the Flask API
│   └── README.md                 # Documentation for the API
│
├── eldoria_battlestorm_main/      # Smart contracts and main game logic (Rust/TS)
│   ├── src/                      # Source code for Rust smart contracts
│   │   ├── entrypoint.rs          # Solana program entrypoint
│   │   ├── instruction.rs         # Instruction parsing and handling
│   │   ├── processor.rs           # Main logic for game processing
│   │   ├── state.rs               # Data structures for game state
│   │   ├── error.rs               # Error handling for the program
│   └── Cargo.toml                # Rust project dependencies and configuration
│
├── solana_nft/                    # NFT-related functionality for Solana blockchain
│   ├── src/                      # NFT-related source code and utilities
│   │   ├── controller/            # Controller logic for NFT creation and management
│   │   ├── router/                # Routes for handling NFT-related requests
│   │   └── utils/                 # Utility functions for image upload, minting, and transfer
│   └── wallet/                   # Solana wallet for transaction management
│
├── LICENSE                       # License for the project
├── README.md                     # Main project documentation
└── package.json                  # Node.js configuration and dependencies
```

## Game Overview

Eldoria Battlestorm is a turn-based strategy game where players control a team of characters, each with unique abilities. The objective is to strategically outmaneuver your opponent and deplete their characters' health to win the game.

### Characters

Each character has the following attributes:

- **mint**: Unique identifier for the character.
- **health**: Health points of the character.
- **hit**: Attack power of the character.
- **xp**: Experience points of the character.

### Game State

The game state is managed by the `GameState` struct, which includes:

- **is_initialized**: Indicates if the game is initialized.
- **initializer**: Public key of the player who initialized the game.
- **guest**: Public key of the guest player.
- Character attributes for both initializer and guest.
- **last_play_time**: Timestamp of the last move.
- **whose_turn**: Indicates whose turn it is to play.

### Actions

Players can perform actions such as:

- **Attack**: Engage in combat with the opponent's characters.
- **Move**: Move characters strategically on the board (implementation details pending).

## Smart Contracts

The core game logic is implemented in Rust using Borsh for serialization. The key structs are:

- `RandomNumber`: Represents a random number for game mechanics.
- `GameState`: Manages the state of the game.
- `Character`: Defines the attributes of a character.
- `Attack`: Represents an attack action.
- `PlayerAccount`: Tracks player stats.

## Building and Running

Compile the smart contracts:

```sh
cargo build-bpf
```

Deploy the contracts to the Solana blockchain:

```sh
solana program deploy path/to/your_program.so
```

Run the Unity frontend (if applicable):

```sh
npm install
npm start
```

## Contributing

Contributions are welcome! If you'd like to contribute to **Eldoria Battlestorm**, follow these steps:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Create a new Pull Request.

## License

Eldoria Battlestorm is licensed under the MIT License. See `LICENSE` for more information.

---

Enjoy playing **Eldoria Battlestorm**! For any issues or support, please open an issue on GitHub.

---

This README now reflects the updated project structure, with details about the client, API, and smart contract files.
