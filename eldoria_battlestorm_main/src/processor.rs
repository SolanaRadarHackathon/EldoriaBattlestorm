use crate::instruction::RNGProgramInstruction;
use crate::state::{Attack, Character, GameState, PlayerAccount, RandomNumber};
use crate::error::RNGProgramError::ArithmeticError;

use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::accounts::BaseAssetV1;
use mpl_core::fetch_plugin;
use solana_program::clock::Clock;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::{get_return_data, invoke_signed};


use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,program::invoke,msg,
  system_instruction,sysvar::Sysvar,
  rent::Rent
};


use mpl_core::{
  types::PluginType,
  instructions::{UpdatePluginV1Builder,BurnV1Builder},
  types::{Attribute, Attributes, Plugin},
  programs::MPL_CORE_ID
};

pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: RNGProgramInstruction = RNGProgramInstruction::unpack(instruction_data)?;

    match instruction {

      RNGProgramInstruction::InitializeGame => {
        Self::initialize_game(accounts,program_id)
      }
      RNGProgramInstruction::JoinGame => {
        Self::join_game(accounts,program_id)
      }
      RNGProgramInstruction::Play {data} => {
        Self::play_game(accounts,program_id,data)
      }
      RNGProgramInstruction::ClaimVic => {
        Self::claim_victory(accounts,program_id)
      }
      RNGProgramInstruction::ClaimVicTime => {
        Self::claim_victory_over_time(accounts,program_id)
      }
      RNGProgramInstruction::Upgrage {data}=> {
        Self::upgrade_character(accounts,data)
      }
      RNGProgramInstruction::Register => {
        Self::register_player(accounts,program_id)
      }
    }
  }

 pub fn initialize_game(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
 ) -> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
  let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_4: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_5: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let game_state_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !player.is_signer{panic!()}

  Self::check_owner_and_program(player.key,nft_1)?;
  Self::check_owner_and_program(player.key,nft_2)?;
  Self::check_owner_and_program(player.key,nft_3)?;
  Self::check_owner_and_program(player.key,nft_4)?;
  Self::check_owner_and_program(player.key,nft_5)?;

  let character_1 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_1, PluginType::Attributes).unwrap();
  let character_2 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_2, PluginType::Attributes).unwrap();
  let character_3 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_3, PluginType::Attributes).unwrap();
  let character_4 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_4, PluginType::Attributes).unwrap();
  let character_5 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_5, PluginType::Attributes).unwrap();

  let attributes_array = [character_1,character_2,  character_3,  character_4,  character_5];

  let mut characters: Vec<Character> = Vec::new();

  for attributes in  attributes_array {

    let xp:u16 = attributes.1.attribute_list[0].value.parse().unwrap();
    let health:u16 = attributes.1.attribute_list[1].value.parse().unwrap();
    let hit:u16 = attributes.1.attribute_list[2].value.parse().unwrap();

    let character = Character{ 
      mint: nft_1.key.to_bytes(), 
      xp,
      health, 
      hit };

      
    characters.push(character)
  }

  let game_state: GameState = GameState{
    is_initialized:1,
    initializer: player.key.to_bytes(),
    guest: [0;32],
    initializers_character_1: characters[0],
    initializers_character_2: characters[1],
    initializers_character_3: characters[2],
    initializers_character_4: characters[3],
    initializers_character_5: characters[4],
    last_play_time: 0,
    guests_character_1: characters[0],
    guests_character_2: characters[1],
    guests_character_3: characters[2],
    guests_character_4: characters[3],
    guests_character_5: characters[4],
    whose_turn: 0,
  };

  let rent: Rent = Rent::default();
  let rent_amount: u64 = rent.minimum_balance(264);

  let ix = &system_instruction::create_account(  
    &player.key, 
    &game_state_account.key,
    rent_amount,
    264,
    program_id
);

  invoke(ix,  &[player.clone(),game_state_account.clone()])?;

  game_state.serialize(&mut &mut game_state_account.data.borrow_mut()[..])?;

  Ok(())
 }

 pub fn join_game(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
 )-> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
  let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_4: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let nft_5: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let game_state_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !player.is_signer{panic!()}
  if game_state_account.owner != program_id{panic!()}

  let mut game_state: GameState = GameState::try_from_slice(&game_state_account.data.borrow())?;

  if game_state.is_initialized != 1 {panic!()}

  Self::check_owner_and_program(player.key,nft_1)?;
  Self::check_owner_and_program(player.key,nft_2)?;
  Self::check_owner_and_program(player.key,nft_3)?;
  Self::check_owner_and_program(player.key,nft_4)?;
  Self::check_owner_and_program(player.key,nft_5)?;

  let character_1 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_1, PluginType::Attributes).unwrap();
  let character_2 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_2, PluginType::Attributes).unwrap();
  let character_3 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_3, PluginType::Attributes).unwrap();
  let character_4 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_4, PluginType::Attributes).unwrap();
  let character_5 = fetch_plugin::<BaseAssetV1, Attributes>(&nft_5, PluginType::Attributes).unwrap();

  let attributes_array = [character_1,character_2,  character_3,  character_4,  character_5];

  let mut characters: Vec<Character> = Vec::new();

  for attributes in  attributes_array {

    let xp:u16 = attributes.1.attribute_list[0].value.parse().unwrap();
    let health:u16 = attributes.1.attribute_list[1].value.parse().unwrap();
    let hit:u16 = attributes.1.attribute_list[2].value.parse().unwrap();

    let character = Character{ 
      mint: nft_1.key.to_bytes(), 
      xp,
      health, 
      hit };
      
    characters.push(character)
  }

  let clock: Clock = Clock::get().unwrap();
  let current_time = clock.unix_timestamp as u64;

  game_state.is_initialized = 2;
  game_state.last_play_time = current_time;
  game_state.guests_character_1= characters[0];
  game_state.guests_character_2= characters[1];
  game_state.guests_character_3= characters[2];
  game_state.guests_character_4= characters[3];
  game_state.guests_character_5= characters[4];
  game_state.whose_turn = 1;

  game_state.serialize(&mut &mut game_state_account.data.borrow_mut()[..])?;

  Ok(())
 }

 pub fn play_game(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
  attack:Attack
 )-> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
  let player_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let opponents_nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let players_nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let game_state_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let burn_authority_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let update_authority_pda: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let mpl_core: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let noop: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !player_account.is_signer{panic!()}
  if game_state_account.owner != program_id{panic!()}



  let mut game_state: GameState = GameState::try_from_slice(&game_state_account.data.borrow())?;

  let initializer = Pubkey::new_from_array(game_state.initializer);
  let guest = Pubkey::new_from_array(game_state.guest);



  let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let price_feed_account_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let price_feed_account_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let price_feed_account_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let fallback_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let current_feed_accounts: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let rng_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let system_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;



  //Creating account metas for CPI to RNG_PROGRAM
  let player_meta = AccountMeta{ pubkey: *player.key, is_signer: true, is_writable: true,};
  let price_feed_account_1_meta = AccountMeta{ pubkey: *price_feed_account_1.key, is_signer: false, is_writable: false,};
  let price_feed_account_2_meta = AccountMeta{ pubkey: *price_feed_account_2.key, is_signer: false, is_writable: false,};
  let price_feed_account_3_meta = AccountMeta{ pubkey: *price_feed_account_3.key, is_signer: false, is_writable: false,};
  let fallback_account_meta = AccountMeta{ pubkey: *fallback_account.key, is_signer: false, is_writable: false,};
  let current_feed_accounts_meta = AccountMeta{ pubkey: *current_feed_accounts.key, is_signer: false, is_writable: true,};
  let temp_meta = AccountMeta{ pubkey: *temp.key, is_signer: true, is_writable: true,};
  let system_program_meta = AccountMeta{ pubkey: *system_program.key, is_signer: false, is_writable: false,};


  //Creating instruction to cpi RNG PROGRAM
  let ix:Instruction = Instruction { program_id: *rng_program.key,
     accounts: [
      player_meta,
      price_feed_account_1_meta,
      price_feed_account_2_meta,
      price_feed_account_3_meta,
      fallback_account_meta,
      current_feed_accounts_meta,
      temp_meta,
      system_program_meta,
     ].to_vec(), data: [0].to_vec() };

  //CPI to RNG_PROGRAM
  invoke(&ix, 
    &[
      player.clone(),
      price_feed_account_1.clone(),
      price_feed_account_2.clone(),
      price_feed_account_3.clone(),
      fallback_account.clone(),
      current_feed_accounts.clone(),
      temp.clone(),
      system_program.clone()
      ])?;

      let returned_data:(Pubkey, Vec<u8>)= get_return_data().unwrap();
      
    //Random number is returned from the RNG_PROGRAM
    let random_number:RandomNumber;
    if &returned_data.0 == rng_program.key{
      random_number = RandomNumber::try_from_slice(&returned_data.1)?;
      msg!("{}",random_number.random_number);
    }else{
        panic!();
    }

  
    let player_no:u8;
    let opponents_nft_from_game_state:Pubkey;
    let opponent:Pubkey;
     //1 initalizer
     //2 guest
  
    let mut enemy:Character;
    let mut attacker:Character;

    let random_hit = (random_number.random_number % 10) as u16;

    let total_hit:u16;
  
  
    if &initializer == player_account.key{
  
      opponent = guest;
      player_no = 1;
  
      if attack.attack_to == 1 {
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.guests_character_1.mint);
        enemy = game_state.guests_character_1;
      }else if attack.attack_to == 2{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.guests_character_2.mint);
        enemy = game_state.guests_character_2;
      }else if attack.attack_to == 3{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.guests_character_3.mint);
        enemy = game_state.guests_character_3;
      }else if attack.attack_to == 4{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.guests_character_4.mint);
        enemy = game_state.guests_character_4;
      }else if attack.attack_to == 5{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.guests_character_5.mint);
        enemy = game_state.guests_character_5;
      }else{
        panic!()
      }
  
      if  attack.attacker == 1{
        attacker = game_state.initializers_character_1;
    }else if  attack.attacker == 2{
      attacker = game_state.initializers_character_2;
        
    }else if  attack.attacker == 3{
      attacker = game_state.initializers_character_3;
        
    }else if  attack.attacker == 4{
      attacker = game_state.initializers_character_4;
        
    }else if  attack.attacker == 5{
      attacker = game_state.initializers_character_5;
        
    }else{
      panic!()
    }
  
    }else if &guest == player_account.key {
  
      opponent = initializer;
      player_no = 2;
  
      if attack.attack_to == 1 {
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.initializers_character_1.mint);
        enemy = game_state.initializers_character_1;
      }else if attack.attack_to == 2{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.initializers_character_2.mint);
        enemy = game_state.initializers_character_2;
      }else if attack.attack_to == 3{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.initializers_character_3.mint);
        enemy = game_state.initializers_character_3;
      }else if attack.attack_to == 4{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.initializers_character_4.mint);
        enemy = game_state.initializers_character_4;
      }else if attack.attack_to == 5{
        opponents_nft_from_game_state= Pubkey::new_from_array(game_state.initializers_character_5.mint);
        enemy = game_state.initializers_character_5;
      }else{
        panic!()
      }
  
      if  attack.attacker == 1{
        attacker = game_state.guests_character_1;
    }else if  attack.attacker == 2{
      attacker = game_state.guests_character_2;
        
    }else if  attack.attacker == 3{
      attacker = game_state.guests_character_3;
        
    }else if  attack.attacker == 4{
      attacker = game_state.guests_character_4;
        
    }else if  attack.attacker == 5{
      attacker = game_state.guests_character_5;
        
    }else{
      panic!()
    }
  
  
  
    }else{panic!()}
  
    if attacker.health == 0 {panic!()}
  
    if game_state.is_initialized != 1 {panic!()}
  
    if &opponents_nft_from_game_state != opponents_nft.key {panic!()}
  
  
    Self::check_owner_and_program(&opponent,opponents_nft)?;
  
    if random_hit == 0{
      total_hit = attacker.hit.checked_add(random_hit).ok_or(ArithmeticError)?;
    }else{
      total_hit = attacker.hit.checked_mul(3).ok_or(ArithmeticError)?;
    }
  
    if enemy.health <= total_hit{
      enemy.health = 0;
      attacker.xp = attacker.xp.checked_add(100).ok_or(ArithmeticError)?;
      Self::burn_nft(burn_authority_pda,opponents_nft,mpl_core)?;
      Self::add_xp(update_authority_pda,players_nft,mpl_core,noop,player,system_program,attacker)?;

    }else{
      enemy.health.checked_sub(total_hit).ok_or(ArithmeticError)?;
      attacker.xp = attacker.xp.checked_add(20).ok_or(ArithmeticError)?;
      Self::add_xp(update_authority_pda,players_nft,mpl_core,noop,player,system_program,attacker)?;
    }


    if &initializer == player_account.key{
  
  
      if attack.attack_to == 1 {
        game_state.guests_character_1 = enemy;
      }else if attack.attack_to == 2{
        game_state.guests_character_2 = enemy;
      }else if attack.attack_to == 3{
        game_state.guests_character_3 = enemy;
      }else if attack.attack_to == 4{
        game_state.guests_character_4 = enemy;
      }else if attack.attack_to == 5{
        game_state.guests_character_5 = enemy;
      }else{
        panic!()
      }
  
      if  attack.attacker == 1{
        game_state.initializers_character_1 = attacker;
    }else if  attack.attacker == 2{
      game_state.initializers_character_2 = attacker;
        
    }else if  attack.attacker == 3{
      game_state.initializers_character_3 = attacker
        
    }else if  attack.attacker == 4{
      game_state.initializers_character_4 = attacker;
        
    }else if  attack.attacker == 5{
      game_state.initializers_character_5 = attacker;
        
    }else{
      panic!()
    }
  
    }else {
  
  
      if attack.attack_to == 1 {
        game_state.initializers_character_1 = enemy;
      }else if attack.attack_to == 2{
        game_state.initializers_character_2 = enemy;
      }else if attack.attack_to == 3{
         game_state.initializers_character_3 = enemy;
      }else if attack.attack_to == 4{
         game_state.initializers_character_4 = enemy;
      }else {
         game_state.initializers_character_5 = enemy;
      }
  
      if  attack.attacker == 1{
         game_state.guests_character_1 = attacker;
    }else if  attack.attacker == 2{
       game_state.guests_character_2 = attacker;
        
    }else if  attack.attacker == 3{
       game_state.guests_character_3 = attacker;
        
    }else if  attack.attacker == 4{
       game_state.guests_character_4 = attacker;
        
    }else {
       game_state.guests_character_5 = attacker;
        
    }
  
    }
  
  

  let clock: Clock = Clock::get().unwrap();
  let current_time: u64 = clock.unix_timestamp as u64;

  game_state.is_initialized = 2;
  game_state.last_play_time = current_time;

  if player_no == 1{
    game_state.whose_turn =2;
  }else{
    game_state.whose_turn = 1;
  }


  game_state.serialize(&mut &mut game_state_account.data.borrow_mut()[..])?;

  Ok(())
 }

 pub fn claim_victory(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
 ) -> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


  let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let initializer_player_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let guest_player_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let game_state_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if game_state_account.owner != program_id{panic!()}


  let game_state: GameState = GameState::try_from_slice(&game_state_account.data.borrow())?;
  let mut initializer_player_record: PlayerAccount = PlayerAccount::try_from_slice(&initializer_player_account.data.borrow())?;
  let mut guest_player_record: PlayerAccount = PlayerAccount::try_from_slice(&guest_player_account.data.borrow())?;

  let initializer_from_bytes: Pubkey = Pubkey::new_from_array(game_state.initializer);
  let guest_from_bytes: Pubkey = Pubkey::new_from_array(game_state.guest);

  let initializer_from_player_account: Pubkey = Pubkey::new_from_array(initializer_player_record.address);
  let guest_from_player_account: Pubkey = Pubkey::new_from_array(guest_player_record.address);

  if &initializer_from_bytes != initializer.key {panic!()}
  if &guest_from_bytes != guest.key {panic!()}

  if initializer_from_bytes != initializer_from_player_account {panic!()}
  if guest_from_bytes != guest_from_player_account {panic!()}

  if game_state.guests_character_1.health == 0
  && game_state.guests_character_2.health == 0
  && game_state.guests_character_3.health == 0
  && game_state.guests_character_4.health == 0
  && game_state.guests_character_5.health == 0
  {
    guest_player_record.victories += 1;

  }else if  game_state.initializers_character_1.health == 0
  && game_state.initializers_character_2.health == 0
  && game_state.initializers_character_3.health == 0
  && game_state.initializers_character_4.health == 0
  && game_state.initializers_character_5.health == 0
  {
    initializer_player_record.victories += 1;
  }else{
    panic!()
  }

  let v: u64 = **game_state_account.try_borrow_lamports()?;

  **game_state_account.try_borrow_mut_lamports()? -= v;
  **initializer.try_borrow_mut_lamports()? += v;

  initializer_player_record.serialize(&mut &mut initializer_player_account.data.borrow_mut()[..])?;
  guest_player_record.serialize(&mut &mut guest_player_account.data.borrow_mut()[..])?;


  Ok(())
 }
 
 pub fn claim_victory_over_time(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
 ) -> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


  let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let initializer_player_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let guest_player_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let game_state_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if game_state_account.owner != program_id{panic!()}


  let game_state: GameState = GameState::try_from_slice(&game_state_account.data.borrow())?;
  let mut initializer_player_record: PlayerAccount = PlayerAccount::try_from_slice(&initializer_player_account.data.borrow())?;
  let mut guest_player_record: PlayerAccount = PlayerAccount::try_from_slice(&guest_player_account.data.borrow())?;

  let initializer_from_bytes: Pubkey = Pubkey::new_from_array(game_state.initializer);
  let guest_from_bytes: Pubkey = Pubkey::new_from_array(game_state.guest);

  let initializer_from_player_account: Pubkey = Pubkey::new_from_array(initializer_player_record.address);
  let guest_from_player_account: Pubkey = Pubkey::new_from_array(guest_player_record.address);

  if &initializer_from_bytes != initializer.key {panic!()}
  if &guest_from_bytes != guest.key {panic!()}

  if initializer_from_bytes != initializer_from_player_account {panic!()}
  if guest_from_bytes != guest_from_player_account {panic!()}

  let clock: Clock = Clock::get()?;
  let current_time: u64 = clock.unix_timestamp as u64;

  let time_left: u64 = game_state.last_play_time - current_time;

  if time_left < 120 {panic!()}

  if game_state.whose_turn == 1{
    initializer_player_record.victories += 1;

  }else{
    guest_player_record.victories += 1;

  }


  initializer_player_record.serialize(&mut &mut initializer_player_account.data.borrow_mut()[..])?;
  guest_player_record.serialize(&mut &mut guest_player_account.data.borrow_mut()[..])?;


  Ok(())
 }
 
 pub fn  upgrade_character(
  accounts: &[AccountInfo],
  upgrade:Character
) -> ProgramResult {

let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
let payer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
let nft: &AccountInfo<'_> = next_account_info(accounts_iter)?;
let mpl_core: &AccountInfo<'_> = next_account_info(accounts_iter)?;
let system_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
let noop: &AccountInfo<'_> = next_account_info(accounts_iter)?;

let attributes = fetch_plugin::<BaseAssetV1, Attributes>(&nft, PluginType::Attributes).unwrap();

let xp:u16 = attributes.1.attribute_list[0].value.parse().unwrap();
let health:u16 = attributes.1.attribute_list[1].value.parse().unwrap();
let hit:u16 = attributes.1.attribute_list[2].value.parse().unwrap();

let upgrade_1: u16 = upgrade.health.checked_sub(health).ok_or(ArithmeticError)?; 
let upgrade_2: u16 = upgrade.hit.checked_sub(hit).ok_or(ArithmeticError)?; 

let total_upgrade: u16 = upgrade_1.checked_add(upgrade_2).ok_or(ArithmeticError)?;

if total_upgrade > xp {panic!()}

let left_xp = xp.checked_sub(total_upgrade).ok_or(ArithmeticError)?;

let update_attributes_plugin_ix: Instruction = UpdatePluginV1Builder::new()
    .asset(*nft.key)
    .payer(*payer.key)
    .authority(Some(*authority.key))
    .plugin(Plugin::Attributes(Attributes {
        attribute_list: vec![
          Attribute {
            key: "health".to_string(),
            value: health.to_string(),
        },
        Attribute {
            key: "hit".to_string(),
            value: hit.to_string(),
        },
        Attribute {
            key: "xp".to_string(),
            value: left_xp.to_string(),
        },
        ],
    }))
    .instruction();

  invoke_signed(&update_attributes_plugin_ix,
    &[nft.clone(),authority.clone(),mpl_core.clone(),payer.clone(),system_program.clone(),noop.clone()],
  &[&["update".to_string().as_bytes(), &[253]]])?;



  Ok(())
}

pub fn register_player(
  accounts: &[AccountInfo],
  program_id:&Pubkey,
 ) -> ProgramResult{

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
  let player: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let player_record_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !player.is_signer{panic!()}


  let rent: Rent = Rent::default();
  let rent_amount: u64 = rent.minimum_balance(34);

  let ix = &system_instruction::create_account(  
    &player.key, 
    &player_record_account.key,
    rent_amount,
    34,
    program_id
);

  invoke(ix,  &[player.clone(),player_record_account.clone()])?;

  let player_record = PlayerAccount{
    address: player.key.to_bytes(),
    victories: 0,
  };

player_record.serialize(&mut &mut player_record_account.data.borrow_mut()[..])?;

  Ok(())
 }


    fn burn_nft<'info>(
      authority_pda: &AccountInfo<'info>,
      nft: &AccountInfo<'info>,
      mpl_core: &AccountInfo<'info>,
    ) -> ProgramResult {


      let burn_asset_ix = BurnV1Builder::new()
      .asset(*nft.key)
      .payer(*authority_pda.key)
      .instruction();


    invoke_signed(&burn_asset_ix, &[nft.clone(),authority_pda.clone(),mpl_core.clone()],
    &[&["burn".to_string().as_bytes(), &[254]]])?;
      
      Ok(())
    }

    fn check_owner_and_program(
      initializer:&Pubkey,
      nft:&AccountInfo,

    ) -> ProgramResult {

      let asset_v1 = BaseAssetV1::from_bytes(&nft.data.borrow()).unwrap();

      let owner = asset_v1.owner;
      
      if &owner != initializer {panic!()}

      if nft.owner != &MPL_CORE_ID {panic!()}
      

      Ok(())
  }


  fn add_xp<'info>(
    update_authority_pda: &AccountInfo<'info>,
    nft: &AccountInfo<'info>,
    mpl_core: &AccountInfo<'info>,
    noop: &AccountInfo<'info>,
    player: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    attacker:Character
  ) -> ProgramResult {


    let update_attributes_plugin_ix = UpdatePluginV1Builder::new()
        .asset(*nft.key)
        .payer(*player.key)
        .authority(Some(*update_authority_pda.key))
        .plugin(Plugin::Attributes(Attributes {
            attribute_list: vec![
                Attribute {
                    key: "health".to_string(),
                    value: attacker.health.to_string(),
                },
                Attribute {
                    key: "hit".to_string(),
                    value: attacker.hit.to_string(),
                },
                Attribute {
                    key: "xp".to_string(),
                    value: attacker.xp.to_string(),
                },
            ],
        }))
        .instruction();

      invoke_signed(&update_attributes_plugin_ix,
        &[nft.clone(),update_authority_pda.clone(),mpl_core.clone(),player.clone(),system_program.clone(),noop.clone()],
      &[&["update".to_string().as_bytes(), &[253]]])?;
    
    Ok(())
  }



}
