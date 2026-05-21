use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    pub enabled: bool,
    pub nodes: Vec<NodeConfig>,
    pub replication_factor: usize,
    pub consensus_method: ConsensusMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub is_leader: bool,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMethod {
    Raft,
    Paxos,
    ByzantineFaultTolerant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedState {
    pub current_leader: Option<String>,
    pub active_nodes: Vec<String>,
    pub node_metrics: HashMap<String, NodeMetrics>,
    pub term: u64,
    pub last_heartbeat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub node_id: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub requests_handled: u64,
    pub latency_ms: f64,
    pub last_updated: u64,
}

impl Default for DistributedState {
    fn default() -> Self {
        Self {
            current_leader: None,
            active_nodes: Vec::new(),
            node_metrics: HashMap::new(),
            term: 0,
            last_heartbeat: 0,
        }
    }
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            nodes: Vec::new(),
            replication_factor: 3,
            consensus_method: ConsensusMethod::Raft,
        }
    }
}

pub struct DistributedManager {
    state: Arc<RwLock<DistributedState>>,
    config: Arc<RwLock<DistributedConfig>>,
}

impl DistributedManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(DistributedState::default())),
            config: Arc::new(RwLock::new(DistributedConfig::default())),
        }
    }

    pub async fn initialize(&self, config: DistributedConfig) -> Result<(), String> {
        let mut cfg = self.config.write().await;
        *cfg = config.clone();
        
        let mut state = self.state.write().await;
        state.active_nodes = config.nodes.iter().map(|n| n.id.clone()).collect();
        
        Ok(())
    }

    pub async fn get_state(&self) -> DistributedState {
        self.state.read().await.clone()
    }

    pub async fn get_config(&self) -> DistributedConfig {
        self.config.read().await.clone()
    }

    pub async fn select_leader(&self) -> Result<String, String> {
        let config = self.config.read().await;
        
        if config.nodes.is_empty() {
            return Err("No nodes available".to_string());
        }
        
        let leader = config.nodes.iter()
            .find(|n| n.is_leader)
            .map(|n| n.id.clone())
            .unwrap_or_else(|| config.nodes[0].id.clone());
        
        let mut state = self.state.write().await;
        state.current_leader = Some(leader.clone());
        state.term += 1;
        
        Ok(leader)
    }

    pub async fn update_node_metrics(&self, node_id: String, metrics: NodeMetrics) {
        let mut state = self.state.write().await;
        state.node_metrics.insert(node_id, metrics);
    }

    pub async fn health_check(&self) -> bool {
        let state = self.state.read().await;
        let config = self.config.read().await;
        
        let active_count = state.active_nodes.len();
        let total_nodes = config.nodes.len();
        
        if total_nodes == 0 {
            return false;
        }
        
        let health_ratio = active_count as f64 / total_nodes as f64;
        health_ratio >= 0.5
    }
}

impl Default for DistributedManager {
    fn default() -> Self {
        Self::new()
    }
}
