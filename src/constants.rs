pub enum VitaBotPublicKeys {
   VitaBot,
   VitaBotBeta 
}

pub fn get_public_key(version: VitaBotPublicKeys) -> String {
    match version {
        VitaBotPublicKeys::VitaBot => "909ddbfd12d5ad92fd213b769753a4b99778b2d36e0a0b5699b72041465299f2".to_string(),
        VitaBotPublicKeys::VitaBotBeta => "e4a0eb5024ad8328d334e933fb68f27517df57579f940ef37c893d1a26838219".to_string()
    }
}