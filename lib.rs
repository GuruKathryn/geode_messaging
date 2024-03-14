/*
ABOUT THIS CONTRACT...
This contract allows users to privately message each other the Geode Blockchain. 
- Send messages privately to other accounts,
- Only those you choose can message you,
- Others can pay to get into your PAID inbox,
- powerful group and list messaging features
- maximum user control! 
*/ 

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod geode_messaging {

    use ink::prelude::vec::Vec;
    use ink::prelude::vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
    use ink::env::hash::{Sha2x256, HashOutput};

    // PRELIMINARY STORAGE STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Settings {
        user_account: AccountId,
        username: Vec<u8>,
        interests: Vec<u8>,
        inbox_fee: Balance,
        hide_from_search: bool,
        last_update: u64,
    }

    impl Default for Settings {
        fn default() -> Settings {
            Settings {
                user_account: AccountId::from([0x0; 32]),
                username: <Vec<u8>>::default(),
                interests: <Vec<u8>>::default(),
                inbox_fee: Balance::default(),
                hide_from_search: false,
                last_update: u64::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct SettingsData {
        interests: Vec<Vec<u8>>,
        inbox_fee: Vec<Balance>,
        last_update: Vec<u64>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct MessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        from_username: Vec<u8>,
        to_acct: AccountId,
        message: Vec<u8>, 
        file_url: Vec<u8>,
        timestamp: u64,
    }

    impl Default for MessageDetails {
        fn default() -> MessageDetails {
            MessageDetails {
                message_id: Hash::default(),
                from_acct: AccountId::from([0x0; 32]),
                from_username: <Vec<u8>>::default(),
                to_acct: AccountId::from([0x0; 32]),
                message: <Vec<u8>>::default(),
                file_url: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ListMessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        username: Vec<u8>,
        to_list_id: Hash,
        to_list_name: Vec<u8>,
        message: Vec<u8>,
        file_url: Vec<u8>,
        timestamp: u64,
    }

    impl Default for ListMessageDetails {
        fn default() -> ListMessageDetails {
            ListMessageDetails {
                message_id: Hash::default(),
                from_acct: AccountId::from([0x0; 32]),
                username: <Vec<u8>>::default(),
                to_list_id: Hash::default(),
                to_list_name: <Vec<u8>>::default(),
                message: <Vec<u8>>::default(), 
                file_url: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct PaidMessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        from_username: Vec<u8>,
        to_acct: AccountId,
        message: Vec<u8>, 
        file_url: Vec<u8>,
        timestamp: u64,
        bid: Balance,
    }

    impl Default for PaidMessageDetails {
        fn default() -> PaidMessageDetails {
            PaidMessageDetails {
                message_id: Hash::default(),
                from_acct: AccountId::from([0x0; 32]),
                from_username: <Vec<u8>>::default(),
                to_acct: AccountId::from([0x0; 32]),
                message: <Vec<u8>>::default(),
                file_url: <Vec<u8>>::default(),
                timestamp: u64::default(),
                bid: Balance::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct AccountVector {
        accountvector: Vec<AccountId>,
    }

    impl Default for AccountVector {
        fn default() -> AccountVector {
            AccountVector {
              accountvector: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct HashVector {
        hashvector: Vec<Hash>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct GroupDetails {
        group_id: Hash,
        group_name: Vec<u8>,
        hide_from_search: bool,
        description: Vec<u8>,
        group_accounts: Vec<AccountId>,
        subscribers: u128,
    }

    impl Default for GroupDetails {
        fn default() -> GroupDetails {
            GroupDetails {
                group_id: Hash::default(),
                group_name: <Vec<u8>>::default(),
                hide_from_search: false,
                description: <Vec<u8>>::default(),
                group_accounts: <Vec<AccountId>>::default(),
                subscribers: u128::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct GroupPublicDetails {
        group_id: Hash,
        group_name: Vec<u8>,
        description: Vec<u8>,
        subscribers: u128,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct OpenListDetails {
        list_id: Hash,
        owner: AccountId,
        list_name: Vec<u8>,
        hide_from_search: bool,
        description: Vec<u8>,
        list_accounts: u128,
    }

    impl Default for OpenListDetails {
        fn default() -> OpenListDetails {
            OpenListDetails {
                list_id: Hash::default(),
                owner: AccountId::from([0x0; 32]),
                list_name: <Vec<u8>>::default(),
                hide_from_search: false,
                description: <Vec<u8>>::default(),
                list_accounts: u128::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct OpenListPublicDetails {
        list_id: Hash,
        owner: AccountId,
        list_name: Vec<u8>,
        description: Vec<u8>,
        list_accounts: u128,
    }

    impl Default for OpenListPublicDetails {
        fn default() -> OpenListPublicDetails {
            OpenListPublicDetails {
                list_id: Hash::default(),
                owner: AccountId::from([0x0; 32]),
                list_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                list_accounts: u128::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ConversationWithAccount {
        allowed_account: AccountId,
        username: Vec<u8>,
        conversation: Vec<MessageDetails>
    }

    impl Default for ConversationWithAccount {
        fn default() -> ConversationWithAccount {
            ConversationWithAccount {
                allowed_account: AccountId::from([0x0; 32]),
                username: <Vec<u8>>::default(),
                conversation: <Vec<MessageDetails>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct MessagesFromList {
        allowed_list: Hash,
        list_name: Vec<u8>,
        list_messages: Vec<ListMessageDetails>
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct MyInboxLists {
        lists: Vec<MessagesFromList>,
        defunct_lists: Vec<Hash>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct GroupSearchResults {
        search: Vec<Vec<u8>>,
        groups: Vec<GroupPublicDetails>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ListSearchResults {
        search: Vec<Vec<u8>>,
        lists: Vec<OpenListPublicDetails>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct AccountsAllowedAndBlocked {
        allowed_accounts: Vec<AccountId>,
        blocked_accounts: Vec<AccountId>,
    }

    impl Default for AccountsAllowedAndBlocked {
        fn default() -> AccountsAllowedAndBlocked {
            AccountsAllowedAndBlocked {
                allowed_accounts: <Vec<AccountId>>::default(),
                blocked_accounts: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct RewardSettings {
        reward_on: u8,
        reward_root_set: u8,
        reward_root: AccountId,
        reward_interval: u128,
        reward_amount: Balance,
        reward_balance: Balance,
        reward_payouts: Balance,
        claim_counter: u128,
    }

    impl Default for RewardSettings {
        fn default() -> RewardSettings {
            RewardSettings {
                reward_on: u8::default(),
                reward_root_set: u8::default(),
                reward_root: AccountId::from([0x0; 32]),
                reward_interval: u128::default(),
                reward_amount: Balance::default(),
                reward_balance: Balance::default(),
                reward_payouts: Balance::default(),
                claim_counter: u128::default(),
            }
        }
    }


    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    
    #[ink(event)]
    // when a user updates their settings
    pub struct SettingsUpdated {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        username: Vec<u8>,
        #[ink(topic)]
        interests: Vec<u8>,
        inbox_fee: Balance,
    }

    #[ink(event)]
    // when a new public group is created
    pub struct NewPublicGroup {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        group_id: Hash,
        #[ink(topic)]
        group_name: Vec<u8>,
        description: Vec<u8>,
    }

    #[ink(event)]
    // when a new public newsletter list is created
    pub struct NewPublicList {
        #[ink(topic)]
        list_id: Hash,
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        list_name: Vec<u8>,
        description: Vec<u8>,
    }

    #[ink(event)]
    // when a new public newsletter list is created
    pub struct ListDeleted {
        #[ink(topic)]
        list_id: Hash,
    }

    #[ink(event)]
    // Writes the new reward to the blockchain 
    pub struct AccountRewardedMessaging {
        #[ink(topic)]
        claimant: AccountId,
        reward: Balance,
    }


    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        // Allowing an account that you allow already
        DuplicateAllow,
        // Disallowing an account that you don't allow anyway
        AlreadyNotAllowed,
        // Blocking an account that you already blocked
        DuplicateBlock,
        // Unblocking an account that you never blocked
        NotBlocked,
        // trying to update your interest before 24 hours have past
        CannotUpdateInterestsWithin24Hours,
        // Too many interests in your list
        InterestsTooLong,
        // trying to message a group or list that does not exist
        NoSuchList,
        // trying to delete a message that does not exist
        MessageNotFound,
        // trying to message an account that has not allowed you
        NotAllowedToMessage,
        // trying to make a group whose name is already taken
        GroupNameTaken,
        // trying to join a group that doesn't exist
        NonexistentGroup,
        // trying to access a list that does not exist
        NonexistentList,
        // creating a duplicate list name
        ListNameTaken,
        // subscribing to a list you already subscribe to
        AlreadySubscribed,
        // sending a paid message without enough to cover inbox fees
        InsufficientStake,
        // if the contract has no money to pay
        ZeroBalance,
        // if the an inbox or data payment fails
        PayoutFailed,
        // Returned if the username already belongs to someone else.
        UsernameTaken,
        // removing an account that was not there
        NonexistentAccount,
        // input data is too large
        DataTooLarge,
        // Cannot follow any more accounts or storage otherwise full
        StorageFull,
        // trying to send a paid message without enough stake or to 
        // someone who has blocked you
        CannotSendPaidMessage,
        // attempting to change reward program settings without permission
        PermissionDenied,
    }


    // ACTUAL CONTRACT STORAGE STRUCT >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[ink(storage)]
    pub struct ContractStorage {
        account_settings: Mapping<AccountId, Settings>,
        account_allowed: Mapping<AccountId, AccountVector>,
        account_blocked_accounts: Mapping<AccountId, AccountVector>,
        account_blocked_lists: Mapping<AccountId, HashVector>,
        account_subscribed_groups: Mapping<AccountId, HashVector>,
        account_owned_open_lists: Mapping<AccountId, HashVector>,
        account_paid_inbox: Mapping<AccountId, HashVector>,
        account_subscribed_lists: Mapping<AccountId, HashVector>,
        sent_messages_to_account: Mapping<(AccountId, AccountId), HashVector>,
        sent_messages_to_list: Mapping<Hash, HashVector>,
        sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>,
        all_messages_to_group: Mapping<Hash, HashVector>,
        message_details: Mapping<Hash, MessageDetails>,
        list_message_details: Mapping<Hash, ListMessageDetails>,
        paid_message_details: Mapping<Hash, PaidMessageDetails>,
        group_message_details: Mapping<Hash, ListMessageDetails>,
        open_list_details: Mapping<Hash, OpenListDetails>,
        group_details: Mapping<Hash, GroupDetails>,
        open_lists: StorageVec<Hash>,
        groups: StorageVec<Hash>,
        username_map: Mapping<Vec<u8>, AccountId>,
        reward_root_set: u8,
        reward_root: AccountId,
        reward_interval: u128,
        reward_amount: Balance,
        reward_on: u8,
        reward_balance: Balance,
        reward_payouts: Balance,
        claim_counter: u128,
    }


    // BEGIN CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                account_settings: Mapping::default(),
                account_allowed: Mapping::default(),
                account_blocked_accounts: Mapping::default(),
                account_blocked_lists: Mapping::default(),
                account_subscribed_groups: Mapping::default(),
                account_owned_open_lists: Mapping::default(),
                account_paid_inbox: Mapping::default(),
                account_subscribed_lists: Mapping::default(),
                sent_messages_to_account: Mapping::default(),
                sent_messages_to_list: Mapping::default(),
                sent_messages_to_group: Mapping::default(),
                all_messages_to_group: Mapping::default(),
                message_details: Mapping::default(),
                list_message_details: Mapping::default(),
                paid_message_details: Mapping::default(),
                group_message_details: Mapping::default(),
                open_list_details: Mapping::default(),
                group_details: Mapping::default(),
                open_lists: StorageVec::default(),
                groups: StorageVec::default(),
                username_map: Mapping::default(),
                reward_root_set: 0,
                reward_root: AccountId::from([0x0; 32]),
                reward_interval: 1000000,
                reward_amount: 0,
                reward_on: 0,
                reward_balance: 0,
                reward_payouts: 0,
                claim_counter: 0,
            }
        }


        // MESSAGE FUNCTIONS THAT CHANGE DATA IN THE CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>

        // 0 🟢 Update Settings 
        // lets a user update their list of keyword interests and other settings 
        // overwrites the mapping in contract storage
        #[ink(message)]
        pub fn update_settings (&mut self, 
            my_username: Vec<u8>,
            my_interests: Vec<u8>,
            my_inbox_fee: Balance,
            hide_from_search: bool
        ) -> Result<(), Error> {
            // if the input data is too bid, send an error
            if my_username.len() > 100 || my_interests.len() > 600 {
                return Err(Error::DataTooLarge);
            }

            let username_clone1 = my_username.clone();
            let username_clone2 = my_username.clone();
            let username_clone3 = my_username.clone();
            let username_clone4 = my_username.clone();
            let interests_clone2 = my_interests.clone();

            // get the current settings for this caller and prepare the update
            let caller = Self::env().caller();
            let current_settings = self.account_settings.get(&caller).unwrap_or_default();
            let settings_update: Settings = Settings {
                user_account: caller,
                username: my_username,
                interests: my_interests,
                inbox_fee: my_inbox_fee,
                hide_from_search: hide_from_search,
                last_update: self.env().block_timestamp()
            };
            
            // check that this user has not updated their settings in 24 hours
            let time_since_last_update = self.env().block_timestamp().saturating_sub(current_settings.last_update);
            if time_since_last_update < 86400000 {
                // send an error that interest cannot be updated so soon
                return Err(Error::CannotUpdateInterestsWithin24Hours)
            }
            else {
                // check that the username is not taken by someone else...
                // if the username is in use already...
                if self.username_map.contains(username_clone1) {
                    // get the account that owns that username
                    let current_owner = self.username_map.get(&username_clone2).unwrap();
                    // if the caller owns that username, update the storage maps
                    if current_owner == caller {
                        if self.account_settings.try_insert(&caller, &settings_update).is_err() {
                            return Err(Error::DataTooLarge);
                        }
                    }
                    else {
                        // if the username belongs to someone else, send an error UsernameTaken
                        return Err(Error::UsernameTaken)
                    }
                }
                else {
                    // if the username is not already in use, update the storage maps
                    // update the account_settings map
                    self.account_settings.insert(&caller, &settings_update);
                    // then update the username map
                    self.username_map.insert(&username_clone3, &caller);
                }

                // Emit an event to register the update to the chain
                // but only if the caller is not hidden
                if hide_from_search == false {
                    Self::env().emit_event(SettingsUpdated {
                        from: caller,
                        username: username_clone4,
                        interests: interests_clone2,
                        inbox_fee: my_inbox_fee,
                    });
                }
            }
            
            Ok(())
        }

        
        // 1 🟢 Send A Private Message 🏆
        #[ink(message)]
        pub fn send_private_message (&mut self, 
            to_acct: AccountId,
            new_message: Vec<u8>, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_message.len() > 600 || file_url.len() > 300 {
                return Err(Error::DataTooLarge);
            }
            // get the list of allowed accounts for to_acct
            let current_allowed = self.account_allowed.get(&to_acct).unwrap_or_default();
            // if caller is in the allowed accounts, proceed
            let caller = Self::env().caller();
            if current_allowed.accountvector.contains(&caller) {
                // set up clones as needed
                let new_message_clone = new_message.clone();

                // set up the data that will go into the new_message_id hash
                let new_timestamp = self.env().block_timestamp();

                // create the new_message_id by hashing the above data
                let encodable = (caller, to_acct, new_message, new_timestamp); // Implements `scale::Encode`
                let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                let new_message_id: Hash = Hash::from(new_message_id_u8);

                // SET UP THE MESSAGE DETAILS FOR THE NEW MESSAGE
                let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                let new_details = MessageDetails {
                    message_id: new_message_id,
                    from_acct: Self::env().caller(),
                    from_username: fromusername,
                    to_acct: to_acct,
                    message: new_message_clone, 
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // get the messages vector for this pair of accounts
                let mut current_messages = self.sent_messages_to_account.get((&caller, &to_acct)).unwrap_or_default();
                // if the vector is full, kick out the oldest message
                if current_messages.hashvector.len() > 2 {
                    // remove the oldest message from message_details
                    let oldest = current_messages.hashvector[0];
                    self.message_details.remove(oldest);
                    // remove the oldest message from sent_messages_to_account
                    current_messages.hashvector.remove(0);
                }

                // update the message_details: Mapping<Hash, MessageDetails>
                if self.message_details.try_insert(&new_message_id, &new_details).is_err() {
                    return Err(Error::DataTooLarge);
                } 

                // update the sent_messages_to_account: Mapping<(AccountId, AccountId), HashVector>
                // add this message to the messages vector for this account
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_account map
                self.sent_messages_to_account.insert((&caller, &to_acct), &current_messages);

                // REWARD PROGRAM ACTIONS... update the claim_counter 
                self.claim_counter = self.claim_counter.wrapping_add(1);
                // IF conditions are met THEN payout a reward
                let min = self.reward_amount.saturating_add(10);
                let payout: Balance = self.reward_amount;
                if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                    // payout
                    if self.env().transfer(caller, payout).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                    // update reward_balance
                    self.reward_balance = self.reward_balance.saturating_sub(payout);
                    // update reward_payouts
                    self.reward_payouts = self.reward_payouts.saturating_add(payout);
                    // emit an event to register the reward to the chain
                    Self::env().emit_event(AccountRewardedMessaging {
                        claimant: caller,
                        reward: payout
                    });
                }
                // END REWARD PROGRAM ACTIONS

                Ok(())
            }
            else {
                // otherwise, if the caller is not allowed to message this account, send an error
                return Err(Error::NotAllowedToMessage)
            }

        }


        // 2 🟢 Send A Message To Group 🏆
        #[ink(message)]
        pub fn send_message_to_group (&mut self, 
            to_group_id: Hash,
            new_message: Vec<u8>, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_message.len() > 600 || file_url.len() > 300 {
                return Err(Error::DataTooLarge);
            }

            // check that the group actually exists
            if self.group_details.contains(&to_group_id) {

                // check that the caller is in the group
                let caller = Self::env().caller();
                let mygroups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
                if mygroups.hashvector.contains(&to_group_id) {

                    // set up clones
                    let new_message_clone = new_message.clone();

                    // set up the data that will go into the new_message_id hash
                    let new_timestamp = self.env().block_timestamp();

                    // create the new_message_id by hashing the above data
                    let encodable = (caller, to_group_id, new_message, new_timestamp); // Implements `scale::Encode`
                    let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                    let new_message_id: Hash = Hash::from(new_message_id_u8);

                    // SET UP THE MESSAGE DETAILS FOR THE NEW MESSAGE
                    let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                    let listname = self.group_details.get(&to_group_id).unwrap_or_default().group_name;
                    let new_details = ListMessageDetails {
                        message_id: new_message_id,
                        from_acct: caller,
                        username: fromusername,
                        to_list_id: to_group_id,
                        to_list_name: listname,
                        message: new_message_clone,
                        file_url: file_url,
                        timestamp: self.env().block_timestamp(),
                    };

                    // update sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>
                    // get the messages vector for this pair of account/group
                    let mut current_messages = self.sent_messages_to_group.get((&caller, &to_group_id)).unwrap_or_default();
                    // if their storage is full, kick out the oldest message
                    if current_messages.hashvector.len() > 24 {
                        let oldest = current_messages.hashvector[0];
                        // remove oldest from all_messages_to_group
                        self.all_messages_to_group.remove(oldest);
                        // remove oldest from group_message_details
                        self.group_message_details.remove(oldest);
                        // remove oldest from sent_messages_to_group
                        current_messages.hashvector.remove(0);
                    }

                    // update all_messages_to_group: Mapping<Hash, HashVector>
                    // get the messages vector for this group
                    let mut current_group_messages = self.all_messages_to_group.get(&to_group_id).unwrap_or_default();
                    // if the group message storage is full, kick out the oldest message
                    if current_group_messages.hashvector.len() > 24 {
                        let oldest = current_group_messages.hashvector[0];
                        let oldest_from = self.group_message_details.get(oldest).unwrap_or_default().from_acct;
                        // remove oldest message in the all_messages_to_group list
                        current_group_messages.hashvector.remove(0);
                        // remove oldest message from message_details
                        self.message_details.remove(oldest);
                        // remove oldest message from sent_messages_to_group 
                        let mut thislist = self.sent_messages_to_group.get((oldest_from, to_group_id)).unwrap_or_default();
                        thislist.hashvector.retain(|value| *value != oldest);
                        self.sent_messages_to_group.insert((oldest_from, to_group_id), &thislist);
                    }

                    // add this message to the messages vector for this account
                    current_messages.hashvector.push(new_message_id);
                    // update the sent_messages_to_group map
                    self.sent_messages_to_group.insert((&caller, &to_group_id), &current_messages);
                    
                    // add this message to the messages vector for this account
                    current_group_messages.hashvector.push(new_message_id);
                    // update the all_messages_to_group map
                    self.all_messages_to_group.insert(&to_group_id, &current_group_messages);

                    // update group_message_details: Mapping<Hash, ListMessageDetails>
                    if self.group_message_details.try_insert(&new_message_id, &new_details).is_err() {
                        return Err(Error::DataTooLarge);
                    }

                    // REWARD PROGRAM ACTIONS... update the claim_counter 
                    self.claim_counter = self.claim_counter.wrapping_add(1);
                    // IF conditions are met THEN payout a reward
                    let min = self.reward_amount.saturating_add(10);
                    let payout: Balance = self.reward_amount;
                    if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                    && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                        // payout
                        if self.env().transfer(caller, payout).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                        // update reward_balance
                        self.reward_balance = self.reward_balance.saturating_sub(payout);
                        // update reward_payouts
                        self.reward_payouts = self.reward_payouts.saturating_add(payout);
                        // emit an event to register the reward to the chain
                        Self::env().emit_event(AccountRewardedMessaging {
                            claimant: caller,
                            reward: payout
                        });
                    }
                    // END REWARD PROGRAM ACTIONS

                }
                else {
                    return Err(Error::NoSuchList)
                }

            }
            else {
                return Err(Error::NoSuchList)
            }
            
            Ok(())

        }


        // 3 🟢 Allow Account
        #[ink(message)]
        pub fn allow_account (&mut self, allow: Vec<AccountId>) -> Result<(), Error> {
            let caller = Self::env().caller();
            let mut current_allowed = self.account_allowed.get(&caller).unwrap_or_default();
            
            // if there is room in the allowed account vector...
            if current_allowed.accountvector.len().saturating_add(allow.len()) < 60 {
                // proceed to add each account
                for acct in allow {
                    // Is this account already allowed? If TRUE, send ERROR
                    if current_allowed.accountvector.contains(&acct) {
                        return Err(Error::DuplicateAllow);
                    }
                    else {
                        // add the new allow to the the vector of accounts caller is allowing
                        current_allowed.accountvector.push(acct);  
                    }
                }
                // Update (overwrite) the account_allowed: Mapping<AccountID, AccountVector> map
                self.account_allowed.insert(&caller, &current_allowed);
            }
            else {
                // error: Account following is full
                return Err(Error::StorageFull)
            }
            
            Ok(())
        }


        // 4 🟢 Disallow Account
        #[ink(message)]
        pub fn disallow_account (&mut self, disallow: AccountId) -> Result<(), Error> {
            // Is this account currently allowed? If TRUE, proceed...
            let caller = Self::env().caller();
            let mut current_allowed = self.account_allowed.get(&caller).unwrap_or_default();
            if current_allowed.accountvector.contains(&disallow) {
                // remove the unwanted account from the the vector of accounts they allow
                // by keeping everyone other than that account
                current_allowed.accountvector.retain(|value| *value != disallow);
                // Update (overwrite) the account_allowed map in the storage
                self.account_allowed.insert(&caller, &current_allowed);
            }
            // If the account is not currently allowed, ERROR: Already Not Allowed
            else {
                return Err(Error::AlreadyNotAllowed);
            }

            Ok(())
        }


        // 5 🟢 Block Account
        #[ink(message)]
        pub fn block_account (&mut self, block: AccountId) -> Result<(), Error> {
            // Is this account already being blocked? If TRUE, send ERROR
            let caller = Self::env().caller();
            let mut current_blocked = self.account_blocked_accounts.get(&caller).unwrap_or_default();
            // check that there is room in storage
            if current_blocked.accountvector.len() < 400 {
                if current_blocked.accountvector.contains(&block) {
                    return Err(Error::DuplicateBlock);
                }
                // Otherwise, update the account_blocked_accounts for this caller
                else {
                    // add the new block to the the vector of accounts caller is blocking
                    current_blocked.accountvector.push(block);
                    // Update (overwrite) the account_blocked_accounts: Mapping<AccountID, AccountVector> map
                    self.account_blocked_accounts.insert(&caller, &current_blocked);
                }
            }
            else {
                return Err(Error::StorageFull);
            }
            
            Ok(())
        }


        // 6 🟢 Unblock Account
        #[ink(message)]
        pub fn unblock_account (&mut self, unblock: AccountId) -> Result<(), Error> {
            // Is this account currently being blocked? If TRUE, proceed...
            let caller = Self::env().caller();
            let mut current_blocked = self.account_blocked_accounts.get(&caller).unwrap_or_default();
            if current_blocked.accountvector.contains(&unblock) {
                // remove the unblock from the the vector of accounts they are blocking
                // by keeping everyone other than that account... 
                current_blocked.accountvector.retain(|value| *value != unblock);
                // Update (overwrite) the account_blocked_accounts map in the storage
                self.account_blocked_accounts.insert(&caller, &current_blocked);
            }
            // If the account is not currently being followed, ERROR: Already Not blocked
            else {
                return Err(Error::NotBlocked);
            }

            Ok(())
        }


        // 7 🟢 Delete A Single Message To An Account
        #[ink(message)]
        pub fn delete_single_message_to_account (&mut self, message_id_to_delete: Hash) -> Result<(), Error> {
            // does this message exist? If it does, proceed
            if self.message_details.contains(&message_id_to_delete) {
                // get the details for this message
                let caller = Self::env().caller();
                let current_details = self.message_details.get(&message_id_to_delete).unwrap_or_default();
                // get the message hash vector between these two accounts
                let to_account = current_details.to_acct;
                let mut conversation = self.sent_messages_to_account.get((&caller, &to_account)).unwrap_or_default();
                // retain all but this message in that vector
                conversation.hashvector.retain(|value| *value != message_id_to_delete);
                // update the sent_messages_to_account mapping
                self.sent_messages_to_account.insert((&caller, &to_account), &conversation);
                // remove this message id from the message_details: Mapping<Hash, MessageDetails> map
                self.message_details.remove(message_id_to_delete); 

                Ok(())
            }
            else {
                return Err(Error::MessageNotFound);
            }
            
        }
        

        // 8 🟢 Delete All Messages Sent To Account
        #[ink(message)]
        pub fn delete_all_messages_to_account (&mut self, 
            delete_all_messages_to: AccountId
        ) -> Result<(), Error> {
            // sent_messages_to_account: Mapping<(AccountId, AccountId), HashVector>
            // message_details: Mapping<Hash, MessageDetails> 
            let caller = Self::env().caller();
            // first, get the vector of messages from the caller to that account
            let conversation = self.sent_messages_to_account.get((&caller, &delete_all_messages_to)).unwrap_or_default();
            // for each message id hash, remove it from the message_details map
            for messageid in conversation.hashvector {
                self.message_details.remove(messageid);
            }
            // then remove the pair of caller/account from the sent_messages_to_account map
            self.sent_messages_to_account.remove((&caller, &delete_all_messages_to));

            Ok(())
        }


        // 9 🟢 Make A Group (public or private) 🏆
        #[ink(message)]
        pub fn make_a_new_group (&mut self, 
            new_group_name: Vec<u8>,
            hide_from_search: bool,
            description: Vec<u8>,
            first_message: Vec<u8>,
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_group_name.len() > 100 || description.len() > 600 || first_message.len() > 600 
            || file_url.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            
            // do you have room in your subscribed groups vector?
            let caller = Self::env().caller();
            let caller_groups = self.account_subscribed_groups.get(caller).unwrap_or_default();
            if caller_groups.hashvector.len() < 15 {
                // proceed...
                // set up any clones needed
                let first_message_clone = first_message.clone();
                let new_group_name_clone = new_group_name.clone();
                let new_group_name_clone2 = new_group_name.clone();
                let new_group_name_clone3 = new_group_name.clone();
                let new_group_description_clone = new_group_name.clone();
                
                // create the new_group_id by hashing the group name
                let encodable = new_group_name; // Implements `scale::Encode`
                let mut new_group_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_group_id_u8);
                let new_group_id: Hash = Hash::from(new_group_id_u8);

                // is the group name already taken?
                if self.group_details.contains(&new_group_id) {
                    // send an error
                    return Err(Error::GroupNameTaken);
                }
                else {
                    // proceed to set up the group
                    // set up the group details and 
                    // make the caller the first subscriber

                    let new_group = GroupDetails {
                        group_id: new_group_id,
                        group_name: new_group_name_clone,
                        hide_from_search: hide_from_search,
                        description: description,
                        group_accounts: vec![caller],
                        subscribers: 1,
                    };

                    // add it to group_details: Mapping<Hash, GroupDetails>
                    self.group_details.insert(&new_group_id, &new_group);

                    // add the group_id to the groups: StorageVec<Hash> in storage
                    self.groups.push(&new_group_id);

                    // add this group hash to the caller's account_subscribed_groups: Mapping<AccountID, HashVector>
                    // get the caller's account_subscribed_groups
                    let mut caller_groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
                    // add this new group to the vector
                    caller_groups.hashvector.push(new_group_id);
                    // update the mapping
                    self.account_subscribed_groups.insert(&caller, &caller_groups);

                    // send the first message to the group... 

                    // create the new_message_id by hashing the right data
                    let new_timestamp = self.env().block_timestamp();
                    let encodable = (caller, new_group_id, first_message, new_timestamp); // Implements `scale::Encode`
                    let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                    let new_message_id: Hash = Hash::from(new_message_id_u8);

                    // SET UP THE MESSAGE DETAILS FOR THE FIRST MESSAGE
                    let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                    let new_details = ListMessageDetails {
                        message_id: new_message_id,
                        from_acct: Self::env().caller(),
                        username: fromusername,
                        to_list_id: new_group_id,
                        to_list_name: new_group_name_clone2,
                        message: first_message_clone,  
                        file_url: file_url,
                        timestamp: self.env().block_timestamp(),
                    };

                    // update group_message_details: Mapping<Hash, ListMessageDetails>
                    if self.group_message_details.try_insert(&new_message_id, &new_details).is_err() {
                        return Err(Error::DataTooLarge);
                    }        

                    // update sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>
                    // get the messages vector for this pair of account/group
                    let caller = Self::env().caller();
                    let mut current_messages = self.sent_messages_to_group.get((&caller, &new_group_id)).unwrap_or_default();
                    // add this message to the messages vector for this account
                    current_messages.hashvector.push(new_message_id);
                    // update the sent_messages_to_group map
                    self.sent_messages_to_group.insert((&caller, &new_group_id), &current_messages);

                    // update all_messages_to_group: Mapping<Hash, HashVector>
                    // get the messages vector for this group
                    let mut current_messages = self.all_messages_to_group.get(&new_group_id).unwrap_or_default();
                    // add this message to the messages vector for this account
                    current_messages.hashvector.push(new_message_id);
                    // update the all_messages_to_group map
                    self.all_messages_to_group.insert(&new_group_id, &current_messages);

                    // Emit an event to register the group to the chain
                    // but only if the group is not hidden
                    if hide_from_search == false {
                        Self::env().emit_event(NewPublicGroup {
                            from: caller,
                            group_id: new_group_id,
                            group_name: new_group_name_clone3,
                            description: new_group_description_clone,
                        });
                    }

                    // REWARD PROGRAM ACTIONS... update the claim_counter 
                    self.claim_counter = self.claim_counter.wrapping_add(1);
                    // IF conditions are met THEN payout a reward
                    let min = self.reward_amount.saturating_add(10);
                    let payout: Balance = self.reward_amount;
                    if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                    && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                        // payout
                        if self.env().transfer(caller, payout).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                        // update reward_balance
                        self.reward_balance = self.reward_balance.saturating_sub(payout);
                        // update reward_payouts
                        self.reward_payouts = self.reward_payouts.saturating_add(payout);
                        // emit an event to register the reward to the chain
                        Self::env().emit_event(AccountRewardedMessaging {
                            claimant: caller,
                            reward: payout
                        });
                    }
                    // END REWARD PROGRAM ACTIONS

                }
            }
            else {
                // send an error
                return Err(Error::StorageFull);
            }

            Ok(())

        }


        // 10 🟢 Join A Group 🏆
        #[ink(message)]
        pub fn join_a_group (&mut self, group_id: Hash) -> Result<(), Error> {
            // does this group exist?
            if self.group_details.contains(&group_id) {
                // get the caller's currently subscribed groups
                let caller = Self::env().caller();
                let mut current_groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
                // is the caller already subscribed?
                if current_groups.hashvector.contains(&group_id) {
                    return Err(Error::AlreadySubscribed);
                }
                else {
                    // check to see if there is room in storage
                    if current_groups.hashvector.len() < 20 {
                        // push this group id onto the account subscribed groups hashvector
                        current_groups.hashvector.push(group_id);
                        // update account_subscribed_groups: Mapping<AccountID, HashVector>
                        self.account_subscribed_groups.insert(&caller, &current_groups);
                        // Get the group details
                        let mut new_group_details = self.group_details.get(&group_id).unwrap_or_default();
                        if new_group_details.group_accounts.len() < 10 {
                            // add this caller to the accounts vector
                            new_group_details.group_accounts.push(caller);
                        }
                        // add one to the subscriber count
                        new_group_details.subscribers = new_group_details.subscribers.saturating_add(1);
                        // update the group details mapping
                        self.group_details.insert(&group_id, &new_group_details);

                        // REWARD PROGRAM ACTIONS... update the claim_counter 
                        self.claim_counter = self.claim_counter.wrapping_add(1);
                        // IF conditions are met THEN payout a reward
                        let min = self.reward_amount.saturating_add(10);
                        let payout: Balance = self.reward_amount;
                        if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                        && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                            // payout
                            if self.env().transfer(caller, payout).is_err() {
                                return Err(Error::PayoutFailed);
                            }
                            // update reward_balance
                            self.reward_balance = self.reward_balance.saturating_sub(payout);
                            // update reward_payouts
                            self.reward_payouts = self.reward_payouts.saturating_add(payout);
                            // emit an event to register the reward to the chain
                            Self::env().emit_event(AccountRewardedMessaging {
                                claimant: caller,
                                reward: payout
                            });
                        }
                        // END REWARD PROGRAM ACTIONS
                    }
                    else {
                        return Err(Error::StorageFull);
                    }
                }
            }
            else {
                // send an error
                return Err(Error::NonexistentGroup);
            }

            Ok(())
            
        }


        // 11 🟢 Delete A Single Group Message
        #[ink(message)]
        pub fn delete_single_message_to_group (&mut self, message_id_to_delete: Hash) -> Result<(), Error> {
            // does this message exist? If it does, proceed
            if self.group_message_details.contains(&message_id_to_delete) {
                // get the details for this message
                let caller = Self::env().caller();
                let current_details = self.group_message_details.get(&message_id_to_delete).unwrap_or_default();

                // is this your message to delete?
                if current_details.from_acct != caller {
                    // error - not yours to delete
                    return Err(Error::MessageNotFound);
                }
                else {
                    //  proceed...
                    // remove this message from sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>
                    // get the message hash vector from the caller to the group
                    let group_id = current_details.to_list_id;
                    let mut all_sent = self.sent_messages_to_group.get((&caller, &group_id)).unwrap_or_default();
                    // retain all but this message in that vector
                    all_sent.hashvector.retain(|value| *value != message_id_to_delete);
                    // update the sent_messages_to_group mapping
                    self.sent_messages_to_group.insert((&caller, &group_id), &all_sent);

                    // remove this message from all_messages_to_group: Mapping<Hash, HashVector>
                    // get all the messages to this group
                    let mut messages = self.all_messages_to_group.get(&group_id).unwrap_or_default();
                    // remove this message id from the hashvector by retaining everything else
                    messages.hashvector.retain(|value| *value != message_id_to_delete);
                    // update the map
                    self.all_messages_to_group.insert(&group_id, &messages);

                    // remove this message id from the group_message_details: Mapping<Hash, ListMessageDetails> map
                    self.group_message_details.remove(message_id_to_delete); 
                }
            }
            else {
                return Err(Error::MessageNotFound);
            }

            Ok(())
        }


        // 12 🟢 Delete All Messages Sent To A Group
        #[ink(message)]
        pub fn delete_all_messages_to_group (&mut self, delete_my_messages_to_group_id: Hash) -> Result<(), Error> {
            // does this group exist? If it does, proceed
            let groupid = delete_my_messages_to_group_id;
            if self.group_details.contains(&groupid) {
                let caller = Self::env().caller();
                // get the list of message IDs from this caller to the groupid
                let all_sent = self.sent_messages_to_group.get((&caller, &groupid)).unwrap_or_default();
                // get all the messages to this group
                let mut allmessagestogroup = self.all_messages_to_group.get(&groupid).unwrap_or_default();
                for message in all_sent.hashvector {
                    // remove each message from all_messages_to_group: Mapping<Hash, HashVector>
                    allmessagestogroup.hashvector.retain(|value| *value != message);
                    // remove each message id from the group_message_details: Mapping<Hash, ListMessageDetails> map
                    self.group_message_details.remove(message); 
                }
                // remove all of their messages from sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>
                self.sent_messages_to_group.remove((&caller, &groupid));

                // update the all_messages_to_group mapping
                self.all_messages_to_group.insert(&groupid, &allmessagestogroup);              
            }
            else {
                return Err(Error::NonexistentGroup);
            }

            Ok(())
        }


        // 13 🟢 Leave A Group
        #[ink(message)]
        pub fn leave_a_group (&mut self, group_id_to_leave: Hash) -> Result<(), Error> {
            // get the vector of subscribed groups for this caller
            let caller = Self::env().caller();
            let mut current_groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
            // is this caller currently subscribed to this group? if so, proceed
            if current_groups.hashvector.contains(&group_id_to_leave) {
                // remove this group id from account_subscribed_groups: Mapping<AccountID, HashVector>
                current_groups.hashvector.retain(|value| *value != group_id_to_leave);
                // update the mapping
                self.account_subscribed_groups.insert(&caller, &current_groups);
                // remove this caller from group_details: Mapping<Hash, GroupDetails>
                // get the group details for this group id
                let mut current_details = self.group_details.get(&group_id_to_leave).unwrap_or_default();
                // remove the caller from the group_accounts vector
                current_details.group_accounts.retain(|value| *value != caller);
                // subtract one from the subscriber count
                current_details.subscribers = current_details.subscribers.saturating_sub(1);
                // update the group details map
                self.group_details.insert(&group_id_to_leave, &current_details);

                Ok(())
            }
            // if the caller is not currently subscribed to this group, send an error
            else {
                return Err(Error::NonexistentGroup);
            }
            
        }


        // 14 🟢 Update Group Settings
        #[ink(message)]
        pub fn update_group_settings (&mut self, 
            group_id: Hash,
            hide_from_search: bool,
            description: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if description.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            // make sure the group exists
            if self.group_details.contains(group_id) {
                // set up the caller
                let caller = Self::env().caller();
                // get the group details
                let details = self.group_details.get(&group_id).unwrap_or_default();
                // make sure the caller is the group owner (first account in the accounts vector)
                let owner = details.group_accounts[0];
                let name = details.group_name.clone();
                if caller == owner {
                    // set up updated details
                    let update = GroupDetails {
                        group_id: group_id,
                        group_name: details.group_name,
                        hide_from_search: hide_from_search,
                        description: description.clone(),
                        group_accounts: details.group_accounts,
                        subscribers: details.subscribers,
                    };
                    // update the map group_details: Mapping<Hash, GroupDetails>
                    if self.group_details.try_insert(&group_id, &update).is_err() {
                        return Err(Error::DataTooLarge);
                    }

                    // Emit an event to register the group to the chain
                    // but only if the group is not hidden
                    if hide_from_search == false {
                        Self::env().emit_event(NewPublicGroup {
                            from: caller,
                            group_id: group_id,
                            group_name: name,
                            description: description,
                        });
                    }

                }
                else {
                    return Err(Error::NonexistentGroup);
                }
            }
            else {
                return Err(Error::NonexistentGroup);
            }

            Ok(())
        }


        // 15 🟢 Send A Message To List 🏆
        #[ink(message)]
        pub fn send_message_to_list (&mut self, 
            to_list_id: Hash,
            new_message: Vec<u8>, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_message.len() > 600 || file_url.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            // Does this list exist? and do you own it? If so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            if self.open_list_details.contains(&to_list_id) && owned_lists.hashvector.contains(&to_list_id) {
                // set up clones
                let new_message_clone = new_message.clone();

                // set up the data that will go into the new_message_id hash
                let new_timestamp = self.env().block_timestamp();
                // create the new_message_id by hashing the above data
                let encodable = (caller, to_list_id, new_message, new_timestamp); // Implements `scale::Encode`
                let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                let new_message_id: Hash = Hash::from(new_message_id_u8);

                // SET UP THE MESSAGE DETAILS FOR THE NEW MESSAGE
                let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                let listname = self.open_list_details.get(&to_list_id).unwrap_or_default().list_name;
                let new_details = ListMessageDetails {
                    message_id: new_message_id,
                    from_acct: Self::env().caller(),
                    username: fromusername,
                    to_list_id: to_list_id,
                    to_list_name: listname,
                    message: new_message_clone,  
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // update sent_messages_to_list: Mapping<Hash, HashVector>
                // get the messages vector for this list
                let mut current_messages = self.sent_messages_to_list.get(&to_list_id).unwrap_or_default();
                // if the storage is full, remove the oldest message 
                if current_messages.hashvector.len() > 14 {
                    // kick out the oldest message from list_messages_details
                    let oldest = current_messages.hashvector[0];
                    self.list_message_details.remove(oldest);
                    // kick out the oldest message from list_message_details
                    current_messages.hashvector.remove(0);
                }

                // add this message to the messages vector for this list
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_list map
                self.sent_messages_to_list.insert(&to_list_id, &current_messages);

                // update list_message_details: Mapping<Hash, ListMessageDetails>
                if self.list_message_details.try_insert(&new_message_id, &new_details).is_err() {
                    return Err(Error::DataTooLarge);
                }    

                // REWARD PROGRAM ACTIONS... update the claim_counter 
                self.claim_counter = self.claim_counter.wrapping_add(1);
                // IF conditions are met THEN payout a reward
                let min = self.reward_amount.saturating_add(10);
                let payout: Balance = self.reward_amount;
                if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                    // payout
                    if self.env().transfer(caller, payout).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                    // update reward_balance
                    self.reward_balance = self.reward_balance.saturating_sub(payout);
                    // update reward_payouts
                    self.reward_payouts = self.reward_payouts.saturating_add(payout);
                    // emit an event to register the reward to the chain
                    Self::env().emit_event(AccountRewardedMessaging {
                        claimant: caller,
                        reward: payout
                    });
                }
                // END REWARD PROGRAM ACTIONS

            }
            // if the list does not exist, or you do not own it, send an error
            else {
                return Err(Error::NonexistentList);
            }

            Ok(())
        }


        // 16 🟢 Make A New List (public or private) 🏆
        #[ink(message)]
        pub fn make_a_new_list (&mut self, 
            new_list_name: Vec<u8>,
            hide_from_search: bool,
            description: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_list_name.len() > 100 || description.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            // does this caller have room for another open list?
            let caller = Self::env().caller();
            let mut current_owned = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            let mut current_lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            if current_owned.hashvector.len() < 20 && current_lists.hashvector.len() < 20 {
                // proceed...
                // set up clones
                let list_name_clone = new_list_name.clone();
                let list_name_clone2 = new_list_name.clone();
                let list_description_clone = description.clone();

                // hash the list name
                let encodable = new_list_name; // Implements `scale::Encode`
                let mut new_list_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_list_id_u8);
                let new_list_id: Hash = Hash::from(new_list_id_u8);

                // is the list name already taken?
                if self.open_list_details.contains(&new_list_id) {
                    // send an error
                    return Err(Error::ListNameTaken);
                }
                else {
                    // proceed to set up the list details
                    // make the caller the first subscriber
                    let new_list = OpenListDetails {
                        list_id: new_list_id,
                        owner: caller,
                        list_name: list_name_clone,
                        hide_from_search: hide_from_search,
                        description: description,
                        list_accounts: 1,
                    };

                    // add this new list to open_list_details: Mapping<Hash, OpenListDetails>
                    if self.open_list_details.try_insert(&new_list_id, &new_list).is_err() {
                        return Err(Error::DataTooLarge);
                    }        

                    // add this new list to open_lists: Vec<Hash>
                    self.open_lists.push(&new_list_id);

                    // add this new list ID to account_owned_open_lists: Mapping<AccountID, HashVector>
                    current_owned.hashvector.push(new_list_id);
                    self.account_owned_open_lists.insert(&caller, &current_owned);

                    // add this new list ID to account_subscribed_lists: Mapping<AccountID, HashVector>
                    // (subscribe to your own list)
                    current_lists.hashvector.push(new_list_id);
                    self.account_subscribed_lists.insert(&caller, &current_lists); 

                    // Emit an event to register the list to the chain
                    // but only if the list is not hidden
                    if hide_from_search == false {
                        Self::env().emit_event(NewPublicList {
                            list_id: new_list_id,
                            owner: caller,
                            list_name: list_name_clone2,
                            description: list_description_clone,
                        });
                    }     

                    // REWARD PROGRAM ACTIONS... update the claim_counter 
                    self.claim_counter = self.claim_counter.wrapping_add(1);
                    // IF conditions are met THEN payout a reward
                    let min = self.reward_amount.saturating_add(10);
                    let payout: Balance = self.reward_amount;
                    if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                    && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                        // payout
                        if self.env().transfer(caller, payout).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                        // update reward_balance
                        self.reward_balance = self.reward_balance.saturating_sub(payout);
                        // update reward_payouts
                        self.reward_payouts = self.reward_payouts.saturating_add(payout);
                        // emit an event to register the reward to the chain
                        Self::env().emit_event(AccountRewardedMessaging {
                            claimant: caller,
                            reward: payout
                        });
                    }
                    // END REWARD PROGRAM ACTIONS
                }
            }
            else {
                // storage full error
                return Err(Error::StorageFull);
            }

            Ok(())
        }


        // 17 🟢 Delete A Single List Message
        #[ink(message)]
        pub fn delete_single_message_to_list (&mut self, message_id_to_delete: Hash) -> Result<(), Error> {
            // does this message exist? and are you the sender? if so, proceed
            let caller = Self::env().caller();
            // get the details for this message
            let current_details = self.list_message_details.get(&message_id_to_delete).unwrap_or_default();
            if self.list_message_details.contains(&message_id_to_delete) && current_details.from_acct == caller {
                // remove this message from sent_messages_to_list: Mapping<Hash, HashVector>
                // get the message hash vector for this list
                let list_id = current_details.to_list_id;
                let mut all_sent = self.sent_messages_to_list.get(&list_id).unwrap_or_default();
                // retain all but this message in that vector
                all_sent.hashvector.retain(|value| *value != message_id_to_delete);
                // update the sent_messages_to_list mapping
                self.sent_messages_to_list.insert(&list_id, &all_sent);

                // remove this message id from list_message_details: Mapping<Hash, ListMessageDetails>
                self.list_message_details.remove(message_id_to_delete); 

                Ok(())
            }
            else {
                return Err(Error::MessageNotFound);
            }
        }


        // 18 🟢 Delete An Open List (and all of its messages and details) 
        #[ink(message)]
        pub fn delete_an_open_list (&mut self, 
            delete_list_id: Hash
        ) -> Result<(), Error> {
            // does this list exist? and are you the owner? if so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            if self.open_list_details.contains(&delete_list_id) && owned_lists.hashvector.contains(&delete_list_id) {

                // unsubscribe the list owner
                // account_subscribed_lists: Mapping<AccountID, HashVector>
                // get their account_subscribed_lists
                let mut lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
                // remove this list
                lists.hashvector.retain(|value| *value != delete_list_id);
                // update the mapping for that account
                self.account_subscribed_lists.insert(&caller, &lists);

                // delete all the messages sent to this list
                self.sent_messages_to_list.remove(delete_list_id);

                // remove this list from account_owned_open_lists: Mapping<AccountID, HashVector>
                let mut ownedlists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
                // remove this list
                ownedlists.hashvector.retain(|value| *value != delete_list_id);
                // update the mapping for that account
                self.account_owned_open_lists.insert(&caller, &ownedlists);

                // remove the list ID from open_list_details: Mapping<Hash, OpenListDetails>
                self.open_list_details.remove(delete_list_id);

                //// event here to announce the list is deleted
                Self::env().emit_event(ListDeleted {
                    list_id: delete_list_id,
                });

                Ok(())                
            }
            // if the list does not exist, send error
            else {
                return Err(Error::NonexistentList);
            }

        }


        // 19 🟢 Join An Open List 🏆
        #[ink(message)]
        pub fn join_an_open_list (&mut self, list_id: Hash) -> Result<(), Error> {
            // does this list exist? if so, proceed
            if self.open_list_details.contains(&list_id) {
                let caller = Self::env().caller();
                // get the caller's currently subscribed lists
                let mut lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
                // is the caller already subscribed to this list? if so, error
                if lists.hashvector.contains(&list_id) {
                    return Err(Error::AlreadySubscribed);
                }
                // if the caller is not yet subscribed, proceed
                else {
                    // check if there is space in storage to subscribe
                    if lists.hashvector.len() < 20 {
                        
                        // add the list to the caller's account_subscribed_lists: Mapping<AccountID, HashVector>
                        lists.hashvector.push(list_id);
                        self.account_subscribed_lists.insert(&caller, &lists);

                        // add the caller to the list_accounts count in open_list_details: Mapping<Hash, OpenListDetails>
                        let mut details = self.open_list_details.get(&list_id).unwrap_or_default();
                        details.list_accounts = details.list_accounts.saturating_add(1);
                        self.open_list_details.insert(&list_id, &details);

                        // REWARD PROGRAM ACTIONS... update the claim_counter 
                        self.claim_counter = self.claim_counter.wrapping_add(1);
                        // IF conditions are met THEN payout a reward
                        let min = self.reward_amount.saturating_add(10);
                        let payout: Balance = self.reward_amount;
                        if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                        && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                            // payout
                            if self.env().transfer(caller, payout).is_err() {
                                return Err(Error::PayoutFailed);
                            }
                            // update reward_balance
                            self.reward_balance = self.reward_balance.saturating_sub(payout);
                            // update reward_payouts
                            self.reward_payouts = self.reward_payouts.saturating_add(payout);
                            // emit an event to register the reward to the chain
                            Self::env().emit_event(AccountRewardedMessaging {
                                claimant: caller,
                                reward: payout
                            });
                        }
                        // END REWARD PROGRAM ACTIONS
                    }
                    else {
                        // error, no more room
                        return Err(Error::StorageFull);
                    }
                }
            }
            // if the list does not exist, send an error
            else {
                return Err(Error::NonexistentList);
            }

            Ok(())
        }


        // 20 🟢 Unsubscribe From An Open List
        #[ink(message)]
        pub fn unsubscribe_from_open_list (&mut self, 
            list_id: Hash
        ) -> Result<(), Error> {
            // does this list exist? if so, proceed
            if self.open_list_details.contains(&list_id) {
                let caller = Self::env().caller();
                // get the caller's currently subscribed lists
                let mut lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
                // is the caller currently subscribed to this list? if so, proceed
                if lists.hashvector.contains(&list_id) {

                    // remove the list ID from the caller's 
                    // account_subscribed_lists: Mapping<AccountID, HashVector>
                    lists.hashvector.retain(|value| *value != list_id);
                    // update the mapping
                    self.account_subscribed_lists.insert(&caller, &lists);

                    // remove the caller account from list_accounts count 
                    // in open_list_details: Mapping<Hash, OpenListDetails>
                    // get the details for this list
                    let mut details = self.open_list_details.get(&list_id).unwrap_or_default(); 
                    // remove the caller from the list_accounts
                    details.list_accounts = details.list_accounts.saturating_sub(1);
                    // update the mapping
                    self.open_list_details.insert(&list_id, &details);
                }
                // if the caller is not currently subscribed, send an error
                else {
                    return Err(Error::NonexistentList);
                }
            }        
            // if the list does not exist, send an error
            else {
                return Err(Error::NonexistentList);
            }

            Ok(())
        }


        // 21 🟢 Send A Paid Message 🏆
        #[ink(message, payable)]
        pub fn send_paid_message (&mut self, 
            to_account: AccountId,
            new_message: Vec<u8>,
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // if the input data is too large, send an error
            if new_message.len() > 600 || file_url.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            let caller = Self::env().caller();
            
            // COLLECT PAYMENT FROM THE CALLER (this is your bid)
            // the 'payable' tag on this message allows the user to send any amount
            let staked: Balance = self.env().transferred_value();
            
            // get the minimum fee for the to_account
            let min_fee: Balance = self.account_settings.get(&to_account).unwrap_or_default().inbox_fee;
            // get the recipient's blocked accounts 
            let blocked = self.account_blocked_accounts.get(&to_account).unwrap_or_default();

            // if the staked is less than min_fee, OR if the sender is blocked, send error 
            if staked < min_fee || blocked.accountvector.contains(&caller) {
                return Err(Error::CannotSendPaidMessage);
            }
            else {
                // get the current paid inbox for this recipient
                let mut inbox = self.account_paid_inbox.get(&to_account).unwrap_or_default();

                // if the inbox is full (has 400 messages)..
                if inbox.hashvector.len() > 399 {
                    // then bidding becomes competitive...
                    // get the current low bidder
                    let first_hash = inbox.hashvector[0];
                    // get the details of the first paid message
                    let firstdetails = self.paid_message_details.get(first_hash).unwrap_or_default();
                    let mut low_bid: Balance = firstdetails.bid;
                    let mut low_index: usize = 0;
                    let mut low_bid_acct: AccountId = firstdetails.from_acct;
                    for (i, ad) in inbox.hashvector.iter().enumerate() {
                        // get the bid and index for each and compare, keeping the lowest
                        let thisbid: Balance = self.paid_message_details.get(ad).unwrap_or_default().bid;
                        let thissender: AccountId = self.paid_message_details.get(ad).unwrap_or_default().from_acct;
                        if thisbid < low_bid { 
                            low_bid = thisbid;
                            low_index = i;
                            low_bid_acct = thissender;
                        }
                    }
                    // if staked is higher than the lowest current bid...
                    if staked > low_bid {
                        // kick out the low bidder 
                        inbox.hashvector.remove(low_index);
                        // make sure the contract has enough funds
                        if self.env().balance() > low_bid.saturating_add(11) {
                            // refund the low bidder their stake
                            if self.env().transfer(low_bid_acct, low_bid).is_err() {
                                return Err(Error::PayoutFailed);
                            }
                        }
                    }
                }

                // if the inbox has space, or this bid won, add this message to the paid inbox...
                // set up clones
                let new_message_clone = new_message.clone();
                // set up the data that will go into the new_message_id hash
                let new_timestamp = self.env().block_timestamp();
                // create the new_message_id by hashing the above data
                let encodable = (caller, new_message, new_timestamp); // Implements `scale::Encode`
                let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                let new_message_id: Hash = Hash::from(new_message_id_u8);

                // SET UP THE MESSAGE DETAILS FOR THE NEW MESSAGE
                let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                let new_details = PaidMessageDetails {
                    message_id: new_message_id,
                    from_acct: Self::env().caller(),
                    from_username: fromusername,
                    to_acct: to_account,
                    message: new_message_clone,  
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                    bid: staked,
                };

                // add the message to paid_message_details: Mapping<Hash, PaidMessageDetails>
                self.paid_message_details.insert(&new_message_id, &new_details);

                // add the message to the recipient's paid inbox
                inbox.hashvector.push(new_message_id);
                self.account_paid_inbox.insert(&to_account, &inbox);

                // add the recipient to the caller's allowed list if they are not already there
                // get the caller's allowed accounts vector
                let mut allowed = self.account_allowed.get(&caller).unwrap_or_default();
                if allowed.accountvector.contains(&to_account) {
                    // do nothing
                }
                else {
                    // check that there is space
                    if allowed.accountvector.len() < 60 {
                        // add this recipient
                        allowed.accountvector.push(to_account);
                    }
                    else {
                        // send error that storage is full
                        return Err(Error::StorageFull);
                    }
                    
                }

                // REWARD PROGRAM ACTIONS... update the claim_counter 
                self.claim_counter = self.claim_counter.wrapping_add(1);
                // IF conditions are met THEN payout a reward
                let min = self.reward_amount.saturating_add(10);
                let payout: Balance = self.reward_amount;
                if self.reward_on == 1 && self.reward_balance > payout && self.env().balance() > min
                && self.claim_counter.checked_rem_euclid(self.reward_interval) == Some(0) {
                    // payout
                    if self.env().transfer(caller, payout).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                    // update reward_balance
                    self.reward_balance = self.reward_balance.saturating_sub(payout);
                    // update reward_payouts
                    self.reward_payouts = self.reward_payouts.saturating_add(payout);
                    // emit an event to register the reward to the chain
                    Self::env().emit_event(AccountRewardedMessaging {
                        claimant: caller,
                        reward: payout
                    });
                }
                // END REWARD PROGRAM ACTIONS

            }

            Ok(())
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> PRIMARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
 
        // 22 🟢 Get My Inbox - Individual people only (groups and lists are in another message each)
        #[ink(message)]
        pub fn get_my_inbox(&self) -> Vec<ConversationWithAccount> {
            let caller = Self::env().caller();

            // set up return structures
            let mut peoplevector: Vec<ConversationWithAccount> = Vec::new();

            // GET ALL CONVERSATIONS WITH INDIVIDUAL ALLOWED ACCOUNTS
            // for every account in account_allowed: Mapping<AccountID, AccountVector>
            let allowed = self.account_allowed.get(&caller).unwrap_or_default();
            for acct in allowed.accountvector {

                // set up a conversation vector Vec<MessageDetails>
                let mut account_convo: Vec<MessageDetails> = Vec::new();
                let acct_username = self.account_settings.get(&acct).unwrap_or_default().username;
                // get all the message IDs from caller to that account
                let messagevec1 = self.sent_messages_to_account.get((&caller, &acct)).unwrap_or_default();
                for message in messagevec1.hashvector {
                    // get the message details for each message ID 
                    let details = self.message_details.get(&message).unwrap_or_default();
                    // and add it to the conversation vector between these accounts
                    account_convo.push(details);
                }
                // and get all the message IDs from that account to the caller
                let messagevec2 = self.sent_messages_to_account.get((&acct, &caller)).unwrap_or_default();
                for message in messagevec2.hashvector {
                    // get the message details for each message ID 
                    let details = self.message_details.get(&message).unwrap_or_default();
                    // and add it to the conversation vector between these accounts
                    account_convo.push(details);
                }
                // make the ConversationWithAccount...
                let convo_with_account = ConversationWithAccount {
                    allowed_account: acct,
                    username: acct_username,
                    conversation: account_convo,
                };
                // add the ConversationWithAccount to the Vec<ConversationWithAccount>
                peoplevector.push(convo_with_account);
            }

            // return the results
            peoplevector
        }


        // 23 🟢 Get My Paid Inbox 
        #[ink(message)]
        pub fn get_my_paid_inbox(&mut self) -> Result<Vec<PaidMessageDetails>, Error>  {
            let caller = Self::env().caller();
            let mut my_paid_inbox: Vec<PaidMessageDetails> = Vec::new();
            let mut payment: Balance = 0;

            // get the caller's inbox
            let messages = self.account_paid_inbox.get(&caller).unwrap_or_default();
            
            // for each message in the hash vector, get the PaidMessageDetails and the bid payment
            for ad in messages.hashvector.iter() {
                let details = self.paid_message_details.get(ad).unwrap_or_default();
                let bid = details.bid;
                // add it to the results
                my_paid_inbox.push(details);
                payment = payment.saturating_add(bid);
            }

            // pay the caller for picking up their paid inbox
            if self.env().balance() > payment.saturating_add(11) {
                if self.env().transfer(caller, payment).is_err() {
                    return Err(Error::PayoutFailed);
                }
            }
            // if the balance is too low, send error
            else {
                return Err(Error::ZeroBalance);
            }

            // clear the caller's inbox to make room for new paid messages
            // clear each message details from paid_message_details 
            for ad in messages.hashvector.iter() {
                self.paid_message_details.remove(ad);
            }
            // clear the inbox itself
            self.account_paid_inbox.remove(caller);

            // return the results
            Ok(my_paid_inbox)
        }


        // 24 🟢 Get My Allowed And Blocked Accounts
        #[ink(message)]
        pub fn get_my_allowed_and_blocked_accounts(&self) -> AccountsAllowedAndBlocked {
            let caller = Self::env().caller();
            let allowed = self.account_allowed.get(&caller).unwrap_or_default().accountvector;
            let blocked = self.account_blocked_accounts.get(&caller).unwrap_or_default().accountvector;

            // package the results
            let accounts = AccountsAllowedAndBlocked {
                allowed_accounts: allowed,
                blocked_accounts: blocked,
            };
            // return the results
            accounts
        }


        // 25 🟢 Get My Groups
        #[ink(message)]
        pub fn get_my_groups(&self) -> Vec<GroupDetails> {
            let caller = Self::env().caller();
            let mut mygroups: Vec<GroupDetails> = Vec::new();
            // get the vector of group ID this caller subscbribes to
            // from account_subscribed_groups: Mapping<AccountID, HashVector>
            let groupidvec = self.account_subscribed_groups.get(&caller).unwrap_or_default().hashvector;
            // for each groupid, get the details from group_details: Mapping<Hash, GroupDetails>
            for groupid in groupidvec {
                let details = self.group_details.get(&groupid).unwrap_or_default();
                mygroups.push(details);
            }
            // return the results
            mygroups
        }


        // 26 🟢 Find Groups By Keyword
        #[ink(message)]
        pub fn find_groups_by_keyword(&self, 
            keywords1: Vec<u8>, 
            keywords2: Vec<u8>, 
            keywords3: Vec<u8>) -> GroupSearchResults {
            // set up results structures
            let mut resultsvector: Vec<GroupPublicDetails> = Vec::new();
            // set up target string
            let targetvecu81 = keywords1.clone();
            let target_string1 = String::from_utf8(targetvecu81).unwrap_or_default();
            let targetvecu82 = keywords2.clone();
            let target_string2 = String::from_utf8(targetvecu82).unwrap_or_default();
            let targetvecu83 = keywords3.clone();
            let target_string3 = String::from_utf8(targetvecu83).unwrap_or_default();

            // for each group ID in the groups: StorageVec<Hash>... 
            if self.groups.len() > 0 {
                for i in 0..self.groups.len() {
                    let groupid = self.groups.get(i).unwrap_or_default();
                    // make sure the group still exists
                    if self.group_details.contains(groupid) {
                        // get the details from group_details: Mapping<Hash, GroupDetails>
                        let details = self.group_details.get(&groupid).unwrap_or_default();
                        // if the group is public
                        if details.hide_from_search == false {
                            // check the group name and description for a keyword match
                            let name_string = String::from_utf8(details.group_name.clone()).unwrap_or_default();
                            let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                            // if the target_string is in the group description or name
                            if (name_string.contains(&target_string1) || description_string.contains(&target_string1))
                            && (name_string.contains(&target_string2) || description_string.contains(&target_string2)) 
                            && (name_string.contains(&target_string3) || description_string.contains(&target_string3)) {
                                // create the public details
                                let public_details = GroupPublicDetails {
                                    group_id: details.group_id,
                                    group_name: details.group_name.clone(),
                                    description: details.description.clone(),
                                    subscribers: details.subscribers,
                                };
                                // add it to the results vector
                                resultsvector.push(public_details);
                            }
                        }
                    }

                }
            }

            // package the results
            let results = GroupSearchResults {
                search: vec![keywords1, keywords2, keywords3],
                groups: resultsvector,
            };
            // return the results
            results
        }


        // 27 🟢 Get My Open Lists
        #[ink(message)]
        pub fn get_my_open_lists(&self) -> Vec<OpenListDetails> {
            let caller = Self::env().caller();
            // set up the results vector
            let mut resultsvector: Vec<OpenListDetails> = Vec::new();
            // get the caller's account_owned_open_lists: Mapping<AccountID, HashVector>
            let mylists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            // for each list ID, get the details from open_list_details: Mapping<Hash, OpenListDetails>
            for listid in mylists.hashvector {
                let details = self.open_list_details.get(&listid).unwrap_or_default();
                // add the details to the results vector
                resultsvector.push(details);
            }
            // return the results
            resultsvector
        }


        // 28 🟢 Get My Subscribed Lists
        #[ink(message)]
        pub fn get_my_subscribed_lists(&self) -> Vec<OpenListPublicDetails> {
            let caller = Self::env().caller();
            // set up the results vector
            let mut resultsvector: Vec<OpenListPublicDetails> = Vec::new();
            // get the caller's account_subscribed_lists: Mapping<AccountID, HashVector>
            let mylists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            // for each list ID, get the details from open_list_details: Mapping<Hash, OpenListDetails>
            for listid in mylists.hashvector {
                let details = self.open_list_details.get(&listid).unwrap_or_default();
                // package the public details
                let public_details = OpenListPublicDetails {
                    list_id: details.list_id,
                    owner: details.owner,
                    list_name: details.list_name.clone(),
                    description: details.description.clone(),
                    list_accounts: details.list_accounts,
                };
                // add the details to the results vector
                resultsvector.push(public_details);
            }
            // return the results
            resultsvector
        }


        // 29 🟢 Find Lists By Keyword
        #[ink(message)]
        pub fn find_lists_by_keyword(&self, 
            keywords1: Vec<u8>, 
            keywords2: Vec<u8>, 
            keywords3: Vec<u8>) -> ListSearchResults {
            // set up results structures
            let mut resultsvector: Vec<OpenListPublicDetails> = Vec::new();
            // set up target string
            let targetvecu81 = keywords1.clone();
            let target_string1 = String::from_utf8(targetvecu81).unwrap_or_default();
            let targetvecu82 = keywords2.clone();
            let target_string2 = String::from_utf8(targetvecu82).unwrap_or_default();
            let targetvecu83 = keywords3.clone();
            let target_string3 = String::from_utf8(targetvecu83).unwrap_or_default();

            // for each list ID in the open_lists: StorageVec<Hash> ... 
            if self.open_lists.len() > 0 {
                for i in 0..self.open_lists.len() {
                    let listid = self.open_lists.get(i).unwrap_or_default();
                    // make sure the list exists
                    if self.open_list_details.contains(listid) {
                        // get the details from open_list_details: Mapping<Hash, OpenListDetails>
                        let details = self.open_list_details.get(&listid).unwrap_or_default();
                        // if the list is public
                        if details.hide_from_search == false {
                            // check the list name and description for a keyword match
                            let name_string = String::from_utf8(details.list_name.clone()).unwrap_or_default();
                            let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                            // if the target_string is in the list description or name
                            if (name_string.contains(&target_string1) || description_string.contains(&target_string1))
                            && (name_string.contains(&target_string2) || description_string.contains(&target_string2)) 
                            && (name_string.contains(&target_string3) || description_string.contains(&target_string3)) {
                                // create the public details
                                let public_details = OpenListPublicDetails {
                                    list_id: details.list_id,
                                    owner: details.owner,
                                    list_name: details.list_name.clone(),
                                    description: details.description.clone(),
                                    list_accounts: details.list_accounts,
                                };
                                // add it to the results vector
                                resultsvector.push(public_details);
                            }
                        }
                    }
                }
            }

            // package the results
            let results = ListSearchResults {
                search: vec![keywords1, keywords2, keywords3],
                lists: resultsvector,
            };
            // return the results
            results
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> SECONDARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


        // 30 🟢 Get Settings Data For Individual Account
        #[ink(message)]
        pub fn get_account_settings_data(&self, get_settings_for: AccountId) -> Settings {
            // get the specific account's settings
            let results = self.account_settings.get(&get_settings_for).unwrap_or_default();
            // return the results
            results
        }

        // 31 🟢 Verify an account has set their settings
        #[ink(message)]
        pub fn verify_account(&self, verify: AccountId) -> u8 {
            let mut results: u8 = 0;
            if self.account_settings.contains(&verify) {
                results = 1;
            }
            // return the results
            results
        }

        // 32 🟢 Get My Inbox - LISTS 
        #[ink(message)]
        pub fn get_my_inbox_lists(&self) -> MyInboxLists {
            let caller = Self::env().caller();
            // set up return structures
            let mut listsvector: Vec<MessagesFromList> = Vec::new();

            // GET ALL MESSAGES FROM SUBSCRIBED LISTS
            // for each list ID in account_subscribed_lists: Mapping<AccountID, HashVector>
            let lists = self.account_subscribed_lists.get(caller).unwrap_or_default();
            let mut defunct: Vec<Hash> = Vec::new();
            for listid in lists.hashvector.iter() {
                // if the list still exists, proceed...
                if self.open_list_details.contains(listid) {
                    // get the list name from open_list_details: Mapping<Hash, OpenListDetails>
                    let name = self.open_list_details.get(&listid).unwrap_or_default().list_name;
                    let mut listmessages:  Vec<ListMessageDetails> = Vec::new();
                    // get the message ID vector from sent_messages_to_list: Mapping<Hash, HashVector>
                    let messagevec = self.sent_messages_to_list.get(&listid).unwrap_or_default();
                    // get details for each messageID from list_message_details: Mapping<Hash, ListMessageDetails>
                    for message in messagevec.hashvector {
                        let details = self.list_message_details.get(&message).unwrap_or_default();
                        // add it to the vector of messages from this group
                        listmessages.push(details);
                    }
                    let messages_from_list = MessagesFromList {
                        allowed_list: *listid,
                        list_name: name,
                        list_messages: listmessages,
                    };
                    // add the MessageFromList to the Vec<MessageFromList>
                    listsvector.push(messages_from_list);
                }
                // if the list no longer exists, add it to this vector...
                defunct.push(*listid);
            }

            // package the inbox results
            let my_inbox = MyInboxLists {
                lists: listsvector,
                defunct_lists: defunct,
            };

            // return the results
            my_inbox
        }

        // 33 🟢 Get My Inbox - GROUPS
        #[ink(message)]
        pub fn get_my_inbox_groups(&self) -> Vec<MessagesFromList> {
            let caller = Self::env().caller();

            // set up return structures
            let mut groupsvector: Vec<MessagesFromList> = Vec::new();
            
            // GET ALL CONVERSATIONS WITH SUBSCRIBED GROUPS
            // for each group ID in account_subscribed_groups: Mapping<AccountID, HashVector>
            let groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
            for groupid in groups.hashvector {
                // get the group name from group_details: Mapping<Hash, GroupDetails>
                let name = self.group_details.get(&groupid).unwrap_or_default().group_name;
                let mut listmessages:  Vec<ListMessageDetails> = Vec::new();
                // get the message ID vector from all_messages_to_group: Mapping<Hash, HashVector>
                let messagevec = self.all_messages_to_group.get(&groupid).unwrap_or_default();
                // get the message details for each message ID
                for message in messagevec.hashvector {
                    let details = self.group_message_details.get(&message).unwrap_or_default();
                    // add it to the vector of messages from this group
                    listmessages.push(details);
                }
                let messages_from_list = MessagesFromList {
                    allowed_list: groupid,
                    list_name: name,
                    list_messages: listmessages,
                };
                // add the MessageFromList to the Vec<MessageFromList>
                groupsvector.push(messages_from_list);
            }

            // return the results
            groupsvector
        }

        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>> REWARD PROGRAM MESSAGES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


        // 🟢 34 Rewards - Set Or Update Reward Root Account [RESTRICTED: ROOT]
        #[ink(message)]
        pub fn set_reward_root(&mut self, newroot: AccountId) -> Result<(), Error> {
            let caller = Self::env().caller();
            // if the root is already set, send an error
            if self.reward_root_set != 1 || self.reward_root == caller {
                // proceed - set the roots and update the storage
                self.reward_root = newroot;
                self.reward_root_set = 1;
            }
            else {
                // error PermissionDenied
                return Err(Error::PermissionDenied)
            }

            Ok(())
        }


        // 🟢 35 Rewards - Set/Update Reward Interval and Ammount [RESTRICTED: ROOT]
        // Reward coin will be given to the account that makes the Xth claim in the system
        #[ink(message)]
        pub fn set_reward(&mut self, on: u8, interval: u128, amount: Balance) -> Result<(), Error> {
            let caller = Self::env().caller();
            if self.reward_root == caller {
                // proceed to set the reward program paramteters
                self.reward_on = on;
                self.reward_interval = interval;
                self.reward_amount = amount;
            }
            else {
                // error PermissionDenied
                return Err(Error::PermissionDenied)
            }
            
            Ok(())
        }

        // 🟢 36 ADD COIN TO REWARD ACCOUNT [RESTRICTED: ROOT]
        #[ink(message, payable)]
        pub fn add_reward_balance(&mut self) -> Result<(), Error> {
            let caller = Self::env().caller();
            if self.reward_root == caller {
                // add the paid in value to the reward_balance
                let staked: Balance = self.env().transferred_value();
                let newbalance: Balance = self.reward_balance.saturating_add(staked);
                self.reward_balance = newbalance;
            }
            else {
                // error PermissionDenied
                return Err(Error::PermissionDenied)
            }
            
            Ok(())
        }


        // 🟢 37 RETREIVE COIN FROM REWARD ACCOUNT [RESTRICTED: ROOT]
        // turns reward program off and returns funds to the root
        #[ink(message)]
        pub fn shut_down_reward(&mut self) -> Result<(), Error> {
            let caller = Self::env().caller();
            if self.reward_root == caller {
                // set the reward program to off
                self.reward_on = 0;
                // refund the coin to the reward root
                // Check that there is a nonzero balance on the contract > existential deposit
                if self.env().balance() > 10 && self.reward_balance > 0 {
                    // pay the root the reward_balance minus 10
                    let payout: Balance = self.reward_balance.saturating_sub(10);
                    if self.env().transfer(caller, payout).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                }
                // if the balance is < 10, Error (ZeroBalance)
                else {
                    return Err(Error::ZeroBalance);
                }
            }
            else {
                // error PermissionDenied
                return Err(Error::PermissionDenied)
            }
            
            Ok(())
        }


        // 🟢 38 GET CURRENT REWARD BALANCE AND SETTINGS [RESTRICTED: ROOT]
        #[ink(message)]
        pub fn get_reward_settings(&self) -> RewardSettings {
            let caller = Self::env().caller();
            let mut results = RewardSettings::default();
            if self.reward_root == caller {
                let settings = RewardSettings {
                    reward_on: self.reward_on,
                    reward_root_set: self.reward_root_set,
                    reward_root: self.reward_root,
                    reward_interval: self.reward_interval,
                    reward_amount: self.reward_amount,
                    reward_balance: self.reward_balance,
                    reward_payouts: self.reward_payouts,
                    claim_counter: self.claim_counter,
                };
                results = settings;
            }

            results
        }



        // END OF MESSAGE LIST

    }
    // END OF CONTRACT STORAGE

}
