use std::fmt;
pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of therads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        ThreadPool
    }
    /// Another way to create a new ThreadPool which uses Result
    /// 
    /// This has the advantage of not causing an unrecoverable state if 0 is passed as the pool size
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError),
            _ => Ok(ThreadPool)
        }
    }
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;
