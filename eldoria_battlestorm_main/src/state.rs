use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct RandomNumber{
    pub random_number:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GameState{
    pub is_initialized:u8,
    pub initializer:[u8;32],
    pub guest:[u8;32],
    pub initializers_character_1:Character,
    pub initializers_character_2:Character,
    pub initializers_character_3:Character,
    pub initializers_character_4:Character,
    pub initializers_character_5:Character,
    pub guests_character_1:Character,
    pub guests_character_2:Character,
    pub guests_character_3:Character,
    pub guests_character_4:Character,
    pub guests_character_5:Character,
    pub last_play_time:u64,
    pub whose_turn:u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone,Copy, PartialEq)]
pub struct Character{
    pub mint:[u8;32],
    pub health:u16,
    pub hit:u16,
    pub xp:u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone,Copy, PartialEq)]
pub struct Attack{
    pub attack_to:u8,
    pub attacker:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone,Copy)]
pub struct PlayerAccount{
    pub address:[u8;32],
    pub victories:u8,
}