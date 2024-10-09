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
Object.defineProperty(exports, "__esModule", { value: true });
exports.createNFT = void 0;
const nft_image_1 = require("../utils/nft_image");
const nft_metadata_1 = require("../utils/nft_metadata");
const nft_mint_1 = require("../utils/nft_mint");
const umi_1 = require("@metaplex-foundation/umi");
const createNFT = (req, res, next) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        //Upload image 
        const imageUri = yield (0, nft_image_1.uploadNftImage)(req.body.image);
        console.log("Image URI: " + imageUri);
        const metadataUri = yield (0, nft_metadata_1.setNftMetadata)(req.body.name, req.body.symbol, req.body.description, imageUri);
        console.log("Metadata URI: " + metadataUri);
        const mint = yield (0, nft_mint_1.mintNft)(req.body.name, req.body.symbol, metadataUri, (0, umi_1.publicKey)(req.body.receiver));
        console.log("Mint address: " + mint);
        //await transferNft(mint.publicKey,publicKey(req.body.receiver))
        res.status(200).send({ pubkey: imageUri });
    }
    catch (error) {
        console.error(error);
        res.sendStatus(500);
    }
});
exports.createNFT = createNFT;
