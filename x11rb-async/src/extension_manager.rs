// This code is dual licensed under MIT OR Apache 2.0.

//! An asynchronous extension manager.

use async_lock::RwLock;
use std::collections::HashMap;

/// An asynchronous extension manager.
pub struct ExtensionManager(RwLock<HashMap<>>);
