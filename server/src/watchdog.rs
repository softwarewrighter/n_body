use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Watchdog that monitors simulation health and detects hung computations
pub struct SimulationWatchdog {
    last_frame: Arc<AtomicU64>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl SimulationWatchdog {
    pub fn new() -> Self {
        SimulationWatchdog {
            last_frame: Arc::new(AtomicU64::new(0)),
            running: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    /// Update the watchdog with the current frame number
    pub fn heartbeat(&self, frame_number: u64) {
        self.last_frame.store(frame_number, Ordering::Relaxed);
    }

    /// Start the watchdog thread
    pub fn start(&self, timeout_seconds: u64) {
        let last_frame = Arc::clone(&self.last_frame);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            let mut last_seen_frame = 0u64;
            let mut stall_start: Option<Instant> = None;

            while running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));

                let current_frame = last_frame.load(Ordering::Relaxed);

                if current_frame == last_seen_frame {
                    // Simulation appears stalled
                    if let Some(start) = stall_start {
                        let stall_duration = start.elapsed().as_secs();

                        if stall_duration >= timeout_seconds {
                            log::error!(
                                "WATCHDOG: Simulation hung for {} seconds at frame {}! \
                                Server may be overloaded. Consider restarting or reducing particle count.",
                                stall_duration,
                                current_frame
                            );

                            // Log every 30 seconds during hang
                            if stall_duration % 30 == 0 {
                                log::error!(
                                    "WATCHDOG: Still hung after {} seconds. Manual intervention required.",
                                    stall_duration
                                );
                            }
                        } else if stall_duration >= 5 {
                            log::warn!(
                                "WATCHDOG: Simulation stalled for {} seconds at frame {}",
                                stall_duration,
                                current_frame
                            );
                        }
                    } else {
                        stall_start = Some(Instant::now());
                    }
                } else {
                    // Simulation is progressing
                    if let Some(start) = stall_start {
                        let stall_duration = start.elapsed().as_secs();
                        if stall_duration >= 5 {
                            log::info!(
                                "WATCHDOG: Simulation recovered after {} second stall",
                                stall_duration
                            );
                        }
                    }
                    stall_start = None;
                    last_seen_frame = current_frame;
                }
            }

            log::info!("Watchdog thread shutting down");
        });
    }

    /// Stop the watchdog thread
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Drop for SimulationWatchdog {
    fn drop(&mut self) {
        self.stop();
    }
}
