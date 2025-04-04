use serde::Deserialize;
use std::fs;
use serde_json;
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Debug, Deserialize)]
struct Operation {
    op: String,
    key: Option<String>,
    secs: Option<u64>,
    value: Option<String>,
    ttl: Option<u64>,
}

fn get(key: &mut String, store: &mut Vec<(String, String, Option<u64>)>) -> Result<String, String> {

    let store_owned = store.clone();
    let size = store.len();
    let store_arc = Arc::new(store_owned);

    let data = Arc::new(Mutex::new(None));
    let mut handles = vec![];
    
    for i in 0..size {
        let store_clone = Arc::clone(&store_arc);
        let data_clone = Arc::clone(&data);
        let key_clone = key.clone();

        let handle = thread::spawn(move || {
            if store_clone[i].0 == key_clone {
                let mut result = data_clone.lock().unwrap();
                *result = Some(store_clone[i].clone());
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let x = if let Some(result) = &*data.lock().unwrap() {
        Ok(result.1.clone())
    } else  {
        Err(format!("{} doesn't exist in store", key))
    };
    x
} 

fn set(entry: (String, String, Option<u64>), store: &mut Vec<(String, String, Option<u64>)>) {
    store.push(entry);
} 

fn delete(key: &mut String, store: &mut Vec<(String, String, Option<u64>)>) -> Result<String, String> {
    let store_owned = store.clone();
    let size = store.len();
    let store_arc = Arc::new(store_owned);

    let data = Arc::new(Mutex::new(None));
    let mut handles = vec![];
    
    for i in 0..size {
        let store_clone = Arc::clone(&store_arc);
        let data_clone = Arc::clone(&data);
        let key_clone = key.clone();

        let handle = thread::spawn(move || {
            if store_clone[i].0 == key_clone {
                let mut result = data_clone.lock().unwrap();
                *result = Some(i);
                if result.is_none() {
                    *result = Some(i);
                }
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let maybe_index = data.lock().unwrap();
    if let Some(index) = *maybe_index {
        store.remove(index);
        Ok("successful!".to_string())
    } else  {
        Err(format!("{} doesn't exist in store", key))
    }
} 

fn wait(time: u64, store: &mut Vec<(String, String, Option<u64>)>) {
    let store_clone = store.clone();
    let size = store.len();
    let store_arc = Arc::new(Mutex::new(store_clone));
    let mut handles = vec![];

    for i in 0..size {
        let store_arc_clone = Arc::clone(&store_arc);

        let handle = thread::spawn(move || {
            let mut store_guard = store_arc_clone.lock().unwrap();

            if let Some(ttl) = store_guard[i].2 {
                let new_ttl = if ttl > time { ttl - time } else { 0 };
                store_guard[i].2 = Some(new_ttl);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let updated_store = Arc::try_unwrap(store_arc)
    .expect("Multiple references still exist")
    .into_inner()
    .unwrap();

    // Write the updated store back into the original mutable reference.
    *store = updated_store;
}

fn delete_ttls(store: &mut Vec<(String, String, Option<u64>)>) {
    store.retain(|(_key, _value, ttl)| {
        match ttl {
            Some(ttl_val) if *ttl_val == 0 => false, // remove this element
            _ => true, // keep this element
        }
    });
}

fn main() {

    let data = fs::read_to_string("data/transactions.json").expect("Unable to read file");

    let operations: Vec<Operation> = serde_json::from_str(&data).expect("JSON was not well formatted");

    let mut store: Vec<(String, String, Option<u64>)> = Vec::new();

    for op in operations {
        match op.op.as_str() {
            "SET" => { 
                let Operation { op, key, secs, value, ttl} = op;
                set((key.clone().unwrap(), value.clone().unwrap(), ttl), &mut store); 
                match ttl {
                    Some(ttl_val) => { println!("SET {} {} ttl: {}", key.unwrap(), value.unwrap(), ttl_val); }
                    None => { println!("SET {} {}", key.unwrap(), value.unwrap()); }
                }
                
            },
            "GET" => {
                let Operation { op, key, secs, value, ttl} = op;
                let result = get(&mut key.clone().unwrap(), &mut store);
                match result {
                    Ok(val) => { println!("GET of key {} is {:?} ", key.unwrap(), val); }
                    Err(e) => {println!("Warning {}", e);}
                }

            }   
            "DELETE"=> {
                let Operation { op, key, secs, value, ttl} = op;
                let result = delete(&mut key.clone().unwrap(), &mut store);
                match result {
                    Ok(val) => {println!("DELETE of key {} was {}", key.unwrap(), val);}
                    Err(e) => {println!("Warning {}", e);}
                }
                
            },

            "WAIT" => {
                let Operation { op, key, secs, value, ttl} = op;
                wait(secs.clone().unwrap(), &mut store);
                println!("WAIT advanaced time by {}", secs.unwrap());
                delete_ttls(&mut store);
            },

            _ => {}
        }
    }
}
