"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.mintNft = mintNft;
const umi_bundle_defaults_1 = require("@metaplex-foundation/umi-bundle-defaults");
const umi_1 = require("@metaplex-foundation/umi");
const mpl_token_metadata_1 = require("@metaplex-foundation/mpl-token-metadata");
const wallet_json_1 = __importDefault(require("../wallet/wallet.json"));
const bs58_1 = __importDefault(require("bs58"));
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = (0, umi_bundle_defaults_1.createUmi)(RPC_ENDPOINT);
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet_json_1.default));
const myKeypairSigner = (0, umi_1.createSignerFromKeypair)(umi, keypair);
umi.use((0, umi_1.signerIdentity)(myKeypairSigner));
umi.use((0, mpl_token_metadata_1.mplTokenMetadata)());
const mint = (0, umi_1.generateSigner)(umi);
function mintNft(name, symbol, uri, owner) {
    return __awaiter(this, void 0, void 0, function* () {
        let tx = (0, mpl_token_metadata_1.createNft)(umi, {
            mint,
            name: name,
            symbol: symbol,
            uri: uri,
            tokenOwner: owner,
            sellerFeeBasisPoints: (0, umi_1.percentAmount)(1, 2)
        });
        let result = yield tx.sendAndConfirm(umi);
        const signature = bs58_1.default.encode(result.signature);
        console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
        console.log("Mint Address: ", mint.publicKey);
        return mint;
    });
}
