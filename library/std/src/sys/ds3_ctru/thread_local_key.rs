use crate::collections::BTreeMap;
use crate::sync::atomic::{AtomicUsize, Ordering};
use crate::ptr::null_mut;

pub type Key = usize;

#[thread_local]
static mut DATA: Option<BTreeMap<Key, *mut u8>> = None;

static KEY_GEN: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let key = KEY_GEN.fetch_add(1, Ordering::SeqCst);

    let data = DATA.get_or_insert_with(|| Default::default());
    data.insert(key, null_mut());

    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let data = DATA.get_or_insert_with(|| Default::default());
    data.insert(key, value);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let data = DATA.get_or_insert_with(|| Default::default());
    data.get(&key).copied().unwrap_or(null_mut())
}

#[inline]
pub unsafe fn destroy(key: Key) {
    let data = DATA.get_or_insert_with(|| Default::default());
    data.remove(&key);
}

#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}
