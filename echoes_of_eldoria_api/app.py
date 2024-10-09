# app.py
from flask import Flask, request, jsonify
from borsh import BorshSerialize, BorshDeserialize
from solana.publickey import PublicKey
from solana.rpc.api import Client
from solana.transaction import Transaction, TransactionInstruction
from solana.system_program import SystemProgram
from solana.keypair import Keypair
from base58 import b58decode
import json

app = Flask(__name__)

API_URL = "https://api.devnet.solana.com"
client = Client(API_URL)

secret_key = [
    95, 168, 25, 23, 8, 29, 184, 48, 24, 224, 178, 228, 249, 73, 243, 66, 254, 205, 99, 69, 118,
    127, 28, 4, 237, 252, 228, 240, 83, 0, 28, 210, 158, 98, 151, 229, 149, 244, 142, 17, 58,
    193, 85, 159, 194, 75, 48, 136, 151, 11, 112, 32, 40, 95, 178, 76, 250, 15, 242, 142, 3, 71, 163, 244
]

keypair = Keypair.from_secret_key(bytes(secret_key))

program_id = PublicKey("Fkohtb7Y6k8QCt7d2SYJyGDNMYT3pu5GZYczDNXdTun2")
mpl_core = PublicKey('CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d')

creator = PublicKey('BfGcY8YBCdgp9eE5V8Dem3nKzU367YKa41vEeHp5xnkj')
burn_auth = PublicKey('CdaBE1S8ew2BpcsnqpMBezmWDu2QdD6AY8euovhVph4V')
update_auth = PublicKey('HQ8643M6MBaxwdJGPwZbMMJsx9nnLkzZMxxXS7oYVv2Q')

@app.route('/')
def index():
    return "Echoes of Eldoria API Server"

@app.route('/create_nft', methods=['POST'])
def create_nft():
    try:
        asset = Keypair.generate()
        transaction = Transaction()
        # Add the create instruction here
        # transaction.add(...)
        response = client.send_transaction(transaction, keypair)
        return jsonify(response), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 400

@app.route('/get_pda_address', methods=['GET'])
def get_pda_address():
    try:
        burn = PublicKey.find_program_address([b"burn"], program_id)
        update = PublicKey.find_program_address([b"update"], program_id)
        return jsonify({"burn": burn[0].to_base58().decode(), "update": update[0].to_base58().decode()}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 400

@app.route('/upgrade_nft', methods=['POST'])
def upgrade_nft():
    try:
        nft = PublicKey(request.json.get('nft'))
        update = PublicKey.find_program_address([b"update"], program_id)
        noop = PublicKey("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV")
        ix = TransactionInstruction(
            program_id=program_id,
            keys=[
                {'pubkey': update[0], 'is_signer': False, 'is_writable': True},
                {'pubkey': keypair.public_key, 'is_signer': True, 'is_writable': True},
                {'pubkey': nft, 'is_signer': False, 'is_writable': True},
                {'pubkey': mpl_core, 'is_signer': False, 'is_writable': True},
                {'pubkey': SystemProgram.program_id, 'is_signer': False, 'is_writable': True},
                {'pubkey': noop, 'is_signer': False, 'is_writable': True},
            ],
            data=bytes([4])
        )
        transaction = Transaction().add(ix)
        response = client.send_transaction(transaction, keypair)
        return jsonify(response), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 400

@app.route('/burn_nft', methods=['POST'])
def burn_nft():
    try:
        nft = PublicKey(request.json.get('nft'))
        burn = PublicKey.find_program_address([b"burn"], program_id)
        ix = TransactionInstruction(
            program_id=program_id,
            keys=[
                {'pubkey': burn[0], 'is_signer': False, 'is_writable': True},
                {'pubkey': nft, 'is_signer': False, 'is_writable': True},
                {'pubkey': mpl_core, 'is_signer': False, 'is_writable': True},
            ],
            data=bytes([3])
        )
        transaction = Transaction().add(ix)
        response = client.send_transaction(transaction, keypair)
        return jsonify(response), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 400


@app.route('/random_number', methods=['POST'])
def random_number():
    data = request.get_json()
    random_number = data.get('random_number')
    response = RandomNumber(random_number=random_number)
    return jsonify(response.__dict__)

@app.route('/game_state', methods=['POST'])
def game_state():
    data = request.get_json()
    game_state = GameState(**data)
    return jsonify(game_state.__dict__)

@app.route('/character', methods=['POST'])
def character():
    data = request.get_json()
    character = Character(**data)
    return jsonify(character.__dict__)

@app.route('/attack', methods=['POST'])
def attack():
    data = request.get_json()
    attack = Attack(**data)
    return jsonify(attack.__dict__)

@app.route('/player_account', methods=['POST'])
def player_account():
    data = request.get_json()
    player_account = PlayerAccount(**data)
    return jsonify(player_account.__dict__)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)