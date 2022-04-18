// Dependencies:
// memcache

use memcache::MemcacheError;

pub struct Cache {
    client: memcache::Client,
}

impl Cache {
    /// Connects to the memcache server and flushes cache from the previous server run.
    /// The default memcache port is 11211. If the server is not running on this port, the thread will hang, and throw
    /// a MemcacheError after 5 seconds.
    pub fn new() -> Result<Cache, MemcacheError> {
        // Connect to the memcache server. Default port is 11211.
        let client = memcache::connect("memcache://127.0.0.1:11211?timeout=5&tcp_nodelay=true")?;
        // Cache should be cleared of the previous session's data.
        client.flush()?;
        Ok(Cache {
            client,
        })
    }

    /// Stores a given key-value pair in the memcache. This operation should never fail, but if it does, it will return
    /// a MemcacheError.
    /// 
    /// # Arguments
    /// * 'ip' - The ip to store.
    /// * 'data' - The corresponding data to store for that ip. Either "rejected" if the connection was denied, or the
    /// data that allows the server to reload a page without actually connecting to it.
    pub fn store(&self, ip: &str, data: &str) -> Result<(), MemcacheError> {
        self.client.set(ip, data, 0)
    }

    /// Attempts to retrieve the data associated with the given ip supplied. If None is returned, then either the IP is
    /// not in the cache, or a MemcacheError occurred when attempting to obtain it.
    /// 
    /// # Arguments
    /// * 'ip' - The ip the cache will be searched for.
    pub fn retrieve(&self, ip: &str) -> Option<String> {
        let safety_check: Result<Option<String>, MemcacheError> = self.client.get(ip);
        match safety_check {
            Ok(safety_check) => safety_check,
            _ => None,
        }
    }
}