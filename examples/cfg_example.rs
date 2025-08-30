use match_any::match_any;

#[derive(Debug, PartialEq)]
enum MediaType {
    Video(String),
    Audio(String),
    Image(String),
    Text(String),
}

fn main() {
    use MediaType::*;

    println!("=== 演示 #[cfg] 属性支持 ===");

    let media_files = vec![
        Video("movie.mp4".to_string()),
        Audio("song.mp3".to_string()),
        Image("photo.jpg".to_string()),
        Text("document.txt".to_string()),
    ];

    for media in media_files {
        let result = match_any!(media,
            #[cfg(feature = "video-player")]
            Video(name) => format!("🎬 正在播放视频: {}", name),

            #[cfg(feature = "audio-player")]
            Audio(name) => format!("🎵 正在播放音频: {}", name),

            // 图片处理总是可用的（无 cfg 属性）
            Image(name) => format!("🖼️  正在显示图片: {}", name),

            #[cfg(feature = "text-editor")]
            Text(name) => format!("📝 正在编辑文本: {}", name),

            // 默认情况处理所有不支持的类型
            _ => format!("❓ 不支持的媒体类型")
        );

        println!("{}", result);
    }

    println!();
    println!("=== 复杂 cfg 条件示例 ===");

    let test_media = Video("test.mp4".to_string());
    let result = match_any!(test_media,
        #[cfg(feature = "video-player")]
        Video(name) => format!("视频播放器处理: {}", name),

        #[cfg(feature = "experimental")]
        Image(name) => format!("实验性图片处理: {}", name),

        _ => "使用默认处理器".to_string()
    );

    println!("结果: {}", result);

    println!();
    println!("提示: 尝试使用以下命令启用不同的功能:");
    println!("  cargo run --example cfg_example --features video-player");
    println!("  cargo run --example cfg_example --features audio-player");
    println!("  cargo run --example cfg_example --features video-player,audio-player,text-editor");
}
