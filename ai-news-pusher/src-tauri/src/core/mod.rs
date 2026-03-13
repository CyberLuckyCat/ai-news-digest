//! 核心逻辑引擎模块
//!
//! 包含以下子模块：
//! - ai_provider: AI 提供商桥接
//! - collector: 信息采集器
//! - processor: 内容处理器
//! - scheduler: 定时任务调度器
//! - storage: 本地数据存储

pub mod ai_provider;
pub mod collector;
pub mod processor;
pub mod scheduler;
pub mod storage;

pub use ai_provider::*;
pub use collector::*;
pub use processor::*;
pub use scheduler::*;
pub use storage::*;
