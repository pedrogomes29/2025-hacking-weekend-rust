use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 1000;

#[derive(CandidType, Deserialize)]
struct Message {
    from: candid::Principal,
    content: String,
}

impl Storable for Message {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

const MESSAGES_MAP_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static MESSAGES_MAP: RefCell<StableBTreeMap<u64, Message, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MESSAGES_MAP_MEMORY_ID))
    ))
}


#[ic_cdk::query]
fn get_messages() -> Vec<Message> {
    let messages = MESSAGES_MAP.with(|m| m.borrow().iter().
    map(|(_,y)|{
        y
    }).collect());
    messages
}

#[ic_cdk::update]
fn send_message(content: String) {
    let message = Message {
        from: ic_cdk::caller(),
        content,
    };

    let id = MESSAGES_MAP.with(|m| m.borrow().len() as u64);
    MESSAGES_MAP.with(|m| m.borrow_mut().insert(id, message));
}
