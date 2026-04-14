use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dashmap::DashMap;
use parking_lot::RwLock;
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs},
    Frame, Terminal,
};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::{env, io, process};

// ============================================================================
// Constants & Configuration
// ============================================================================

const MAX_MEMPOOL_SIZE: usize = 100_000;
const MAX_ACCOUNT_TXS: usize = 16;
const TX_TIMEOUT_SECS: u64 = 300;
const TARGET_TPS: usize = 15_000;
const BLOCK_GAS_LIMIT: u64 = 30_000_000;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct TxHash([u8; 32]);

impl TxHash {
    fn to_hex(&self) -> String {
        hex::encode(&self.0[..4])
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    hash: TxHash,
    from: String,
    to: String,
    gas_price: u64, // Gwei
    gas_limit: u64,
    size: u64, // bytes
    nonce: u64,
    timestamp: Instant,
}

impl Transaction {
    // For priority queue: higher gas price = better
    // BTreeSet sorts in ascending order by default, so we reverse the key
    fn priority_key(&self) -> (ReverseGas, TxHash) {
        (ReverseGas(self.gas_price), self.hash.clone())
    }
}

// Wrapper to reverse gas price ordering for BTreeSet
// #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[derive(Debug, Clone, Ord)]
struct ReverseGas(u64);
impl Eq for ReverseGas {}
impl PartialEq for ReverseGas {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for ReverseGas {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|o| o.reverse())
    }
}

// ============================================================================
// Mempool Core (Lock-Free / Sharded)
// ============================================================================

pub struct Mempool {
    // Sharded HashMap for high-concurrency inserts (DashMap handles sharding internally)
    transactions: DashMap<TxHash, Transaction>,
    
    // Per-account tracking (DashMap allows concurrent access per account)
    account_txs: DashMap<String, BTreeSet<TxHash>>,

    // Global Priority Queue (Protected by RwLock for batch ops)
    // Stores (GasPrice, Hash) to keep it sorted by fee
    priority_index: RwLock<BTreeSet<(ReverseGas, TxHash)>>,

    // Metrics
    total_added: AtomicU64,
    total_evicted: AtomicU64,
    total_rejected: AtomicU64,
    total_mined: AtomicU64,
    current_size: AtomicU64,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: DashMap::new(),
            account_txs: DashMap::new(),
            priority_index: RwLock::new(BTreeSet::new()),
            total_added: AtomicU64::new(0),
            total_evicted: AtomicU64::new(0),
            total_rejected: AtomicU64::new(0),
            total_mined: AtomicU64::new(0),
            current_size: AtomicU64::new(0),
        }
    }

    /// High-performance insert
    pub fn add_tx(&self, tx: Transaction) -> bool {
        // 1. Check account limit
        let mut account_entry = self.account_txs.entry(tx.from.clone()).or_insert_with(BTreeSet::new);
        
        // DashMap entry returns a reference guard
        if account_entry.len() >= MAX_ACCOUNT_TXS {
            self.total_rejected.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        
        // 2. Check capacity & Evict if necessary
        if self.transactions.len() >= MAX_MEMPOOL_SIZE {
            // Evict lowest fee transaction
            if !self.evict_lowest() {
                self.total_rejected.fetch_add(1, Ordering::Relaxed);
                return false;
            }
        }

        // 3. Insert into main storage
        let hash = tx.hash.clone();
        let size = tx.size;
        
        // Insert into account set
        account_entry.insert(hash.clone());
        // Drop the guard to release the lock on that specific account shard
        drop(account_entry);

        // Insert into global maps
        self.transactions.insert(hash.clone(), tx.clone());
        self.priority_index.write().insert(tx.priority_key());
        
        // Metrics
        self.total_added.fetch_add(1, Ordering::Relaxed);
        self.current_size.fetch_add(size, Ordering::Relaxed);
        true
    }

    /// Evict the worst transaction (lowest fee)
    fn evict_lowest(&self) -> bool {
        let mut priority = self.priority_index.write();
        
        // Get lowest fee tx (first element in set because of ReverseGas sorting)
        if let Some(first) = priority.iter().next().cloned() {
            priority.remove(&first);
            drop(priority); // Release lock quickly

            let (_, hash) = first;
            
            if let Some((_, tx)) = self.transactions.remove(&hash) {
                self.current_size.fetch_sub(tx.size, Ordering::Relaxed);
                
                // Remove from account map
                if let Some(mut acc_set) = self.account_txs.get_mut(&tx.from) {
                    acc_set.remove(&hash);
                }
                
                self.total_evicted.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Mine a block (Get best transactions)
    pub fn mine_block(&self, max_gas: u64) -> Vec<Transaction> {
        let mut block = Vec::new();
        let mut gas_used = 0;
        let mut to_remove = Vec::new();

        let priority = self.priority_index.read();
        // Iterate from high gas price to low (reverse iterator)
        for (rev_gas, hash) in priority.iter().rev() {
            if gas_used + 21_000 > max_gas { break; } // Simplified gas calc
            
            if let Some(tx) = self.transactions.get(hash) {
                gas_used += 21_000;
                block.push(tx.clone());
                to_remove.push((rev_gas.clone(), hash.clone()));
            }
        }
        drop(priority);

        // Remove mined txs
        let mut priority = self.priority_index.write();
        for key in to_remove {
            priority.remove(&key);
            self.transactions.remove(&key.1);
        }

        let mined_count = block.len() as u64;
        self.total_mined.fetch_add(mined_count, Ordering::Relaxed);
        block
    }

    pub fn stats(&self) -> (usize, u64, u64, u64, u64, u64) {
        (
            self.transactions.len(),
            self.total_added.load(Ordering::Relaxed),
            self.total_evicted.load(Ordering::Relaxed),
            self.total_rejected.load(Ordering::Relaxed),
            self.total_mined.load(Ordering::Relaxed),
            self.current_size.load(Ordering::Relaxed),
        )
    }
    
    pub fn get_top_txs(&self, n: usize) -> Vec<Transaction> {
        let priority = self.priority_index.read();
        priority.iter().rev()
            .take(n)
            .filter_map(|(_, h)| self.transactions.get(h).map(|r| r.clone()))
            .collect()
    }
}

// ============================================================================
// Simulation Engine
// ============================================================================

pub struct Simulator {
    mempool: Mempool,
    tps_counter: AtomicU64,
    last_tick: Instant,
    target_tps: usize,
}

impl Simulator {
    pub fn new(target_tps: usize) -> Self {
        Self {
            mempool: Mempool::new(),
            tps_counter: AtomicU64::new(0),
            last_tick: Instant::now(),
            target_tps,
        }
    }

    // Generate random transaction
    fn gen_tx(i: usize) -> Transaction {
        let mut rng = rand::thread_rng();
        let mut hasher = Sha256::new();
        hasher.update(format!("tx-{}-{}", i, rng.gen::<u64>()));
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);

        Transaction {
            hash: TxHash(hash_bytes),
            from: format!("0x{:040x}", rng.gen::<u32>()),
            to: format!("0x{:040x}", rng.gen::<u32>()),
            gas_price: rng.gen_range(20..200),
            gas_limit: 21_000,
            size: rng.gen_range(200..1000),
            nonce: rng.gen(),
            timestamp: Instant::now(),
        }
    }

    // Run simulation loop
    pub async fn run(&self) {
        let mempool = self.mempool.clone(); // DashMap is Arc-based internally, cheap clone
        let tps_counter = self.tps_counter.clone();
        let target = self.target_tps;

        // Task 1: Transaction Producer (Multi-threaded via Rayon)
        tokio::spawn(async move {
            let batch_size = 1000;
            loop {
                // Generate batch in parallel using Rayon
                let txs: Vec<Transaction> = (0..batch_size)
                    .into_par_iter()
                    .map(|i| Simulator::gen_tx(i))
                    .collect();

                // Insert serially (or use sharded parallel insert if lock contention becomes high, 
                // but DashMap handles concurrent write methods well)
                for tx in txs {
                    mempool.add_tx(tx);
                    tps_counter.fetch_add(1, Ordering::Relaxed);
                }
                
                // Sleep to throttle to target TPS
                let sleep_ms = (batch_size as f64 / target as f64 * 1000.0) as u64;
                if sleep_ms > 0 {
                    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
                }
            }
        });

        // Task 2: Block Producer (Every 2 seconds)
        let mp_mine = self.mempool.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(2)).await;
                mp_mine.mine_block(BLOCK_GAS_LIMIT);
            }
        });
    }
}

// Need Clone for Mempool to pass to threads
impl Clone for Mempool {
    fn clone(&self) -> Self {
        Self {
            transactions: self.transactions.clone(),
            account_txs: self.account_txs.clone(),
            priority_index: self.priority_index.clone(),
            total_added: AtomicU64::new(self.total_added.load(Ordering::Relaxed)),
            total_evicted: AtomicU64::new(self.total_evicted.load(Ordering::Relaxed)),
            total_rejected: AtomicU64::new(self.total_rejected.load(Ordering::Relaxed)),
            total_mined: AtomicU64::new(self.total_mined.load(Ordering::Relaxed)),
            current_size: AtomicU64::new(self.current_size.load(Ordering::Relaxed)),
        }
    }
}

// ============================================================================
// Terminal UI (Ratatui)
// ============================================================================

struct App {
    simulator: Simulator,
    current_tps: u64,
    peak_tps: u64,
}

impl App {
    fn new(target_tps: usize) -> Self {
        Self {
            simulator: Simulator::new(target_tps),
            current_tps: 0,
            peak_tps: 0,
        }
    }

    fn update_tps(&mut self) {
        let count = self.simulator.tps_counter.swap(0, Ordering::Relaxed);
        // Smooth the TPS calculation
        self.current_tps = (self.current_tps * 4 + count) / 5;
        if self.current_tps > self.peak_tps {
            self.peak_tps = self.current_tps;
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(7), // Stats
            Constraint::Min(10),   // Table
            Constraint::Length(3), // Footer
        ].as_ref())
        .split(f.size());

    // Header
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" ⚡ RUST MEMPOOL SIMULATOR ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(format!("| Target: {:?} tx/s", app.simulator.target_tps)),
    ])).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(title, chunks[0]);

    // Stats
    let (tx_count, added, evicted, rejected, mined, size) = app.simulator.mempool.stats();
    let stats_text = vec![
        Line::from(vec![
            Span::styled("TPS: ", Style::default().fg(Color::White)),
            Span::styled(format!("{:>6}", app.current_tps), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("Peak: ", Style::default().fg(Color::White)),
            Span::styled(format!("{:>6}", app.peak_tps), Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled("Pending: ", Style::default().fg(Color::White)),
            Span::styled(format!("{:>5}", tx_count), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("Added: {:?} | Mined: {:?} | Evicted: {:?} | Rejected: {:?}", added, mined, evicted, rejected)),
        Line::from(format!("Mempool Size: {:.2} MB", size as f64 / 1_000_000.0)),
    ];
    
    let stats_block = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Metrics"));
    f.render_widget(stats_block, chunks[1]);

    // Table
    let top_txs = app.simulator.mempool.get_top_txs(10);
    let rows: Vec<Row> = top_txs.iter().map(|tx| {
        Row::new(vec![
            tx.hash.to_hex(),
            tx.from[2..8].to_string(),
            format!("{} Gwei", tx.gas_price),
            format!("{} bytes", tx.size),
        ]).style(Style::default().fg(Color::White))
    }).collect();

    let table = Table::new(rows)
        .header(Row::new(vec!["Hash", "From", "Gas Price", "Size"])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(Block::default().title("Top Priority Transactions").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
        ]);
    f.render_widget(table, chunks[2]);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

// ============================================================================
// Main Entry
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Args
    let args: Vec<String> = env::args().collect();
    let target_tps = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10_000);

    // App
    let mut app = App::new(target_tps);
    app.simulator.run().await; // Start background tasks

    // Main Loop
    let mut last_update = Instant::now();
    loop {
        // Draw
        terminal.draw(|f| ui(f, &app))?;

        // Event Handling
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Update Metrics
        if last_update.elapsed() >= Duration::from_millis(200) {
            app.update_tps();
            last_update = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    
    println!("Simulation ended. Peak TPS: {}", app.peak_tps);
    Ok(())
}