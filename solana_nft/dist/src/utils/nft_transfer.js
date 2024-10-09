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
exports.transferNft = transferNft;
const mpl_token_metadata_1 = require("@metaplex-foundation/mpl-token-metadata");
const umi_bundle_defaults_1 = require("@metaplex-foundation/umi-bundle-defaults");
const wallet_json_1 = __importDefault(require("../wallet/wallet.json"));
const umi_1 = require("@metaplex-foundation/umi");
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = (0, umi_bundle_defaults_1.createUmi)(RPC_ENDPOINT);
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet_json_1.default));
const signer = (0, umi_1.createSignerFromKeypair)(umi, keypair);
umi.use((0, umi_1.signerIdentity)(signer));
function transferNft(mint, newOwner) {
    return __awaiter(this, void 0, void 0, function* () {
        yield (0, mpl_token_metadata_1.transferV1)(umi, {
            mint,
            authority: signer,
            tokenOwner: signer.publicKey,
            destinationOwner: newOwner,
            tokenStandard: mpl_token_metadata_1.TokenStandard.NonFungible,
        }).sendAndConfirm(umi);
    });
}
