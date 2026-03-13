//! AI News Pusher - 主入口文件
//! 跨平台资讯推送系统

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ai_news_pusher_lib::run;

fn main() {
    run();
}
