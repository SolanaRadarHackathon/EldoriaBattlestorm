
# Echoes of Eldoria API


## Overview

Echoes of Eldoria is a turn-based strategy game where players engage in battles using characters with unique abilities and attributes.

## Features

- **Random Number Generation**: Endpoint to generate a random number.
- **Game State Management**: Endpoint to manage and retrieve game states.
- **Character Management**: Endpoint to manage and retrieve character data.
- **Attack Management**: Endpoint to manage and execute attacks.
- **Player Account Management**: Endpoint to manage player accounts and victories.

## Installation

To run the Echoes of Eldoria API Server locally, follow these steps:

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd echoesofeldoria
   ```

2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

3. Run the server:
   ```bash
   python app.py
   ```

## API Endpoints

- **Random Number Generation**
    - Endpoint: `/random_number`
    - Method: POST
    - Parameters: `random_number`
    - Returns: Generated random number.

- **Game State Management**
    - Endpoint: `/game_state`
    - Method: POST
    - Parameters: `is_initialized`, `initializer`, `guest`, and characters (`initializers_character_1` to `initializers_character_5`, `guests_character_1` to `guests_character_5`), `last_play_time`, `whose_turn`.
    - Returns: Game state information.

- **Character Management**
    - Endpoint: `/character`
    - Method: POST
    - Parameters: `mint`, `health`, `hit`, `xp`.
    - Returns: Character data.

- **Attack Management**
    - Endpoint: `/attack`
    - Method: POST
    - Parameters: `attack_to`, `attacker`.
    - Returns: Attack details.

- **Player Account Management**
    - Endpoint: `/player_account`
    - Method: POST
    - Parameters: `address`, `victories`.
    - Returns: Player account information.

## Contributing

Contributions to Echoes of Eldoria are welcome! Here's how you can contribute:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

