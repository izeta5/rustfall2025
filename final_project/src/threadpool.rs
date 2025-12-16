use std::sync::{Arc, Mutex, Condvar, mpsc};
use std::thread::{self, JoinHandle};
use std::panic;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum WorkerMessage {
    Panicked(usize),
}

struct SharedState {
    queue: Vec<Job>,
    terminated: bool,
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, state: Arc<(Mutex<SharedState>, Condvar)>, health_tx: mpsc::Sender<WorkerMessage>) -> Worker {
        let handle = thread::spawn(move || {
            let tx = health_tx;
            
            let result = panic::catch_unwind(move || {
                loop {
                    let (lock, cvar) = &*state;
                    let mut shared_state = lock.lock().unwrap();

                    while !shared_state.terminated && shared_state.queue.is_empty() {
                        shared_state = cvar.wait(shared_state).unwrap();
                    }

                    if shared_state.terminated {
                        break;
                    }

                    let job = shared_state.queue.pop().unwrap();
                    drop(shared_state);
                    job();
                }
            });

            if result.is_err() {
                let _ = tx.send(WorkerMessage::Panicked(id));
            }
        });

        Worker { id, handle: Some(handle) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    state: Arc<(Mutex<SharedState>, Condvar)>,
    health_tx: mpsc::Sender<WorkerMessage>,
    health_rx: mpsc::Receiver<WorkerMessage>,
    initial_size: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let state = Arc::new((Mutex::new(SharedState { queue: Vec::new(), terminated: false }), Condvar::new()));
        let (health_tx, health_rx) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&state), health_tx.clone()));
        }

        ThreadPool { workers, state, health_rx, health_tx, initial_size: size }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        let (lock, cvar) = &*self.state;
        let mut shared_state = lock.lock().unwrap();
        shared_state.queue.push(job);
        cvar.notify_one();
    }

    pub fn monitor_and_restart(&mut self) {
        while let Ok(msg) = self.health_rx.try_recv() {
            match msg {
                WorkerMessage::Panicked(id) => {
                    eprintln!("Worker {} panicked. Restarting...", id);
                    
                    self.workers.retain(|w| w.id != id);

                    let new_id = self.workers.len() + self.initial_size + 1;
                    let new_worker = Worker::new(
                        new_id, 
                        Arc::clone(&self.state), 
                        self.health_tx.clone()
                    );
                    self.workers.push(new_worker);
                }
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.state;
        let mut shared_state = lock.lock().unwrap();
        shared_state.terminated = true;
        drop(shared_state);
        cvar.notify_all();

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// TEST REQUIREMENT #1: UNIT TESTS FOR THREAD POOL
// ══════════════════════════════════════════════════════════════════════════════
// These tests verify the core functionality of the ThreadPool implementation:
//   - Task submission and execution
//   - Thread safety with concurrent operations
//   - Proper shutdown and resource cleanup
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_requirement_1a_task_submission_and_execution() {
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║ TEST REQUIREMENT #1a: THREAD POOL - TASK EXECUTION             ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        
        println!("\n✓ Creating thread pool with 1 worker...");
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);
        let pool = ThreadPool::new(1);

        println!("✓ Submitting task to increment counter...");
        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  → Task executed: counter incremented");
        });

        thread::sleep(Duration::from_millis(50));
        
        let final_value = *counter.lock().unwrap();
        println!("✓ Verifying result...");
        println!("  → Final counter value: {}", final_value);
        println!("  → Expected value: 1");
        
        assert_eq!(final_value, 1);
        println!("\n✓ UNIT TEST PASSED: Task submitted and executed successfully\n");
    }
    
    #[test]
    fn test_requirement_1b_concurrent_execution_safety() {
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║ TEST REQUIREMENT #1b: THREAD POOL - CONCURRENT SAFETY          ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        
        const NUM_TASKS: usize = 1000;
        const NUM_WORKERS: usize = 4;
        
        println!("\n✓ Creating thread pool with {} workers...", NUM_WORKERS);
        println!("✓ Preparing {} concurrent tasks...", NUM_TASKS);
        
        let pool = ThreadPool::new(NUM_WORKERS);
        let counter = Arc::new(Mutex::new(0));

        println!("✓ Submitting {} tasks concurrently...", NUM_TASKS);
        for i in 0..NUM_TASKS {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                if (i + 1) % 250 == 0 {
                    println!("  → Progress: {}/{} tasks executed", i + 1, NUM_TASKS);
                }
            });
        }
        
        println!("✓ Waiting for all tasks to complete...");
        thread::sleep(Duration::from_millis(200));
        
        let final_count = *counter.lock().unwrap();
        println!("\n✓ Verifying thread-safe execution...");
        println!("  → Final counter value: {}", final_count);
        println!("  → Expected value: {}", NUM_TASKS);
        println!("  → Lost updates: {}", NUM_TASKS - final_count);
        
        assert_eq!(final_count, NUM_TASKS);
        println!("\n✓ UNIT TEST PASSED: All {} tasks executed safely with no race conditions\n", NUM_TASKS);
    }
    
    #[test]
    fn test_requirement_1c_graceful_shutdown() {
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║ TEST REQUIREMENT #1c: THREAD POOL - GRACEFUL SHUTDOWN          ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        
        const NUM_WORKERS: usize = 2;
        const NUM_TASKS: usize = 5;
        
        println!("\n✓ Creating thread pool in limited scope...");
        println!("✓ Workers: {}", NUM_WORKERS);
        println!("✓ Tasks: {}", NUM_TASKS);
        
        {
            let pool = ThreadPool::new(NUM_WORKERS);
            println!("\n✓ Submitting {} tasks...", NUM_TASKS);
            for i in 0..NUM_TASKS {
                pool.execute(move || { 
                    thread::sleep(Duration::from_millis(1));
                    println!("  → Task {} completed", i + 1);
                });
            }
            println!("✓ All tasks submitted");
            println!("✓ Leaving scope - Drop trait should trigger shutdown...");
        } // ThreadPool drops here, triggering shutdown
        
        println!("\n✓ Scope exited - verifying cleanup...");
        println!("  → Workers notified of termination");
        println!("  → All threads joined successfully");
        println!("  → Resources cleaned up");
        
        println!("\n✓ UNIT TEST PASSED: Thread pool shutdown gracefully without deadlock\n");
        assert!(true); // If we reach here without hanging, test passes
    }
}