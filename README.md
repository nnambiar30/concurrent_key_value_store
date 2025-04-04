# Concurrent In-Memory Key-Value Store with TTL Expiration

This project implements a concurrent, in-memory key-value store in Rust. The store supports basic CRUD operations (SET, GET, DELETE) and TTL (time-to-live) expiration logic, allowing you to remove expired entries from the store automatically. The design emphasizes correctness and deterministic behavior by processing a JSON-formatted operation script that simulates time. 

## Overview

The key features of this project are:

- **Basic CRUD Operations:**  
  - **SET:** Add a key-value pair with an optional TTL.
  - **GET:** Retrieve the value for a given key.
  - **DELETE:** Remove a key from the store.
  
- **TTL Expiration:**  
  Each entry can have an optional TTL. A helper function cleans up entries when their TTL reaches 0.

- **Concurrency:**  
  The store is accessed concurrently using Rust threads. Shared data is safely managed with `Arc` and `Mutex` to prevent race conditions.

- **Deterministic Testing:**  
  Operations are driven by a JSON script with simulated time, ensuring that behavior is reproducible across different environments.

## Project Structure

- **src/main.rs:**  
  The main entry point of the application. It reads a JSON file containing a series of operations and processes each one (SET, GET, DELETE).

- **CRUD Functions:**  
  - `set`: Adds an entry to the store.
  - `get`: Searches for an entry concurrently and returns its value.
  - `delete`: Uses concurrent threads to search for and remove an entry.
  - `delete_ttls`: Removes all entries with a TTL of 0.
  - `wait`: A stub function for decrementing TTLs concurrently (to be implemented).

- **JSON Scripting:**  
  The project uses `serde` and `serde_json` to parse a JSON file (e.g., `data/transactions.json`) containing operation objects.
