use std::{borrow::Borrow, collections::{btree_map, BTreeMap}, hash::Hasher, ops::Deref};

use serde::de;

use crate::error::Error;

use super::value::{KeyValue, Value};
use std::hash::Hash;

type MapImpl<K, V> = BTreeMap<K, V>;
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, KeyValue, Value>;
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, KeyValue, Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}

impl Map<KeyValue, Value> {
    /// 空のMapを新しく作る
    /// 
    /// return: Map
    #[inline]
    pub fn new() -> Self {
        Self {
            map: MapImpl::new(),
        }
    }

    /// キャパを指定して新しくMapを作る
    /// 
    /// capacity: usize
    /// 
    /// return: Map
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let _ = capacity;
        Self {
            // BTreeMap はキャパを指定できないから無視
            map: MapImpl::new(),
        }
    }

    /// Mapをクリアする
    /// 
    /// return: ()
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Keyに対応するValueを参照で取る
    /// 
    /// key: &KeyValue
    /// 
    /// return: Option<&Value>
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        KeyValue: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get(key)
    }

    /// Key が存在するか確認する
    /// 
    /// key: &KeyValue
    /// 
    /// return: bool
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        KeyValue: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }

    /// Keyに一致するKeyとValueのペアを参照で取る
    /// 
    /// key: &Q
    /// 
    /// return: Option<(&KeyValue, &Value)>
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&KeyValue, &Value)>
    where
        KeyValue: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_key_value(key)
    }

    /// Mapに要素を挿入する
    /// 
    /// key: KeyValue
    /// 
    /// value: Value // 上書きされた時の古い値です
    #[inline]
    pub fn insert(&mut self, key: KeyValue, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }

    /// Keyに一致する要素を抜き取る
    /// 
    /// key: &Q
    /// 
    /// return: Option<Value> // 削除された値
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        KeyValue: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.remove(key)
    }

    /// Keyに一致する要素
    /// 
    /// key: &Q
    /// 
    /// return: Option<(KeyValue, Value)> // 削除された値のペア
    #[inline]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(KeyValue, Value)>
    where
        KeyValue: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.remove_entry(key)
    }

    /// ほかのMapのすべての要素をこのMapにmoveする
    /// 
    /// other: &mut Self
    /// 
    /// return: ()
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.map.append(&mut other.map);
    }


    /// Keyに一致する要素を取得する
    /// 
    /// key: &Q
    /// 
    /// 
    pub fn entry<S>(&mut self, key: S) -> Entry
    where
        S: Into<KeyValue>,
    {
        match self.map.entry(key.into()) {
            btree_map::Entry::Vacant(v) => Entry::Vacant(VacantEntry { vacant: v }),
            btree_map::Entry::Occupied(o) => Entry::Occupied(OccupiedEntry { occupied: o }),
            
        }
    }
}

impl Hash for Map<KeyValue, Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.hash(state);
    }
}

impl Deref for Map<KeyValue, Value> {
    type Target = MapImpl<KeyValue, Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

// impl<'de> de::Deserialize<'de> for Map<KeyValue, Value> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct MapVisitor;

//         impl<'de> de::Visitor<'de> for MapVisitor {
//             type Value = Map<KeyValue, Value>;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("a map")
//             }

//             #[inline]
//             fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
//             where
//                 V: de::MapAccess<'de>,
//             {
//                 let mut values = MapImpl::new();

//                 while let Some((key, value)) = map.next_entry()? {
//                     values.insert(key, value);
//                 }

//                 Ok(Map { map: values })
//             }
//         }

//         deserializer.deserialize_map(MapVisitor)
//     }
// }

// impl<'de> de::IntoDeserializer<'de, Error> for Map<KeyValue, Value> {
//     type Deserializer = Self;

//     fn into_deserializer(self) -> Self {
//         self
//     }
// }

// impl<'de> de::IntoDeserializer<'de, Error> for &'de Map<KeyValue, Value> {
//     type Deserializer = Self;

//     fn into_deserializer(self) -> Self {
//         self
//     }
// }

pub struct VacantEntry<'a> {
    vacant: VacantEntryImpl<'a>,
}

impl<'a> VacantEntry<'a> {
    #[inline]
    pub fn key(&self) -> &KeyValue {
        self.vacant.key()
    }
    
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}

pub struct OccupiedEntry<'a> {
    occupied: OccupiedEntryImpl<'a>,
}

impl<'a> OccupiedEntry<'a> {
    #[inline]
    pub fn key(&self) -> &KeyValue {
        self.occupied.key()
    }

    #[inline]
    pub fn get(&self) -> &Value {
        self.occupied.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {
        self.occupied.get_mut()
    }

    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }

    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }

    #[inline]
    pub fn remove(self) -> Value {
        self.occupied.remove()
    }

    #[inline]
    pub fn remove_entry(self) -> (KeyValue, Value) {
        self.occupied.remove_entry()
    }
    
}

pub enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}

impl<'a> Entry<'a> {
    pub fn key(&self) -> &KeyValue {
        match self {
            Entry::Vacant(v) => v.key(),
            Entry::Occupied(o) => o.key(),
        }
    }

    pub fn or_insert(self, value: Value) -> &'a mut Value {
        match self {
            Entry::Vacant(v) => v.insert(value),
            Entry::Occupied(o) => o.into_mut(),
        }
    }

    pub fn or_insert_with<F>(self, f: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(v) => v.insert(f()),
            Entry::Occupied(o) => o.into_mut(),
        }
    }

    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Value),
    {
        match self {
            Entry::Vacant(v) => Entry::Vacant(v),
            Entry::Occupied(mut o) => {
                f(o.get_mut());
                Entry::Occupied(o)
            }
        }
    }
}