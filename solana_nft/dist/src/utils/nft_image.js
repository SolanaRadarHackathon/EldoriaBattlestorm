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
exports.uploadNftImage = uploadNftImage;
const wallet_json_1 = __importDefault(require("../wallet/wallet.json"));
const umi_bundle_defaults_1 = require("@metaplex-foundation/umi-bundle-defaults");
const umi_1 = require("@metaplex-foundation/umi");
const umi_uploader_bundlr_1 = require("@metaplex-foundation/umi-uploader-bundlr");
const umi_uploader_irys_1 = require("@metaplex-foundation/umi-uploader-irys");
// Create a devnet connection
function uploadNftImage(base64Image) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const umi = (0, umi_bundle_defaults_1.createUmi)('https://api.devnet.solana.com');
            const bundlrUploader = (0, umi_uploader_bundlr_1.createBundlrUploader)(umi);
            let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet_json_1.default));
            const signer = (0, umi_1.createSignerFromKeypair)(umi, keypair);
            umi.use((0, umi_1.signerIdentity)(signer));
            umi.use((0, umi_uploader_irys_1.irysUploader)());
            let content = Buffer.from(base64Image, 'base64');
            const image = (0, umi_1.createGenericFile)(content, "Generug");
            console.log(image);
            const [myUri] = yield umi.uploader.upload([image]);
            console.log("Your image URI: ", myUri);
            return myUri;
        }
        catch (error) {
            console.log("Oops.. Something went wrong", error);
            return "";
        }
    });
}
