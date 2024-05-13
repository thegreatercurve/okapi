use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

fn sort_array(value: Value) -> Result<Value, serde_json::Error> {
    let list = value.as_array().map(|x| x.to_owned()).unwrap();
    let mut new_list = vec![];
    for json_value in list.into_iter() {
        new_list.push(sort_json_keys(json_value)?)
    }
    Ok(serde_json::to_value(new_list)?)
}

fn sort_object_keys(value: Value) -> Result<Value, serde_json::Error> {
    let map = value.as_object().map(|x| x.to_owned()).unwrap();
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        new_map.insert(key, sort_json_keys(value)?);
    }
    let btree_map: BTreeMap<_, _> = new_map.iter().collect();
    Ok(serde_json::to_value(btree_map)?)
}

pub(crate) fn sort_json_keys(value: Value) -> Result<Value, serde_json::Error> {
    if value.is_object() {
        return sort_object_keys(value);
    }

    if value.is_array() {
        return sort_array(value);
    }

    Ok(value)
}
