#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_cdk::caller;
use validator::Validate;
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::borrow::Borrow;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SmartStorageItem {
    id: u64,
    owner: String,
    name: String,
    description: String,
    location: String,
    created_at: u64,
    updated_at: Option<u64>,
    is_available: bool,
}

impl Storable for SmartStorageItem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SmartStorageItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE_ITEM_STORAGE: RefCell<StableBTreeMap<u64, SmartStorageItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Validate)]
struct SmartStorageItemPayload {
    #[validate(length(min = 3))]
    name: String,
    #[validate(length(min = 5))]
    description: String,
    #[validate(length(min = 2))]
    location: String,
    is_available: bool,
}

// function to get an item
#[ic_cdk::query]
fn get_smart_storage_item(id: u64) -> Result<SmartStorageItem, Error> {
    match _get_smart_storage_item(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

// function to get all items
#[ic_cdk::query]
fn get_all_smart_storage_items() -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// function to get only available items
#[ic_cdk::query]
fn get_available_smart_storage_items() -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, item)| item.is_available)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// function to search for items matching a specific query
#[ic_cdk::query]
fn search_smart_storage_items(query: String) -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, item)| item.name.contains(&query) || item.description.contains(&query))
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// function to add an item to the canister
#[ic_cdk::update]
fn add_smart_storage_item(item: SmartStorageItemPayload) -> Result<SmartStorageItem, Error> {
    // validates payload
    let check_payload = _check_input(&item);
    // if input validations fail for payload, return an error
    if check_payload.is_err(){
        return Err(check_payload.err().unwrap());
    }
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let storage_item = SmartStorageItem {
        id,
        owner: caller().to_string(),
        name: item.name,
        description: item.description,
        location: item.location,
        created_at: time(),
        updated_at: None,
        is_available: item.is_available,
    };
    // save item
    do_insert_smart_storage_item(&storage_item);
    Ok(storage_item)
}

#[ic_cdk::update]
fn update_smart_storage_item(id: u64, payload: SmartStorageItemPayload) -> Result<SmartStorageItem, Error> {
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(mut item) => {
            // Ensures that only the owner of an item can update
            if !_check_if_owner(&item) {
                return Err(Error::NotOwner {msg: format!("Caller={} is not owner of item with id={}.", caller().to_string(), id)})
            }
            // Validates payload
            let check_payload = _check_input(&payload);
            // Returns an error if validations failed
            if check_payload.is_err(){
                return Err(check_payload.err().unwrap());
            }
            item.name = payload.name;
            item.description = payload.description;
            item.location = payload.location;
            item.updated_at = Some(time());
            item.is_available = payload.is_available;
            
            // save item
            do_insert_smart_storage_item(&item);

            Ok(item.clone())
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update an item with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn is_item_available(id: u64) -> Result<bool, Error> {
    match _get_smart_storage_item(&id) {
        Some(item) => Ok(item.is_available),
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn mark_item_as_available(id: u64) -> Result<SmartStorageItem, Error> {
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(mut item) => {
            // Ensures only the owner of an item can update its availability
            if !_check_if_owner(&item) {
                return Err(Error::NotOwner {msg: format!("Caller={} is not owner of item with id={}.", caller().to_string(), id)})
            }
            item.is_available = true;
            // save item
            do_insert_smart_storage_item(&item);
            Ok(item.clone())
        }
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn mark_item_as_unavailable(id: u64) -> Result<SmartStorageItem, Error> {
    if let Some(mut item) = STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        // Ensures only the owner of an item can update its availability
        if !_check_if_owner(&item) {
            return Err(Error::NotOwner {msg: format!("Caller={} is not owner of item with id={}.", caller().to_string(), id)})
        }
        item.is_available = false;
        // save item
        do_insert_smart_storage_item(&item);
        Ok(item.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        })
    }
}

fn do_insert_smart_storage_item(item: &SmartStorageItem) {
    STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().insert(item.id, item.clone()));
}

#[ic_cdk::update]
fn delete_smart_storage_item(id: u64) -> Result<SmartStorageItem, Error> {
    let smart_storage_item = _get_smart_storage_item(&id);
    // returns an error if item doesn't exist
    if smart_storage_item.is_none(){
        return Err(Error::NotFound {
            msg: format!(
                "couldn't delete an item with id={}. item not found.",
                id
            ),
        })
    }
    // Ensures only the owner of the item can delete it
    if !_check_if_owner(&smart_storage_item.unwrap()) {
        return Err(Error::NotOwner {msg: format!("Caller={} is not owner of item with id={}.", caller().to_string(), id)})
    }
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete an item with id={}. item not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotOwner {msg: String},
    ValidationFailed {content: String}
}

fn _get_smart_storage_item(id: &u64) -> Option<SmartStorageItem> {
    // Assuming MemoryId::new(1) is reserved for smart storage item storage
    let item_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    StableBTreeMap::<u64, SmartStorageItem, Memory>::init(item_storage)
        .borrow()
        .get(id)
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct ChangeRecord {
    timestamp: u64,
    change_type: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ItemStatistics {
    total_items: usize,
    average_availability_rate: f64,
}

// function to return all items in an orderly manner based off their names
#[ic_cdk::query]
fn sort_items_by_name() -> Vec<SmartStorageItem> {
    let mut items = STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect::<Vec<_>>()
    });
    // sorts items based off their names
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

#[ic_cdk::query]
fn get_item_history(id: u64) -> Vec<ChangeRecord> {
    match _get_smart_storage_item(&id) {
        Some(item) => {
            let mut history = Vec::new();
            if let Some(updated_at) = item.updated_at {
                history.push(ChangeRecord {
                    timestamp: updated_at,
                    change_type: "Update".to_string(),
                });
            }
            history.push(ChangeRecord {
                timestamp: item.created_at,
                change_type: "Creation".to_string(),
            });
            history
        }
        None => Vec::new(),
    }
}

#[ic_cdk::query]
fn batch_query(queries: Vec<Query>) -> Vec<QueryResult> {
    let mut results = Vec::new();
    for query in queries {
        match query {
            Query::GetItem(id) => {
                if let Some(item) = _get_smart_storage_item(&id) {
                    results.push(QueryResult::Item(item));
                } else {
                    results.push(QueryResult::Error(Error::NotFound {
                        msg: format!("an item with id={} not found", id),
                    }));
                }
            }
        }
    }
    results
}

#[derive(candid::CandidType, Serialize, Deserialize)]
enum Query {
    GetItem(u64),
}

#[derive(candid::CandidType, Serialize, Deserialize)]
enum QueryResult {
    Item(SmartStorageItem),
    Error(Error),
}

#[ic_cdk::query]
fn get_item_statistics() -> ItemStatistics {
    let items = STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect::<Vec<_>>()
    });

    let total_items = items.len();
    let total_available_items = items.iter().filter(|item| item.is_available).count();
    let average_availability_rate =
        total_available_items as f64 / total_items as f64 * 100.0; // Calculate as a percentage

    ItemStatistics {
        total_items,
        average_availability_rate,
    }
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct TransactionRecord {
    timestamp: u64,
    change_type: String, // Add this line
    transaction_type: String,
}

#[ic_cdk::query]
fn get_item_transaction_history(id: u64) -> Vec<TransactionRecord> {
    match _get_smart_storage_item(&id) {
        Some(item) => {
            let mut history = Vec::new();
            if let Some(updated_at) = item.updated_at {
                history.push(TransactionRecord {
                    timestamp: updated_at,
                    change_type: "Update".to_string(),
                    transaction_type: "Update".to_string(),
                });
            }
            history.push(TransactionRecord {
                timestamp: item.created_at,
                change_type: "Creation".to_string(),
                transaction_type: "Creation".to_string(),
            });
            history
        }
        None => Vec::new(),
    }
}


#[ic_cdk::update]
fn bulk_update_smart_storage_items(updates: Vec<(u64, SmartStorageItemPayload)>) -> Vec<Result<SmartStorageItem, Error>> {
    let mut results = Vec::new();
    for (id, payload) in updates {
        let result = update_smart_storage_item(id, payload);
        results.push(result);
    }
    results
}

#[ic_cdk::query]
fn get_paginated_smart_storage_items(limit: usize, offset: usize) -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// Helper function to check whether the caller is the author of the blog post
fn _check_if_owner(smart_storage_item: &SmartStorageItem) -> bool {
    if smart_storage_item.owner.to_string() != caller().to_string(){
        false  
    }else{
        true
    }
}

// Helper function to check the input data of the payload
fn _check_input(payload: &SmartStorageItemPayload) -> Result<(), Error> {
    let check_payload = payload.validate();
    if check_payload.is_err() {
        return Err(Error::ValidationFailed { content: check_payload.err().unwrap().to_string()})
    }else{
        Ok(())
    }
}


ic_cdk::export_candid!();
