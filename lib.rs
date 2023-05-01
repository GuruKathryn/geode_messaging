/*
ABOUT THIS CONTRACT...
This contract allows users to privately message each other the Geode Blockchain. 
- Send messages privately to other accounts,
- Only those you choose can message you,
- Others can pay to get into your PAID inbox,
- powerful group and list messaging features
- maximum user control! 
*/ 

#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod geode_messaging {

    use ink::prelude::vec::Vec;
    use ink::prelude::vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::env::hash::{Sha2x256, HashOutput};
    use openbrush::{
        contracts::{
            reentrancy_guard::*,
            traits::errors::ReentrancyGuardError,
        },
        traits::{
            Storage,
            ZERO_ADDRESS
        },
    };

    // PRELIMINARY STORAGE STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct Settings {
        user_account: AccountId,
        username: Vec<u8>,
        interests: Vec<u8>,
        inbox_fee: Balance,
        last_inbox_access: u64,
        last_paid_access: u64,
        hide_from_search: bool,
        last_update: u64,
    }

    impl Default for Settings {
        fn default() -> Settings {
            Settings {
                user_account: ZERO_ADDRESS.into(),
                username: <Vec<u8>>::default(),
                interests: <Vec<u8>>::default(),
                inbox_fee: Balance::default(),
                last_inbox_access: u64::default(),
                last_paid_access: u64::default(),
                hide_from_search: false,
                last_update: u64::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct SettingsData {
        interests: Vec<Vec<u8>>,
        inbox_fee: Vec<Balance>,
        last_inbox_access: Vec<u64>,
        last_paid_access: Vec<u64>,
        last_update: Vec<u64>,
    }

    impl Default for SettingsData {
        fn default() -> SettingsData {
            SettingsData {
                interests: <Vec<Vec<u8>>>::default(),
                inbox_fee: <Vec<Balance>>::default(),
                last_inbox_access: <Vec<u64>>::default(),
                last_paid_access: <Vec<u64>>::default(),
                last_update: <Vec<u64>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        from_username: Vec<u8>,
        to_acct: AccountId,
        message: Vec<u8>,
        file_hash: Hash, 
        file_url: Vec<u8>,
        timestamp: u64,
    }

    impl Default for MessageDetails {
        fn default() -> MessageDetails {
            MessageDetails {
                message_id: Hash::default(),
                from_acct: ZERO_ADDRESS.into(),
                from_username: <Vec<u8>>::default(),
                to_acct: ZERO_ADDRESS.into(),
                message: <Vec<u8>>::default(),
                file_hash: Hash::default(), 
                file_url: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ListMessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        username: Vec<u8>,
        to_list_id: Hash,
        to_list_name: Vec<u8>,
        message: Vec<u8>,
        file_hash: Hash, 
        file_url: Vec<u8>,
        timestamp: u64,
    }

    impl Default for ListMessageDetails {
        fn default() -> ListMessageDetails {
            ListMessageDetails {
                message_id: Hash::default(),
                from_acct: ZERO_ADDRESS.into(),
                username: <Vec<u8>>::default(),
                to_list_id: Hash::default(),
                to_list_name: <Vec<u8>>::default(),
                message: <Vec<u8>>::default(),
                file_hash: Hash::default(), 
                file_url: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
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

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct HashVector {
        hashvector: Vec<Hash>,
    }

    impl Default for HashVector {
        fn default() -> HashVector {
            HashVector {
              hashvector: <Vec<Hash>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct GroupDetails {
        group_id: Hash,
        group_name: Vec<u8>,
        hide_from_search: bool,
        description: Vec<u8>,
        group_accounts: Vec<AccountId>,
    }

    impl Default for GroupDetails {
        fn default() -> GroupDetails {
            GroupDetails {
                group_id: Hash::default(),
                group_name: <Vec<u8>>::default(),
                hide_from_search: false,
                description: <Vec<u8>>::default(),
                group_accounts: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct OpenListDetails {
        list_id: Hash,
        owner: AccountId,
        list_name: Vec<u8>,
        hide_from_search: bool,
        description: Vec<u8>,
        list_accounts: Vec<AccountId>,
    }

    impl Default for OpenListDetails {
        fn default() -> OpenListDetails {
            OpenListDetails {
                list_id: Hash::default(),
                owner: ZERO_ADDRESS.into(),
                list_name: <Vec<u8>>::default(),
                hide_from_search: false,
                description: <Vec<u8>>::default(),
                list_accounts: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct PaidListDetails {
        list_id: Hash,
        owner: AccountId,
        list_name: Vec<u8>,
        total_fee: Balance,
        description: Vec<u8>,
        list_accounts: Vec<AccountId>,
    }

    impl Default for PaidListDetails {
        fn default() -> PaidListDetails {
            PaidListDetails {
                list_id: Hash::default(),
                owner: ZERO_ADDRESS.into(),
                list_name: <Vec<u8>>::default(),
                total_fee: Balance::default(),
                description: <Vec<u8>>::default(),
                list_accounts: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ConversationWithAccount {
        allowed_account: AccountId,
        username: Vec<u8>,
        conversation: Vec<MessageDetails>
    }

    impl Default for ConversationWithAccount {
        fn default() -> ConversationWithAccount {
            ConversationWithAccount {
                allowed_account: ZERO_ADDRESS.into(),
                username: <Vec<u8>>::default(),
                conversation: <Vec<MessageDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MessagesFromList {
        allowed_list: Hash,
        list_name: Vec<u8>,
        list_messages: Vec<ListMessageDetails>
    }

    impl Default for MessagesFromList {
        fn default() -> MessagesFromList {
            MessagesFromList {
                allowed_list: Hash::default(),
                list_name: <Vec<u8>>::default(),
                list_messages: <Vec<ListMessageDetails>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MyInbox {
        blocked_accts: Vec<AccountId>,
        people: Vec<ConversationWithAccount>,
        groups: Vec<MessagesFromList>,
        lists: Vec<MessagesFromList>
    }

    impl Default for MyInbox {
        fn default() -> MyInbox {
            MyInbox {
                blocked_accts: <Vec<AccountId>>::default(),
                people: <Vec<ConversationWithAccount>>::default(),
                groups: <Vec<MessagesFromList>>::default(),
                lists: <Vec<MessagesFromList>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct InboxSearchResults {
        search: Vec<u8>,
        private_messages: Vec<MessageDetails>,
        group_messages: Vec<ListMessageDetails>,
        list_messages: Vec<ListMessageDetails>,
    }

    impl Default for InboxSearchResults {
        fn default() -> InboxSearchResults {
            InboxSearchResults {
                search: <Vec<u8>>::default(),
                private_messages: <Vec<MessageDetails>>::default(),
                group_messages: <Vec<ListMessageDetails>>::default(),
                list_messages: <Vec<ListMessageDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct InboxAcctSearchResults {
        search: AccountId,
        username: Vec<u8>,
        private_messages: Vec<MessageDetails>,
        group_messages: Vec<ListMessageDetails>,
        list_messages: Vec<ListMessageDetails>,
    }

    impl Default for InboxAcctSearchResults {
        fn default() -> InboxAcctSearchResults {
            InboxAcctSearchResults {
                search: ZERO_ADDRESS.into(),
                username: <Vec<u8>>::default(),
                private_messages: <Vec<MessageDetails>>::default(),
                group_messages: <Vec<ListMessageDetails>>::default(),
                list_messages: <Vec<ListMessageDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct GroupSearchResults {
        search: Vec<u8>,
        groups: Vec<GroupDetails>,
    }

    impl Default for GroupSearchResults {
        fn default() -> GroupSearchResults {
            GroupSearchResults {
                search: <Vec<u8>>::default(),
                groups: <Vec<GroupDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ListSearchResults {
        search: Vec<u8>,
        lists: Vec<OpenListDetails>,
    }

    impl Default for ListSearchResults {
        fn default() -> ListSearchResults {
            ListSearchResults {
                search: <Vec<u8>>::default(),
                lists: <Vec<OpenListDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct AccountSearchResults {
        search: Vec<u8>,
        accounts: Vec<Settings>,
    }

    impl Default for AccountSearchResults {
        fn default() -> AccountSearchResults {
            AccountSearchResults {
                search: <Vec<u8>>::default(),
                accounts: <Vec<Settings>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MyPaidInbox {
        blocked_lists: Vec<Hash>,
        messages: Vec<ListMessageDetails>,
    }

    impl Default for MyPaidInbox {
        fn default() -> MyPaidInbox {
            MyPaidInbox {
                blocked_lists: <Vec<Hash>>::default(),
                messages: <Vec<ListMessageDetails>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
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


    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // no events emitted in this contract


    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
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
        // Reentrancy Guard error
        ReentrancyError(ReentrancyGuardError),
        // Returned if the username already belongs to someone else.
        UsernameTaken,
        // removing an account that was not there
        NonexistentAccount,
    }

    impl From<ReentrancyGuardError> for Error {
        fn from(error:ReentrancyGuardError) -> Self {
            Error::ReentrancyError(error)
        }
    }


    // ACTUAL CONTRACT STORAGE STRUCT >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ContractStorage {
        #[storage_field]
        guard: reentrancy_guard::Data,
        account_settings: Mapping<AccountId, Settings>,
        account_allowed: Mapping<AccountId, AccountVector>,
        account_blocked_accounts: Mapping<AccountId, AccountVector>,
        account_blocked_lists: Mapping<AccountId, HashVector>,
        account_subscribed_groups: Mapping<AccountId, HashVector>,
        account_owned_open_lists: Mapping<AccountId, HashVector>,
        account_owned_paid_lists: Mapping<AccountId, HashVector>,
        paid_lists_with_account: Mapping<AccountId, HashVector>,
        account_subscribed_lists: Mapping<AccountId, HashVector>,
        sent_messages_to_account: Mapping<(AccountId, AccountId), HashVector>,
        sent_messages_to_list: Mapping<Hash, HashVector>,
        sent_messages_to_paid_list: Mapping<Hash, HashVector>,
        sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>,
        all_messages_to_group: Mapping<Hash, HashVector>,
        message_details: Mapping<Hash, MessageDetails>,
        list_message_details: Mapping<Hash, ListMessageDetails>,
        paid_list_message_details: Mapping<Hash, ListMessageDetails>,
        group_message_details: Mapping<Hash, ListMessageDetails>,
        open_list_details: Mapping<Hash, OpenListDetails>,
        paid_list_details: Mapping<Hash, PaidListDetails>,
        group_details: Mapping<Hash, GroupDetails>,
        open_lists: Vec<Hash>,
        groups: Vec<Hash>,
        username_map: Mapping<Vec<u8>, AccountId>,
        all_accounts_with_settings: Vec<AccountId>,
    }

    // note: to remove an entry from a mapping use my_map.remove(thing) 
    // see this page: https://use.ink/datastructures/mapping/


    // BEGIN CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                guard: Default::default(),
                account_settings: Mapping::default(),
                account_allowed: Mapping::default(),
                account_blocked_accounts: Mapping::default(),
                account_blocked_lists: Mapping::default(),
                account_subscribed_groups: Mapping::default(),
                account_owned_open_lists: Mapping::default(),
                account_owned_paid_lists: Mapping::default(),
                paid_lists_with_account: Mapping::default(),
                account_subscribed_lists: Mapping::default(),
                sent_messages_to_account: Mapping::default(),
                sent_messages_to_list: Mapping::default(),
                sent_messages_to_paid_list: Mapping::default(),
                sent_messages_to_group: Mapping::default(),
                all_messages_to_group: Mapping::default(),
                message_details: Mapping::default(),
                list_message_details: Mapping::default(),
                paid_list_message_details: Mapping::default(),
                group_message_details: Mapping::default(),
                open_list_details: Mapping::default(),
                paid_list_details: Mapping::default(),
                group_details: Mapping::default(),
                open_lists: <Vec<Hash>>::default(),
                groups: <Vec<Hash>>::default(),
                username_map: Mapping::default(),
                all_accounts_with_settings: <Vec<AccountId>>::default(),
            }
        }


        // MESSAGE FUNCTIONS THAT CHANGE DATA IN THE CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>

        // 1 🟢 Update Settings 
        // lets a user to update their list of keyword interests and other settings 
        // overwrites the mapping in contract storage
        #[ink(message)]
        pub fn update_settings (&mut self, 
            my_username: Vec<u8>,
            my_interests: Vec<u8>,
            my_inbox_fee: Balance,
            hide_from_search: bool
        ) -> Result<(), Error> {
            let username_clone1 = my_username.clone();
            let username_clone2 = my_username.clone();
            let username_clone3 = my_username.clone();
            let interests_clone = my_interests.clone();

            // get the current settings for this caller and prepare the update
            let caller = Self::env().caller();
            let current_settings = self.account_settings.get(&caller).unwrap_or_default();
            let settings_update: Settings = Settings {
                user_account: caller,
                username: my_username,
                interests: my_interests,
                inbox_fee: my_inbox_fee,
                last_inbox_access: current_settings.last_inbox_access,
                last_paid_access: current_settings.last_paid_access,
                hide_from_search: hide_from_search,
                last_update: self.env().block_timestamp()
            };
            
            // check that this user has not updated their settings in 24 hours
            let time_since_last_update = self.env().block_timestamp() - current_settings.last_update;
            if time_since_last_update < 86400000 {
                // send an error that interest cannot be updated so soon
                return Err(Error::CannotUpdateInterestsWithin24Hours)
            }
            else {
                // check that the set of interest keywords are not too long
                // maximum length is 600 which would give us 300 characters
                let interests_length = interests_clone.len();
                if interests_length > 600 {
                    // intrests are too long, send an error
                    return Err(Error::InterestsTooLong)
                }
                else {
                    // check that the username is not taken by someone else...
                    // if the username is in use already...
                    if self.username_map.contains(username_clone1) {
                        // get the account that owns that username
                        let current_owner = self.username_map.get(&username_clone2).unwrap();
                        // if the caller owns that username, update the storage maps
                        if current_owner == caller {
                            self.account_settings.insert(&caller, &settings_update);
                            // add this account to the vector of accounts with settings
                            if self.all_accounts_with_settings.contains(&caller) {
                                // do nothing
                            }
                            else {
                                // add the caller to the vector of accounts with settings
                                self.all_accounts_with_settings.push(caller);
                            }
                            
                        }
                        else {
                            // if the username belongs to someone else, send an error UsernameTaken
                            return Err(Error::UsernameTaken)
                        }
                    }
                    else {
                        // if the username is not already in use, update the storage map
                        self.account_settings.insert(&caller, &settings_update);
                        // then update the username map
                        self.username_map.insert(&username_clone3, &caller);
                        // then add this account to the vector of accounts with settings
                        if self.all_accounts_with_settings.contains(&caller) {
                            // do nothing
                        }
                        else {
                            // add the caller to the vector of accounts with settings
                            self.all_accounts_with_settings.push(caller);
                        }
                    }
                }
                
            }
            
            Ok(())
        }

        
        // 2 🟢 Send A Private Message
        #[ink(message)]
        pub fn send_private_message (&mut self, 
            to_acct: AccountId,
            new_message: Vec<u8>,
            file_hash: Hash, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
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
                    file_hash: file_hash, 
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // update the message_details: Mapping<Hash, MessageDetails>
                self.message_details.insert(&new_message_id, &new_details);

                // update the sent_messages_to_account: Mapping<(AccountId, AccountId), HashVector>
                // get the messages vector for this pair of accounts
                let caller = Self::env().caller();
                let mut current_messages = self.sent_messages_to_account.get((&caller, &to_acct)).unwrap_or_default();
                // add this message to the messages vector for this account
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_account map
                self.sent_messages_to_account.insert((&caller, &to_acct), &current_messages);

                Ok(())
            }
            else {
                // otherwise, if the caller is not allowed to message this account, send an error
                return Err(Error::NotAllowedToMessage)
            }

        }


        // 3 🟢 Send A Message To Group
        #[ink(message)]
        pub fn send_message_to_group (&mut self, 
            to_group_id: Hash,
            new_message: Vec<u8>,
            file_hash: Hash, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // check that the group actually exists
            if self.groups.contains(&to_group_id) {
                // set up clones
                let new_message_clone = new_message.clone();

                // set up the data that will go into the new_message_id hash
                let from = Self::env().caller();
                let new_timestamp = self.env().block_timestamp();

                // create the new_message_id by hashing the above data
                let encodable = (from, to_group_id, new_message, new_timestamp); // Implements `scale::Encode`
                let mut new_message_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_message_id_u8);
                let new_message_id: Hash = Hash::from(new_message_id_u8);

                // SET UP THE MESSAGE DETAILS FOR THE NEW MESSAGE
                let caller = Self::env().caller();
                let fromusername = self.account_settings.get(&caller).unwrap_or_default().username;
                let listname = self.group_details.get(&to_group_id).unwrap_or_default().group_name;
                let new_details = ListMessageDetails {
                    message_id: new_message_id,
                    from_acct: Self::env().caller(),
                    username: fromusername,
                    to_list_id: to_group_id,
                    to_list_name: listname,
                    message: new_message_clone,
                    file_hash: file_hash,  
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // update group_message_details: Mapping<Hash, ListMessageDetails>
                self.group_message_details.insert(&new_message_id, &new_details);

                // update sent_messages_to_group: Mapping<(AccountId, Hash), HashVector>
                // get the messages vector for this pair of account/group
                let caller = Self::env().caller();
                let mut current_messages = self.sent_messages_to_group.get((&caller, &to_group_id)).unwrap_or_default();
                // add this message to the messages vector for this account
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_group map
                self.sent_messages_to_group.insert((&caller, &to_group_id), &current_messages);

                // update all_messages_to_group: Mapping<Hash, HashVector>
                // get the messages vector for this group
                let mut current_messages = self.all_messages_to_group.get(&to_group_id).unwrap_or_default();
                // add this message to the messages vector for this account
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_group map
                self.all_messages_to_group.insert(&to_group_id, &current_messages);
            }
            else {
                return Err(Error::NoSuchList)
            }
            
            Ok(())

        }


        // 4 🟢 Allow Account
        #[ink(message)]
        pub fn allow_account (&mut self, allow: AccountId) -> Result<(), Error> {
            // Is this account already allowed? If TRUE, send ERROR
            let caller = Self::env().caller();
            let mut current_allowed = self.account_allowed.get(&caller).unwrap_or_default();
            if current_allowed.accountvector.contains(&allow) {
                return Err(Error::DuplicateAllow);
            }
            // Otherwise, update the account_allowed map for this caller
            else {
                // add the new allow to the the vector of accounts caller is allowing
                current_allowed.accountvector.push(allow);
                // Update (overwrite) the account_allowed: Mapping<AccountID, AccountVector> map
                self.account_allowed.insert(&caller, &current_allowed);
            }
            
            Ok(())
        }


        // 5 🟢 Disallow Account
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


        // 6 🟢 Block Account
        #[ink(message)]
        pub fn block_account (&mut self, block: AccountId) -> Result<(), Error> {
            // Is this account already being blocked? If TRUE, send ERROR
            let caller = Self::env().caller();
            let mut current_blocked = self.account_blocked_accounts.get(&caller).unwrap_or_default();
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
            
            Ok(())
        }


        // 7 🟢 Unblock Account
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


        // 8 🟢 Delete A Single Message To An Account
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
        

        // 9 🟢 Delete All Messages Sent To Account
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


        // 10 🟢 Make A Group (public or private)
        #[ink(message)]
        pub fn make_a_new_group (&mut self, 
            new_group_name: Vec<u8>,
            hide_from_search: bool,
            description: Vec<u8>,
            first_message: Vec<u8>,
            file_hash: Hash,  
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // set up any clones needed
            let first_message_clone = first_message.clone();
            let new_group_name_clone = new_group_name.clone();
            let new_group_name_clone2 = new_group_name.clone();
            // create the new_group_id by hashing the group name
            let encodable = new_group_name; // Implements `scale::Encode`
            let mut new_group_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_group_id_u8);
            let new_group_id: Hash = Hash::from(new_group_id_u8);

            // is the group name already taken?
            if self.groups.contains(&new_group_id) {
                // send an error
                return Err(Error::GroupNameTaken);
            }
            else {
                // proceed to set up the group
                // set up the group details and 
                // make the caller the first subscriber
                let caller = Self::env().caller();
                let new_group = GroupDetails {
                    group_id: new_group_id,
                    group_name: new_group_name_clone,
                    hide_from_search: hide_from_search,
                    description: description,
                    group_accounts: vec![caller],
                };
                // add it to group_details: Mapping<Hash, GroupDetails>
                self.group_details.insert(&new_group_id, &new_group);

                // add the group_id to the groups: Vec<Hash> in storage
                self.groups.push(new_group_id);

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
                    file_hash: file_hash,  
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // update group_message_details: Mapping<Hash, ListMessageDetails>
                self.group_message_details.insert(&new_message_id, &new_details);

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
                // update the sent_messages_to_group map
                self.all_messages_to_group.insert(&new_group_id, &current_messages);

                Ok(())
            }

        }


        // 11 🟢 Join A Group
        #[ink(message)]
        pub fn join_a_group (&mut self, group_id: Hash) -> Result<(), Error> {
            // does this group exist?
            if self.groups.contains(&group_id) {
                // get the caller's currently subscribed groups
                let caller = Self::env().caller();
                let mut current_groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
                // is the caller already subscribed?
                if current_groups.hashvector.contains(&group_id) {
                    return Err(Error::AlreadySubscribed);
                }
                else {
                    // push this group id onto the hashvector
                    current_groups.hashvector.push(group_id);
                    // update account_subscribed_groups: Mapping<AccountID, HashVector>
                    self.account_subscribed_groups.insert(&caller, &current_groups);
                }
                Ok(())
            }
            else {
                // send an error
                return Err(Error::NonexistentGroup);
            }
            
        }


        // 12 🟢 Delete A Single Group Message
        #[ink(message)]
        pub fn delete_single_message_to_group (&mut self, message_id_to_delete: Hash) -> Result<(), Error> {
            // does this message exist? If it does, proceed
            if self.group_message_details.contains(&message_id_to_delete) {
                // get the details for this message
                let caller = Self::env().caller();
                let current_details = self.group_message_details.get(&message_id_to_delete).unwrap_or_default();

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

                Ok(())
            }
            else {
                return Err(Error::MessageNotFound);
            }
        }


        // 13 🟢 Delete All Messages Sent To A Group
        #[ink(message)]
        pub fn delete_all_messages_to_group (&mut self, delete_my_messages_to_group_id: Hash) -> Result<(), Error> {
            // does this group exist? If it does, proceed
            let groupid = delete_my_messages_to_group_id;
            if self.groups.contains(&groupid) {
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

                Ok(())
            }
            else {
                return Err(Error::NonexistentGroup);
            }
        }


        // 14 🟢 Leave A Group
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
                // update the group details map
                self.group_details.insert(&group_id_to_leave, &current_details);

                Ok(())
            }
            // if the caller is not currently subscribed to this group, send an error
            else {
                return Err(Error::NonexistentGroup);
            }
            
        }


        // 15 🟢 Send A Message To List
        #[ink(message)]
        pub fn send_message_to_list (&mut self, 
            to_list_id: Hash,
            new_message: Vec<u8>,
            file_hash: Hash, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // Does this list exist? and do you own it? If so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            if self.open_lists.contains(&to_list_id) && owned_lists.hashvector.contains(&to_list_id) {
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
                    file_hash: file_hash,  
                    file_url: file_url,
                    timestamp: self.env().block_timestamp(),
                };

                // update list_message_details: Mapping<Hash, ListMessageDetails>
                self.list_message_details.insert(&new_message_id, &new_details);

                // update sent_messages_to_list: Mapping<Hash, HashVector>
                // get the messages vector for this list
                let mut current_messages = self.sent_messages_to_list.get(&to_list_id).unwrap_or_default();
                // add this message to the messages vector for this account
                current_messages.hashvector.push(new_message_id);
                // update the sent_messages_to_list map
                self.sent_messages_to_list.insert(&to_list_id, &current_messages);

                Ok(())

            }
            // if the list does not exist, or you do not own it, send an error
            else {
                return Err(Error::NonexistentList);
            }

        }


        // 16 🟢 Make A New List (public or private)
        #[ink(message)]
        pub fn make_a_new_list (&mut self, 
            new_list_name: Vec<u8>,
            hide_from_search: bool,
            description: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let list_name_clone = new_list_name.clone();
            // hash the list name
            let encodable = new_list_name; // Implements `scale::Encode`
            let mut new_list_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_list_id_u8);
            let new_list_id: Hash = Hash::from(new_list_id_u8);

            // is the list name already taken?
            if self.open_lists.contains(&new_list_id) {
                // send an error
                return Err(Error::ListNameTaken);
            }
            else {
                // proceed to set up the list details
                // make the caller the first subscriber
                let caller = Self::env().caller();
                let new_list = OpenListDetails {
                    list_id: new_list_id,
                    owner: caller,
                    list_name: list_name_clone,
                    hide_from_search: hide_from_search,
                    description: description,
                    list_accounts: vec![caller],
                };

                // add this new list to open_list_details: Mapping<Hash, OpenListDetails>
                self.open_list_details.insert(&new_list_id, &new_list);

                // add this new list to open_lists: Vec<Hash>
                self.open_lists.push(new_list_id);

                // add this new list ID to account_owned_open_lists: Mapping<AccountID, HashVector>
                let mut current_owned = self.account_owned_open_lists.get(&caller).unwrap_or_default();
                current_owned.hashvector.push(new_list_id);
                self.account_owned_open_lists.insert(&caller, &current_owned);

                // add this new list ID to account_subscribed_lists: Mapping<AccountID, HashVector>
                // (subscribe to your own list)
                let mut current_lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
                current_lists.hashvector.push(new_list_id);
                self.account_subscribed_lists.insert(&caller, &current_lists);

                Ok(())                
            }

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


        // 18 🟢 Delete An Open List (and all of its messages and subscribers)
        #[ink(message)]
        pub fn delete_an_open_list (&mut self, 
            delete_list_id: Hash
        ) -> Result<(), Error> {
            // does this list exist? and are you the owner? if so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
            if self.open_lists.contains(&delete_list_id) && owned_lists.hashvector.contains(&delete_list_id) {

                // get the details for this list
                let details = self.open_list_details.get(&delete_list_id).unwrap_or_default();

                // unsubscribe all accounts currently following this list using
                // account_subscribed_lists: Mapping<AccountID, HashVector>
                for subscriber in details.list_accounts {
                    // get their account_subscribed_lists
                    let mut lists = self.account_subscribed_lists.get(&subscriber).unwrap_or_default();
                    // remove this list
                    lists.hashvector.retain(|value| *value != delete_list_id);
                    // update the mapping for that account
                    self.account_subscribed_lists.insert(&subscriber, &lists);
                }

                // delete all the messages sent to this list
                // get the vector of messages sent to this list
                let sent = self.sent_messages_to_list.get(&delete_list_id).unwrap_or_default();
                // for each message
                for message in sent.hashvector {
                    // remove it from list_message_details: Mapping<Hash, ListMessageDetails>
                    self.list_message_details.remove(message);
                }
                // then remove the list ID from sent_messages_to_list: Mapping<Hash, HashVector>
                self.sent_messages_to_list.remove(delete_list_id);

                // remove this list from account_owned_open_lists: Mapping<AccountID, HashVector>
                let mut ownedlists = self.account_owned_open_lists.get(&caller).unwrap_or_default();
                // remove this list
                ownedlists.hashvector.retain(|value| *value != delete_list_id);
                // update the mapping for that account
                self.account_owned_open_lists.insert(&caller, &ownedlists);

                // remove the list ID from open_list_details: Mapping<Hash, OpenListDetails>
                self.open_list_details.remove(delete_list_id);

                // remove the list ID from open_lists: Vec<Hash>
                self.open_lists.retain(|value| *value != delete_list_id);

                Ok(())                
            }
            // if the list does not exist, send error
            else {
                return Err(Error::NonexistentList);
            }

        }


        // 19 🟢 Join An Open List
        #[ink(message)]
        pub fn join_an_open_list (&mut self, list_id: Hash) -> Result<(), Error> {
            // does this list exist in open_lists: Vec<Hash>? if so, proceed
            if self.open_lists.contains(&list_id) {
                let caller = Self::env().caller();
                // get the caller's currently subscribed lists
                let mut lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
                // is the caller already subscribed to this list? if so, error
                if lists.hashvector.contains(&list_id) {
                    return Err(Error::AlreadySubscribed);
                }
                // if the caller is not yet subscribed, proceed
                else {
                    // add the list to the caller's account_subscribed_lists: Mapping<AccountID, HashVector>
                    lists.hashvector.push(list_id);
                    // add the caller to the list_accounts in open_list_details: Mapping<Hash, OpenListDetails>
                    let mut details = self.open_list_details.get(&list_id).unwrap_or_default();
                    details.list_accounts.push(caller);
                    self.open_list_details.insert(&list_id, &details);

                    Ok(())
                }

            }
            // if the list does not exist, send an error
            else {
                return Err(Error::NonexistentList);
            }

        }


        // 20 🟢 Unsubscribe From An Open List
        #[ink(message)]
        pub fn unsubscribe_from_open_list (&mut self, 
            list_id: Hash
        ) -> Result<(), Error> {
            // does this list exist in open_lists: Vec<Hash>? if so, proceed
            if self.open_lists.contains(&list_id) {
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

                    // remove the caller account from list_accounts in 
                    // open_list_details: Mapping<Hash, OpenListDetails>
                    // get the details for this list
                    let mut details = self.open_list_details.get(&list_id).unwrap_or_default(); 
                    // remove the caller from the list_accounts
                    details.list_accounts.retain(|value| *value != caller);
                    // update the mapping
                    self.open_list_details.insert(&list_id, &details);

                    Ok(())
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

        }


        // 21 🟢 Send A Message To Paid List
        #[ink(message, payable)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn send_message_to_paid_list (&mut self, 
            to_list_id: Hash,
            new_message: Vec<u8>,
            file_hash: Hash, 
            file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // Does this list exist? and do you own it? If so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_paid_lists.get(&caller).unwrap_or_default();
            if owned_lists.hashvector.contains(&to_list_id) {
                
                // COLLECT PAYMENT FROM THE CALLER
                // the 'payable' tag on this message allows the user to send any amount
                let staked: Balance = self.env().transferred_value();
                // total the fees required to send to this list                
                // get the list_accounts from paid_list_details: Mapping<Hash, PaidListDetails>
                let listaccounts = self.paid_list_details.get(&to_list_id).unwrap_or_default().list_accounts;
                let mut total_fee: Balance = 0;
                // for each acccount on the paid list...
                for inbox in &listaccounts {
                    // get their inbox fee from account_settings: Mapping<AccountID, Settings>
                    let fee: Balance = self.account_settings.get(&inbox).unwrap_or_default().inbox_fee;
                    // add it to the total_fee
                    total_fee += fee;
                }
                // if staked is more than total_fee, proceed
                if staked > total_fee {
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
                    let listname = self.paid_list_details.get(&to_list_id).unwrap_or_default().list_name;
                    let new_details = ListMessageDetails {
                        message_id: new_message_id,
                        from_acct: Self::env().caller(),
                        username: fromusername,
                        to_list_id: to_list_id,
                        to_list_name: listname,
                        message: new_message_clone,
                        file_hash: file_hash,  
                        file_url: file_url,
                        timestamp: self.env().block_timestamp(),
                    };

                    // add the message to paid_list_message_details: Mapping<Hash, ListMessageDetails>
                    self.paid_list_message_details.insert(&new_message_id, &new_details);

                    // add the message ID to sent_messages_to_paid_list: Mapping<Hash, HashVector>
                    // get the messages vector for this list
                    let mut current_messages = self.sent_messages_to_paid_list.get(&to_list_id).unwrap_or_default();
                    // add this message to the messages vector for this list
                    current_messages.hashvector.push(new_message_id);
                    // update the sent_messages_to_paid_list map
                    self.sent_messages_to_paid_list.insert(&to_list_id, &current_messages);

                    // PAY EACH ACCOUNT ON THE PAID LIST 
                    for inbox in listaccounts {
                        // get their inbox fee from account_settings: Mapping<AccountID, Settings>
                        let fee: Balance = self.account_settings.get(&inbox).unwrap_or_default().inbox_fee;
                        // send them their inbox_fee
                        self.env().transfer(inbox, fee).expect("payout failed");
                        if self.env().transfer(inbox, fee).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }
                    // then pay the caller what is left from their stake after paying the list
                    let leftovers: Balance = staked - total_fee;
                    if leftovers > 0 {
                        self.env().transfer(caller, leftovers).expect("payout failed");
                        if self.env().transfer(caller, leftovers).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }

                    Ok(())
                }
                // if staked is less than or = to the total fee, send an error
                else {
                    return Err(Error::InsufficientStake);
                }

            }
            // if the list does not exist, or you do not own it, send an error
            else {
                return Err(Error::NonexistentList);
            }

        }


        // 22 🟢 Make A Paid List
        #[ink(message)]
        pub fn make_a_new_paid_list (&mut self, 
            new_list_name: Vec<u8>,
            description: Vec<u8>,
            initial_accounts: Vec<AccountId>,
        ) -> Result<(), Error> {
            let caller = Self::env().caller();
            // set up clones
            let list_name_clone = new_list_name.clone();
            // hash the list name and owner
            let encodable = (caller, new_list_name); // Implements `scale::Encode`
            let mut new_list_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_list_id_u8);
            let new_list_id: Hash = Hash::from(new_list_id_u8);

            // does the caller already have a list by this name?
            let mut paid_lists = self.account_owned_paid_lists.get(&caller).unwrap_or_default();
            if paid_lists.hashvector.contains(&new_list_id) {
                // send an error
                return Err(Error::ListNameTaken);
            }
            else {
                // calculate the total fee
                let mut totalfee: Balance = 0;
                for acct in &initial_accounts {
                    // get their inbox fee from account_settings: Mapping<AccountID, Settings>
                    let fee: Balance = self.account_settings.get(&acct).unwrap_or_default().inbox_fee;
                    // add it to the total_fee
                    totalfee += fee;
                    // update paid_lists_with_account: Mapping<AccountID, HashVector>
                    let mut lists = self.paid_lists_with_account.get(&acct).unwrap_or_default();
                    lists.hashvector.push(new_list_id);
                    self.paid_lists_with_account.insert(&acct, &lists);
                }
                // set up the list details
                let new_list = PaidListDetails {
                    list_id: new_list_id,
                    owner: caller,
                    list_name: list_name_clone,
                    total_fee: totalfee,
                    description: description,
                    list_accounts: initial_accounts,
                };

                // add this new list to paid_list_details: Mapping<Hash, PaidListDetails>
                self.paid_list_details.insert(&new_list_id, &new_list);

                // add this new list ID to account_owned_paid_lists: Mapping<AccountID, HashVector>
                paid_lists.hashvector.push(new_list_id);
                self.account_owned_paid_lists.insert(&caller, &paid_lists);

                Ok(())                
            }
        }


        // 23 🟢 Delete Paid List (and ALL of its messages ever sent)
        #[ink(message)]
        pub fn delete_paid_list (&mut self, delete_this_list: Hash) -> Result<(), Error> {
            // do you own this paid list? if so, proceed
            let caller = Self::env().caller();
            let owned_lists = self.account_owned_paid_lists.get(&caller).unwrap_or_default();
            if owned_lists.hashvector.contains(&delete_this_list) {

                // remove all the messages sent to this list...
                // get the vector of messages sent to this list
                let sent = self.sent_messages_to_paid_list.get(&delete_this_list).unwrap_or_default();
                // for each message...
                for message in sent.hashvector {
                    // remove it from paid_list_message_details: Mapping<Hash, ListMessageDetails>
                    self.paid_list_message_details.remove(message);
                }
                // then remove the list ID from sent_messages_to_paid_list: Mapping<Hash, HashVector>
                self.sent_messages_to_paid_list.remove(delete_this_list);

                // for each account on the list, remove the list ID from
                // paid_lists_with_account: Mapping<AccountID, HashVector> 
                let details = self.paid_list_details.get(&delete_this_list).unwrap_or_default();
                let listaccts = details.list_accounts;
                for acct in listaccts {
                    let mut paidlists = self.paid_lists_with_account.get(&acct).unwrap_or_default();
                    paidlists.hashvector.retain(|value| *value != delete_this_list);
                    self.paid_lists_with_account.insert(&acct, &paidlists);
                }

                // remove the list ID from paid_list_details: Mapping<Hash, PaidListDetails>
                self.paid_list_details.remove(delete_this_list);

                // remove the list ID from account_owned_paid_lists: Mapping<AccountID, HashVector>
                let mut ownedlists = self.account_owned_paid_lists.get(&caller).unwrap_or_default();
                // remove this list
                ownedlists.hashvector.retain(|value| *value != delete_this_list);
                // update the mapping for that account
                self.account_owned_paid_lists.insert(&caller, &ownedlists);

                Ok(())
            }
            // if you don't own this paid list, send error
            else {
                return Err(Error::NonexistentList);
            }

        }
 

        // 24 🟢 Block A Paid List
        #[ink(message)]
        pub fn block_paid_list (&mut self, list_id_to_block: Hash) -> Result<(), Error> {
            // Is this list already being blocked? If TRUE, send ERROR
            let caller = Self::env().caller();
            let mut current_blocked = self.account_blocked_lists.get(&caller).unwrap_or_default();
            if current_blocked.hashvector.contains(&list_id_to_block) {
                return Err(Error::DuplicateBlock);
            }
            // Otherwise, update the account_blocked_lists for this caller
            else {
                // add the new block to the the vector of lists the caller is blocking
                current_blocked.hashvector.push(list_id_to_block);
                // Update (overwrite) the account_blocked_lists: Mapping<AccountID, HashVector> map
                self.account_blocked_lists.insert(&caller, &current_blocked);
            }
            
            Ok(())
        }


        // 25 🟢 Unblock A Paid List
        #[ink(message)]
        pub fn unblock_paid_list (&mut self, list_id_to_unblock: Hash) -> Result<(), Error> {
            // Is this account currently being blocked? If TRUE, proceed...
            let caller = Self::env().caller();
            let mut current_blocked = self.account_blocked_lists.get(&caller).unwrap_or_default();
            if current_blocked.hashvector.contains(&list_id_to_unblock) {
                // remove the unblock from the the vector of lists they are blocking
                // by keeping everyone other than that list... 
                current_blocked.hashvector.retain(|value| *value != list_id_to_unblock);
                // Update (overwrite) the account_blocked_lists map in the storage
                self.account_blocked_lists.insert(&caller, &current_blocked);
            }
            // If the account is not currently being followed, ERROR: Already Not blocked
            else {
                return Err(Error::NotBlocked);
            }

            Ok(())
        }   
       

        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> PRIMARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
 
        // 26 🟢 Get My Inbox
        #[ink(message)]
        pub fn get_my_inbox(&mut self) -> MyInbox {
            let caller = Self::env().caller();

            // set up return structures
            let myblocked = self.account_blocked_accounts.get(&caller).unwrap_or_default().accountvector;
            let mut peoplevector: Vec<ConversationWithAccount> = Vec::new();
            let mut groupsvector: Vec<MessagesFromList> = Vec::new();
            let mut listsvector: Vec<MessagesFromList> = Vec::new();

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

            // GET ALL MESSAGES FROM SUBSCRIBED LISTS
            // for each list ID in account_subscribed_lists: Mapping<AccountID, HashVector>
            let lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            for listid in lists.hashvector {
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
                    allowed_list: listid,
                    list_name: name,
                    list_messages: listmessages,
                };
                // add the MessageFromList to the Vec<MessageFromList>
                listsvector.push(messages_from_list);
            }

            // get the current timestamp
            let rightnow = self.env().block_timestamp();
            // get the caller's settings and update the last_inbox_access
            let current_settings = self.account_settings.get(&caller).unwrap_or_default();
            let update = Settings {
                user_account: current_settings.user_account,
                username: current_settings.username,
                interests: current_settings.interests,
                inbox_fee: current_settings.inbox_fee,
                last_inbox_access: rightnow,
                last_paid_access: current_settings.last_paid_access,
                hide_from_search: current_settings.hide_from_search,
                last_update: current_settings.last_update,
            };
            self.account_settings.insert(&caller, &update);

            // package the inbox results
            let my_inbox = MyInbox {
                blocked_accts: myblocked,
                people: peoplevector,
                groups: groupsvector,
                lists: listsvector,
            };

            // return the results
            my_inbox
        }


        // 27 🟢 Get My Paid Inbox
        #[ink(message)]
        pub fn get_my_paid_inbox(&mut self) -> MyPaidInbox {
            let caller = Self::env().caller();
            // get the caller's account_blocked_lists: Mapping<AccountID, HashVector>
            let blocked = self.account_blocked_lists.get(&caller).unwrap_or_default().hashvector;
            // set up the return structure
            let mut mypaidmessages: Vec<ListMessageDetails> = Vec::new();

            // get the vector of paid lists that this account is on
            // from paid_lists_with_account: Mapping<AccountID, HashVector>
            let paidlists = self.paid_lists_with_account.get(&caller).unwrap_or_default();
            // for each list, get the message IDs sent to that list
            // from sent_messages_to_paid_list: Mapping<Hash, HashVector>
            for listid in paidlists.hashvector {
                let messagevec = self.sent_messages_to_paid_list.get(&listid).unwrap_or_default();
                // for each message ID, get the details 
                // from paid_list_message_details: Mapping<Hash, ListMessageDetails>
                for message in messagevec.hashvector {
                    let details = self.paid_list_message_details.get(&message).unwrap_or_default();
                    // add the details to the mypaidmessages vector
                    mypaidmessages.push(details);
                }
                // loop through the rest of the lists
            }

            // get the current timestamp
            let rightnow = self.env().block_timestamp();
            // get the caller's settings and update the last_paid_access
            let current_settings = self.account_settings.get(&caller).unwrap_or_default();
            let update = Settings {
                user_account: current_settings.user_account,
                username: current_settings.username,
                interests: current_settings.interests,
                inbox_fee: current_settings.inbox_fee,
                last_inbox_access: current_settings.last_inbox_access,
                last_paid_access: rightnow,
                hide_from_search: current_settings.hide_from_search,
                last_update: current_settings.last_update,
            };
            self.account_settings.insert(&caller, &update);

            // package the inbox results
            let my_paid_inbox = MyPaidInbox {
                blocked_lists: blocked,
                messages: mypaidmessages,
            };

            // return the results
            my_paid_inbox
        }


        // 28 🟢 Get My Allowed And Blocked Accounts
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


        // 29 🟢 Get My Groups
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


        // 30 🟢 Search Inbox By Keyword
        #[ink(message)]
        pub fn search_inbox_by_keyword(&self, keywords: Vec<u8>) -> InboxSearchResults {
            let caller = Self::env().caller();
            // set up results structures
            let mut peoplevector: Vec<MessageDetails> = Vec::new();
            let mut groupsvector: Vec<ListMessageDetails> = Vec::new();
            let mut listsvector: Vec<ListMessageDetails> = Vec::new();
            let targetvecu8 = keywords.clone();
            let target_string = String::from_utf8(targetvecu8).unwrap_or_default();

            // call for MyInbox and check messages for a match along the way...

            // GET ALL CONVERSATIONS WITH INDIVIDUAL ALLOWED ACCOUNTS
            // for every account in account_allowed: Mapping<AccountID, AccountVector>
            let allowed = self.account_allowed.get(&caller).unwrap_or_default();
            for acct in allowed.accountvector {
                let acct_username = self.account_settings.get(&acct).unwrap_or_default().username;
                // get all the message IDs from caller to that account
                let messagevec1 = self.sent_messages_to_account.get((&caller, &acct)).unwrap_or_default();
                for message in messagevec1.hashvector {
                    // get the message details for each message ID 
                    let details = self.message_details.get(&message).unwrap_or_default();
                    // check to see if the keywords are in the message
                    let message_string = String::from_utf8(details.message.clone()).unwrap_or_default();
                    let username_string = String::from_utf8(acct_username.clone()).unwrap_or_default();
                    let url_string = String::from_utf8(details.file_url.clone()).unwrap_or_default();
                    // if the target_string is in the message_string
                    if message_string.contains(&target_string) || username_string.contains(&target_string) || 
                    url_string.contains(&target_string) {
                        // add it to the results vector
                        peoplevector.push(details);
                    }
                }
                // and get all the message IDs from that account to the caller
                let messagevec2 = self.sent_messages_to_account.get((&acct, &caller)).unwrap_or_default();
                for message in messagevec2.hashvector {
                    // get the message details for each message ID 
                    let details = self.message_details.get(&message).unwrap_or_default();
                    // check to see if the keywords are in the message
                    let message_string = String::from_utf8(details.message.clone()).unwrap_or_default();
                    let username_string = String::from_utf8(details.from_username.clone()).unwrap_or_default();
                    let url_string = String::from_utf8(details.file_url.clone()).unwrap_or_default();
                    // if the target_string is in the message_string
                    if message_string.contains(&target_string) || username_string.contains(&target_string) || 
                    url_string.contains(&target_string) {
                        // add it to the results vector
                        peoplevector.push(details);
                    }
                }
            }
            
            // GET ALL CONVERSATIONS WITH SUBSCRIBED GROUPS
            // for each group ID in account_subscribed_groups: Mapping<AccountID, HashVector>
            let groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
            for groupid in groups.hashvector {
                // get the message ID vector from all_messages_to_group: Mapping<Hash, HashVector>
                let messagevec = self.all_messages_to_group.get(&groupid).unwrap_or_default();
                // get the message details for each message ID
                for message in messagevec.hashvector {
                    let details = self.group_message_details.get(&message).unwrap_or_default();
                    // check to see if the keywords are in the message
                    let message_string = String::from_utf8(details.message.clone()).unwrap_or_default();
                    let username_string = String::from_utf8(details.username.clone()).unwrap_or_default();
                    let url_string = String::from_utf8(details.file_url.clone()).unwrap_or_default();
                    let list_string = String::from_utf8(details.to_list_name.clone()).unwrap_or_default();
                    // if the target_string is in the message_string
                    if message_string.contains(&target_string) || username_string.contains(&target_string) || 
                    url_string.contains(&target_string) || list_string.contains(&target_string) {
                        // add it to the results vector
                        groupsvector.push(details);
                    }
                }
            }

            // GET ALL MESSAGES FROM SUBSCRIBED LISTS
            // for each list ID in account_subscribed_lists: Mapping<AccountID, HashVector>
            let lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            for listid in lists.hashvector {
                // get the message ID vector from sent_messages_to_list: Mapping<Hash, HashVector>
                let messagevec = self.sent_messages_to_list.get(&listid).unwrap_or_default();
                // get details for each messageID from list_message_details: Mapping<Hash, ListMessageDetails>
                for message in messagevec.hashvector {
                    let details = self.list_message_details.get(&message).unwrap_or_default();
                    // check to see if the keywords are in the message
                    let message_string = String::from_utf8(details.message.clone()).unwrap_or_default();
                    let username_string = String::from_utf8(details.username.clone()).unwrap_or_default();
                    let url_string = String::from_utf8(details.file_url.clone()).unwrap_or_default();
                    let list_string = String::from_utf8(details.to_list_name.clone()).unwrap_or_default();
                    // if the target_string is in the message_string
                    if message_string.contains(&target_string) || username_string.contains(&target_string) || 
                    url_string.contains(&target_string) || list_string.contains(&target_string) {
                        // add it to the results vector
                        listsvector.push(details);
                    }
                }
            }

            // package the results
            let results = InboxSearchResults {
                search: keywords,
                private_messages: peoplevector,
                group_messages: groupsvector,
                list_messages: listsvector,
            };
            // return the results
            results
        }

        
        // 31 🟢 Search Inbox By Account
        #[ink(message)]
        pub fn search_inbox_by_account(&self, find_account: AccountId) -> InboxAcctSearchResults {
            let caller = Self::env().caller();
            // set up results structures
            let mut peoplevector: Vec<MessageDetails> = Vec::new();
            let mut groupsvector: Vec<ListMessageDetails> = Vec::new();
            let mut listsvector: Vec<ListMessageDetails> = Vec::new();
            let acct_username = self.account_settings.get(&find_account).unwrap_or_default().username;

            // GET ALL CONVERSATIONS WITH THAT ACCOUNT 
            // get all the message IDs from caller to that account
            let messagevec1 = self.sent_messages_to_account.get((&caller, &find_account)).unwrap_or_default();
            for message in messagevec1.hashvector {
                // get the message details for each message ID 
                let details = self.message_details.get(&message).unwrap_or_default();
                // add it to the results vector
                peoplevector.push(details); 
            }
            // and get all the message IDs from that account to the caller
            let messagevec2 = self.sent_messages_to_account.get((&find_account, &caller)).unwrap_or_default();
            for message in messagevec2.hashvector {
                // get the message details for each message ID 
                let details = self.message_details.get(&message).unwrap_or_default();
                // add it to the results vector
                peoplevector.push(details);
            }
            
            // GET ALL CONVERSATIONS WITH SUBSCRIBED GROUPS
            // for each group ID in account_subscribed_groups: Mapping<AccountID, HashVector>
            let groups = self.account_subscribed_groups.get(&caller).unwrap_or_default();
            for groupid in groups.hashvector {
                // get the message ID vector from all_messages_to_group: Mapping<Hash, HashVector>
                let messagevec = self.all_messages_to_group.get(&groupid).unwrap_or_default();
                // get the message details for each message ID
                for message in messagevec.hashvector {
                    let details = self.group_message_details.get(&message).unwrap_or_default();
                    // if find_account sent this message, add it to the results vector
                    if details.from_acct == find_account {
                        // add it to the results vector
                        groupsvector.push(details);
                    }
                }
            }

            // GET ALL MESSAGES FROM SUBSCRIBED LISTS OWNED BY THAT FIND_ACCOUNT
            // for each list ID in account_subscribed_lists: Mapping<AccountID, HashVector>
            let lists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            for listid in lists.hashvector {
                // get the owner of that list from open_list_details: Mapping<Hash, OpenListDetails>
                let listowner = self.open_list_details.get(&listid).unwrap_or_default().owner;
                if listowner == find_account {
                    // get the message ID vector from sent_messages_to_list: Mapping<Hash, HashVector>
                    let messagevec = self.sent_messages_to_list.get(&listid).unwrap_or_default();
                    // get details for each messageID from list_message_details: Mapping<Hash, ListMessageDetails>
                    for message in messagevec.hashvector {
                        let details = self.list_message_details.get(&message).unwrap_or_default();
                        // add it to the results vector
                        listsvector.push(details);
                    }
                }
            }

            // package the results
            let results = InboxAcctSearchResults {
                search: find_account,
                username: acct_username,
                private_messages: peoplevector,
                group_messages: groupsvector,
                list_messages: listsvector,
            };
            // return the results
            results
        }


        // 32 🟢 Find Groups By Keyword
        #[ink(message)]
        pub fn find_groups_by_keyword(&self, keywords: Vec<u8>) -> GroupSearchResults {
            // set up results structures
            let mut resultsvector: Vec<GroupDetails> = Vec::new();
            // set up target string
            let targetvecu8 = keywords.clone();
            let target_string = String::from_utf8(targetvecu8).unwrap_or_default();

            // for each group ID in the groups: Vec<Hash>... 
            for groupid in &self.groups {
                // get the details from group_details: Mapping<Hash, GroupDetails>
                let details = self.group_details.get(&groupid).unwrap_or_default();
                // if the group is public
                if details.hide_from_search == false {
                    // check the group name and description for a keyword match
                    let name_string = String::from_utf8(details.group_name.clone()).unwrap_or_default();
                    let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                    // if the target_string is in the group description or name
                    if name_string.contains(&target_string) || description_string.contains(&target_string) {
                        // add it to the results vector
                        resultsvector.push(details);
                    }
                }
            }
            // package the results
            let results = GroupSearchResults {
                search: keywords,
                groups: resultsvector,
            };
            // return the results
            results
        }


        // 33 🟢 Get My Open Lists
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


        // 34 🟢 Get My Paid Lists
        #[ink(message)]
        pub fn get_my_paid_lists(&self) -> Vec<PaidListDetails> {
            let caller = Self::env().caller();
            // set up the results vector
            let mut resultsvector: Vec<PaidListDetails> = Vec::new();
            // get the caller's account_owned_paid_lists: Mapping<AccountID, HashVector>
            let mylists = self.account_owned_paid_lists.get(&caller).unwrap_or_default();
            // for each list ID, get the details from paid_list_details: Mapping<Hash, PaidListDetails
            for listid in mylists.hashvector {
                let details = self.paid_list_details.get(&listid).unwrap_or_default();
                // add the details to the results vector
                resultsvector.push(details);
            }
            // return the results
            resultsvector
        }


        // 35 🟢 Get My Subscribed Lists
        #[ink(message)]
        pub fn get_my_subscribed_lists(&self) -> Vec<OpenListDetails> {
            let caller = Self::env().caller();
            // set up the results vector
            let mut resultsvector: Vec<OpenListDetails> = Vec::new();
            // get the caller's account_subscribed_lists: Mapping<AccountID, HashVector>
            let mylists = self.account_subscribed_lists.get(&caller).unwrap_or_default();
            // for each list ID, get the details from open_list_details: Mapping<Hash, OpenListDetails>
            for listid in mylists.hashvector {
                let details = self.open_list_details.get(&listid).unwrap_or_default();
                // add the details to the results vector
                resultsvector.push(details);
            }
            // return the results
            resultsvector
        }


        // 36 🟢 Find Lists By Keyword
        #[ink(message)]
        pub fn find_lists_by_keyword(&self, keywords: Vec<u8>) -> ListSearchResults {
            // set up results structures
            let mut resultsvector: Vec<OpenListDetails> = Vec::new();
            // set up target string
            let targetvecu8 = keywords.clone();
            let target_string = String::from_utf8(targetvecu8).unwrap_or_default();

            // for each list ID in the open_lists: Vec<Hash> ... 
            for listid in &self.open_lists {
                // get the details from open_list_details: Mapping<Hash, OpenListDetails>
                let details = self.open_list_details.get(&listid).unwrap_or_default();
                // if the list is public
                if details.hide_from_search == false {
                    // check the list name and description for a keyword match
                    let name_string = String::from_utf8(details.list_name.clone()).unwrap_or_default();
                    let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                    // if the target_string is in the list description or name
                    if name_string.contains(&target_string) || description_string.contains(&target_string) {
                        // add it to the results vector
                        resultsvector.push(details);
                    }
                }
            }
            // package the results
            let results = ListSearchResults {
                search: keywords,
                lists: resultsvector,
            };
            // return the results
            results
        }


        // 37 🟢 Find Accounts By Keyword
        // Useful for making paid lists. Returns all settings details.
        // Front end might allow the user to select a set of accounts, tell them the 
        // total fee (currently) and let them copy a comma separated list of the account IDs
        // so they can make a new list easily
        #[ink(message)]
        pub fn find_accounts_by_keyword(&self, keywords: Vec<u8>) -> AccountSearchResults {
            // set up results structures
            let mut resultsvector: Vec<Settings> = Vec::new();
            // set up target string
            let targetvecu8 = keywords.clone();
            let target_string = String::from_utf8(targetvecu8).unwrap_or_default();
            // iterate on all_accounts_with_settings: Vec<AccountId>
            for acct in &self.all_accounts_with_settings {
                // get their settings in account_settings: Mapping<AccountID, Settings>
                let settings = self.account_settings.get(&acct).unwrap_or_default();
                // if they are not hidden from search...
                if settings.hide_from_search == false {
                    // check the interests for a keyword match
                    let interests_string = String::from_utf8(settings.interests.clone()).unwrap_or_default();
                    let username_string = String::from_utf8(settings.username.clone()).unwrap_or_default();
                    // if the target_string is in the list description or name
                    if interests_string.contains(&target_string) || username_string.contains(&target_string) {
                        // add it to the results vector
                        resultsvector.push(settings);
                    }
                }
            }
            // package the results
            let results = AccountSearchResults {
                search: keywords,
                accounts: resultsvector,
            };
            // return the results
            results
        }
  
  
        // 38 🟢 Get Settings Data For Analysis
        // returns all of the settings data for analysis, fully anonymized,
        // but only for accounts who are not hidden from search
        #[ink(message)]
        pub fn get_settings_data(&self) -> SettingsData {
            // set up results structures 
            let mut interests_data: Vec<Vec<u8>> = Vec::new();
            let mut fee_data: Vec<Balance> = Vec::new();
            let mut inbox_access_data: Vec<u64> = Vec::new();
            let mut paid_inbox_data: Vec<u64> = Vec::new();
            let mut settings_update_data: Vec<u64> = Vec::new();

            // iterate on all_accounts_with_settings: Vec<AccountId>
            for acct in &self.all_accounts_with_settings {
                // get their settings in account_settings: Mapping<AccountID, Settings>
                let settings = self.account_settings.get(&acct).unwrap_or_default();
                // if they are not hidden from search...
                if settings.hide_from_search == false {
                    // add the anonymous parts to the results vectors
                    interests_data.push(settings.interests);
                    fee_data.push(settings.inbox_fee);
                    inbox_access_data.push(settings.last_inbox_access);
                    paid_inbox_data.push(settings.last_paid_access);
                    settings_update_data.push(settings.last_update);
                }
            }
            // package the results
            let results = SettingsData {
                interests: interests_data,
                inbox_fee: fee_data,
                last_inbox_access: inbox_access_data,
                last_paid_access: paid_inbox_data,
                last_update:settings_update_data,
            };
            // return the results
            results
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> SECONDARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // no secondary messages

        // END OF MESSAGE LIST

    }
    // END OF CONTRACT STORAGE

}
