use chrono::Local;
use tauri_plugin_log::{log::LevelFilter, RotationStrategy, Target, TargetKind};

pub fn init_log_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_log::Builder::new()
        .targets([
            Target::new(TargetKind::LogDir {
                file_name: Some("info".to_string()),
            })
            .filter(|metadata| metadata.level() <= LevelFilter::Info),

            Target::new(TargetKind::LogDir {
                file_name: Some("error".to_string()),
            })
            .filter(|metadata| metadata.level() <= LevelFilter::Error)
            
        ])
        .level(LevelFilter::Debug)
        .max_file_size(50000 /* bytes */)
        .rotation_strategy(RotationStrategy::KeepAll)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}-{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .build()
}
