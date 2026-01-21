use image::{Luma};
use imageproc::contrast::adaptive_threshold;
use std::env;
use std::path::Path;

fn main() {
    // 1. 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 如果用户没有输入路径（比如直接双击运行）
    if args.len() < 2 {
        println!("❌ 错误: 请输入图片路径！");
        println!("用法: watermark_extractor.exe <图片路径>");
        println!("或者直接将图片拖到此 exe 文件上。");
        // 为了防止双击时黑窗口闪退，让它停一下
        wait_for_keypress();
        return;
    }

    // 2. 获取并清理输入路径
    let input_raw = &args[1];
    let input_path_str = input_raw.trim().trim_matches('"').trim_matches('\'');
    let input_path = Path::new(input_path_str);

    if !input_path.exists() {
        println!("❌ 错误: 找不到文件 {:?}", input_path_str);
        wait_for_keypress();
        return;
    }

    // 3. 根据输入文件名生成输出路径
    // 例如: "./input/1.png" -> "1_processed.png"
    let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let output_name = format!("{}_processed.png", file_stem);
    
    // 默认保存在程序运行的目录下，或者你可以改为 input_path.with_file_name(...)
    let output_path = Path::new(&output_name);

    println!("📖 正在读取: {:?}", input_path);

    // 4. 执行你原本生效的逻辑
    let img = image::open(input_path).expect("无法打开图片");
    let gray_img = img.to_luma8();

    println!("⚙️ 正在提取水印...");
    let adaptive = adaptive_threshold(&gray_img, 10);

    // 5. 保存结果
    match adaptive.save(output_path) {
        Ok(_) => println!("✅ 处理完成！结果已保存为: {}", output_name),
        Err(e) => println!("❌ 保存失败: {}", e),
    }

    println!("按回车键退出...");
    wait_for_keypress();
}

// 辅助函数：防止程序运行完立刻关闭窗口
fn wait_for_keypress() {
    use std::io::{self, Read};
    let mut _unused = [0u8; 1];
    let _ = io::stdin().read(&mut _unused);
}