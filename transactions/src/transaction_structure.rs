pub struct Transaction {
    #[wasm_bindgen(skip)]
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,

    #[wasm_bindgen(skip)]
    pub message: Message,
}

/*
Transactions have a total size limit of 1232 bytes. This limit includes both the signatures array and 
the message struct.

This limit is designed to avoid packet fragmentation on typical internet infrastructure. While IPv6 
supports MTUs larger than 9000 bytes, most internet routers use a default MTU of 1500 bytes 
(standard Ethernet). To ensure transactions fit within a single packet without fragmentation, 
Solana uses 1280 bytes (the minimum MTU required for IPv6) minus 48 bytes for network headers 
(40 bytes IPv6 + 8 bytes fragment/UDP header), resulting in the 1232 byte transaction size limit.

Signatures - max of 19, 64 bytes each
Messages - metadata + accounts - max of 35, 32 bytes each 
Total = 1232 bytes
*/