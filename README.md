# 🧙‍♂️ mc-server-status-use-rust
用 Rust 寫的 Minecraft 伺服器狀態查詢工具。每次 ping 都是靈魂的召喚，每次 debug 都是詩歌的吟唱。

## ✨ 功能特色
查詢 Minecraft Java 版伺服器狀態

顯示 MOTD、玩家人數、版本資訊

CLI 工具，適合整合進 Discord bot、Minebuntu 儀式流程

可選 JSON 輸出，支援 webhook 通知

## 🦀 安裝方式
```bash
git clone https://github.com/hray1413/mc-server-status-use-rust.git
cd mc-server-status-use-rust
cargo build --release
./target/release/mc-server-status-use-rust mc.hypixel.net
```
## 📦 Cargo.toml
```toml
[package]
name = "mc-server-status-use-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
mcquery = "0.3"
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```
## 🧪 使用方式
```bash
cargo run -- mc.hypixel.net
```
或輸出 JSON：

```bash
cargo run -- mc.hypixel.net --json
```
## 🧝‍♂️ Goblin 儀式建議
整合 Discord bot：將 JSON 結果推送至 webhook，作為伺服器儀式心跳

Minebuntu onboarding：新生進入時自動 ping 伺服器，確認靈魂連線

NAS 面板整合：每日定時查詢伺服器狀態，作為 RAID 1 的精神象徵
