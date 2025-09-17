use std::env;
use mcquery::McQuery;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法：mc_status <伺服器 IP>");
        return;
    }

    let server_ip = &args[1];
    match McQuery::new(server_ip, 25565).get_status() {
        Ok(status) => {
            println!("✅ 伺服器在線：{}", server_ip);
            println!("📝 MOTD：{}", status.motd);
            println!("👥 玩家：{}/{}", status.players_online, status.players_max);
            println!("🎮 版本：{}", status.version_name);
        }
        Err(e) => {
            println!("❌ 無法連線到伺服器：{}", server_ip);
            println!("錯誤：{}", e);
        }
    }
}
