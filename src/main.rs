use mcdk::{get_app_data_path, get_minecraft_data_path, get_games_com_netease_path};

fn main() {
    println!("=== MCDK 环境信息 ===");
    println!("AppData: {:?}", get_app_data_path());
    println!("MinecraftPE_Netease: {:?}", get_minecraft_data_path());
    println!("games/com.netease: {:?}", get_games_com_netease_path());
    println!("\n编译成功！");
}