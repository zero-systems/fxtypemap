use fxhash::FxHashMap;
use std::any::{Any, TypeId};

/// TypeID based map for storing different types
#[derive(Default)]
pub struct TypeMap {
    map: FxHashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    /// Create new empty map (wrapper to default)
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a value to map
    pub fn insert<T: 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }

    /// Get immutable reference to entry
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Get mutable reference to entry
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }

    /// Check if map contains specific type
    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Remove entry from map
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    /// Clear whole map
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }
}

#[test]
fn test_mut() {
    let mut map = TypeMap::new();

    map.insert(String::from("Hello, "));

    map.get_mut::<String>()
        .map(|v| v.push_str("world!"))
        .unwrap();

    assert_eq!("Hello, world!", map.get::<String>().unwrap());
}

#[test]
fn test_remove() {
    let mut map = TypeMap::new();

    map.insert::<i8>(123);
    assert!(map.get::<i8>().is_some());

    map.remove::<i8>();
    assert!(!map.get::<i8>().is_some());
}

#[test]
fn test_clear() {
    let mut map = TypeMap::new();

    map.insert::<i8>(8);
    map.insert::<i16>(16);
    map.insert::<i32>(32);

    assert!(map.contains::<i8>());
    assert!(map.contains::<i16>());
    assert!(map.contains::<i32>());

    map.clear();

    assert!(!map.contains::<i8>());
    assert!(!map.contains::<i16>());
    assert!(!map.contains::<i32>());

    map.insert::<i8>(10);
    assert_eq!(*map.get::<i8>().unwrap(), 10);
}

#[test]
fn test_integers() {
    let mut map = TypeMap::new();

    map.insert::<i8>(8);
    assert!(map.get::<i8>().is_some());

    map.insert::<i16>(16);
    assert!(map.get::<i16>().is_some());

    map.insert::<i32>(32);
    assert!(map.get::<i32>().is_some());

    map.insert::<i64>(64);
    assert!(map.get::<i64>().is_some());

    map.insert::<i128>(128);
    assert!(map.get::<i128>().is_some());

    map.insert::<u8>(8);
    assert!(map.get::<i8>().is_some());

    map.insert::<u16>(16);
    assert!(map.get::<i16>().is_some());

    map.insert::<u32>(32);
    assert!(map.get::<i32>().is_some());

    map.insert::<u64>(64);
    assert!(map.get::<i64>().is_some());

    map.insert::<u128>(128);
    assert!(map.get::<i128>().is_some());
}

#[test]
fn test_composition() {
    struct Magi<T>(pub T);

    struct Madoka {
        pub is_rei: bool,
    }

    struct Homura {
        pub attempts: usize,
    }

    struct Mami {
        pub guns: usize,
    }

    let mut map = TypeMap::new();

    map.insert(Magi(Madoka { is_rei: false }));
    map.insert(Magi(Homura { attempts: 0 }));
    map.insert(Magi(Mami { guns: 999 }));

    assert_eq!(false, map.get::<Magi<Madoka>>().unwrap().0.is_rei);
    assert_eq!(0, map.get::<Magi<Homura>>().unwrap().0.attempts);
    assert_eq!(999, map.get::<Magi<Mami>>().unwrap().0.guns);
}
