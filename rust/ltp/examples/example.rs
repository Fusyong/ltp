/// LTP Rust 用例示例
///
/// 这个示例展示了如何使用 LTP Rust 库进行中文自然语言处理，
/// 包括分词（CWS）、词性标注（POS）和命名实体识别（NER）。
///
/// 使用方法：
///   cargo run --example example --features serialization,parallel
///
/// 或者指定输入文本：
///   cargo run --example example --features serialization,parallel -- "他叫汤姆去拿外衣。"
///
/// 或者从文件读取：
///   cargo run --example example --features serialization,parallel -- --input input.txt

use itertools::multizip;
use ltp::{CWSModel, ModelSerde, NERModel, POSModel, Reader};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

/// 模型文件路径配置
struct ModelPaths {
    cws: String,
    pos: String,
    ner: String,
}

impl Default for ModelPaths {
    fn default() -> Self {
        // 从 rust/ltp 目录到 LTP/legacy 的相对路径
        // cargo run --example 会在 rust/ltp 目录下运行
        ModelPaths {
            cws: "../../data/LTP/legacy/cws_model.bin".to_string(),
            pos: "../../data/LTP/legacy/pos_model.bin".to_string(),
            ner: "../../data/LTP/legacy/ner_model.bin".to_string(),
        }
    }
}

impl ModelPaths {
    /// 尝试查找模型文件，支持多个可能的路径
    fn find_models() -> Option<Self> {
        let possible_paths = vec![
            // 从 rust/ltp 目录运行
            ("../../data/LTP/legacy/", "cws_model.bin"),
            // 从项目根目录运行
            ("data/LTP/legacy/", "cws_model.bin"),
            // 传统路径
            ("data/legacy-models/", "cws_model.bin"),
        ];

        for (base, filename) in possible_paths {
            let cws_path = format!("{}{}", base, filename);
            if Path::new(&cws_path).exists() {
                return Some(ModelPaths {
                    cws: cws_path,
                    pos: format!("{}pos_model.bin", base),
                    ner: format!("{}ner_model.bin", base),
                });
            }
        }
        None
    }
}

/// 加载所有模型
fn load_models(paths: &ModelPaths) -> Result<(CWSModel, POSModel, NERModel), Box<dyn std::error::Error>> {
    println!("正在加载模型...");
    let start = Instant::now();

    // 加载分词模型
    println!("  加载分词模型: {}", paths.cws);
    let cws_file = File::open(&paths.cws)?;
    let cws_reader = Reader::new(cws_file)?;
    let cws: CWSModel = ModelSerde::load_avro(cws_reader)?;

    // 加载词性标注模型
    println!("  加载词性标注模型: {}", paths.pos);
    let pos_file = File::open(&paths.pos)?;
    let pos_reader = Reader::new(pos_file)?;
    let pos: POSModel = ModelSerde::load_avro(pos_reader)?;

    // 加载命名实体识别模型
    println!("  加载命名实体识别模型: {}", paths.ner);
    let ner_file = File::open(&paths.ner)?;
    let ner_reader = Reader::new(ner_file)?;
    let ner: NERModel = ModelSerde::load_avro(ner_reader)?;

    let duration = start.elapsed();
    println!("模型加载完成，耗时: {:.2}秒\n", duration.as_secs_f64());

    Ok((cws, pos, ner))
}

/// 处理单个句子
fn process_sentence(
    sentence: &str,
    cws: &CWSModel,
    pos: &POSModel,
    ner: &NERModel,
) -> Result<(), Box<dyn std::error::Error>> {
    if sentence.trim().is_empty() {
        return Ok(());
    }

    let start = Instant::now();

    // 分词
    let words = cws.predict(sentence)?;

    // 词性标注（需要分词结果）
    let pos_tags = pos.predict(&words)?;

    // 命名实体识别（需要分词和词性标注结果）
    let ner_tags = ner.predict((&words, &pos_tags))?;

    let duration = start.elapsed();

    // 输出结果
    println!("原文: {}", sentence);
    println!("结果:");
    for (w, p, n) in multizip((words, pos_tags, ner_tags)) {
        if n != "O" {
            // 如果是命名实体，用特殊格式显示
            println!("  {}/{}/[{}]", w, p, n);
        } else {
            println!("  {}/{}", w, p);
        }
    }
    println!("处理耗时: {:.2}毫秒\n", duration.as_millis());

    Ok(())
}

/// 处理多个句子
fn process_sentences(
    sentences: Vec<String>,
    cws: &CWSModel,
    pos: &POSModel,
    ner: &NERModel,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();
    let count = sentences.len();

    for (i, sentence) in sentences.iter().enumerate() {
        println!("=== 句子 {}/{} ===", i + 1, count);
        process_sentence(sentence, cws, pos, ner)?;
    }

    let total_duration = total_start.elapsed();
    println!("总共处理 {} 个句子，总耗时: {:.2}秒，平均: {:.2}毫秒/句",
             count,
             total_duration.as_secs_f64(),
             total_duration.as_millis() as f64 / count as f64);

    Ok(())
}

/// 从文件读取句子
fn read_sentences_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sentences: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect();
    Ok(sentences)
}

/// 打印使用说明
fn print_usage() {
    println!("LTP Rust 用例示例");
    println!();
    println!("使用方法:");
    println!("  cargo run --example example --features serialization,parallel");
    println!("  cargo run --example example --features serialization,parallel -- \"输入文本\"");
    println!("  cargo run --example example --features serialization,parallel -- --input <文件路径>");
    println!();
    println!("示例:");
    println!("  cargo run --example example --features serialization,parallel -- \"他叫汤姆去拿外衣。\"");
    println!("  cargo run --example example --features serialization,parallel -- --input input.txt");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 尝试查找模型文件
    let model_paths = match ModelPaths::find_models() {
        Some(paths) => paths,
        None => {
            eprintln!("错误: 找不到模型文件！");
            eprintln!();
            eprintln!("请确保已下载 Legacy 模型文件并放在以下任一位置:");
            eprintln!("  - data/LTP/legacy/ (推荐)");
            eprintln!("  - data/legacy-models/");
            eprintln!();
            eprintln!("模型文件包括:");
            eprintln!("  - cws_model.bin (分词模型)");
            eprintln!("  - pos_model.bin (词性标注模型)");
            eprintln!("  - ner_model.bin (命名实体识别模型)");
            eprintln!();
            eprintln!("可以从以下地址下载:");
            eprintln!("  https://huggingface.co/LTP/legacy");
            eprintln!("  或镜像: https://hf-mirror.com/LTP/legacy");
            return Ok(());
        }
    };

    println!("使用模型路径:");
    println!("  - {}", model_paths.cws);
    println!("  - {}", model_paths.pos);
    println!("  - {}", model_paths.ner);
    println!();

    // 加载模型
    let (cws, pos, ner) = load_models(&model_paths)?;

    // 获取命令行参数
    let args: Vec<String> = env::args().skip(1).collect();

    let sentences = if args.is_empty() {
        // 没有参数，使用默认示例
        vec!["美狄亚这一天很不幸。美狄娅的奶奶从太阳神庙回来后就病倒了。（太阳神庙现在是全国重点文物保护单位——不叫全国文物重点保护单位。".to_string()]
    } else if args.len() == 1 && args[0] == "--help" || args[0] == "-h" {
        print_usage();
        return Ok(());
    } else if args.len() == 2 && args[0] == "--input" || args[0] == "-i" {
        // 从文件读取
        let file_path = &args[1];
        println!("从文件读取: {}", file_path);
        read_sentences_from_file(file_path)?
    } else {
        // 直接使用命令行参数作为输入文本
        args
    };

    // 处理句子
    process_sentences(sentences, &cws, &pos, &ner)?;

    Ok(())
}
