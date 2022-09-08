use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;

#[tokio::main]
async fn main() {
    let mut value = 78_u128;
    let atomic_ptr = Arc::new(AtomicPtr::new(&mut value));

    assert_eq!(unsafe { *atomic_ptr.load(Relaxed) }, value);

    let atomic_ptr_clone = atomic_ptr.clone();
    spawn(move ||{
        assert_eq!(unsafe { *atomic_ptr_clone.load(Relaxed) }, value);
    }).join().unwrap();

    let atomic_ptr_clone = atomic_ptr.clone();
    tokio::spawn(async move {
        assert_eq!(unsafe { *atomic_ptr_clone.load(Relaxed) }, value);
    }).await.unwrap();
}