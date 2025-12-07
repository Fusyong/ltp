# LTP Rust 用例示例说明

这个示例展示了如何使用 LTP Rust 库进行中文自然语言处理，包括：
- 中文分词（CWS - Chinese Word Segmentation）
- 词性标注（POS - Part-of-Speech Tagging）
- 命名实体识别（NER - Named Entity Recognition）

## 前置要求

### 1. 下载模型文件

在使用之前，需要下载 Legacy 模型文件。可以从以下地址下载：

- 官方地址：https://huggingface.co/LTP/legacy
- 镜像地址：https://hf-mirror.com/LTP/legacy

下载后，将模型文件放到以下任一位置（推荐使用第一个）：

```
LTP/
└── legacy/
    ├── cws_model.bin    # 分词模型
    ├── pos_model.bin    # 词性标注模型
    └── ner_model.bin    # 命名实体识别模型
```

或者：

```
data/
└── legacy-models/
    ├── cws_model.bin    # 分词模型
    ├── pos_model.bin    # 词性标注模型
    └── ner_model.bin    # 命名实体识别模型
```

**注意**：程序会自动查找模型文件，支持多个可能的路径。

### 2. 安装依赖

确保已安装 Rust 和 Cargo。然后运行：

```bash
cd rust/ltp
cargo build --features serialization,parallel
```

## 使用方法

### 方法 1：使用默认示例文本

```bash
cargo run --example example --features serialization,parallel
```

这将使用默认的示例文本："他叫汤姆去拿外衣。"

### 方法 2：指定输入文本

```bash
cargo run --example example --features serialization,parallel -- "美狄亚这一天很不幸。"
```

### 方法 3：从文件读取

首先创建一个文本文件（例如 `input.txt`），每行一个句子：

```
他叫汤姆去拿外衣。
美狄亚这一天很不幸。
美狄娅的奶奶从太阳神庙回来后就病倒了。
```

然后运行：

```bash
cargo run --example example --features serialization,parallel -- --input input.txt
```

### 方法 4：查看帮助

```bash
cargo run --example example --features serialization,parallel -- --help
```

## 输出格式

示例程序会输出以下信息：

1. **模型加载信息**：显示加载进度和耗时
2. **处理结果**：对每个句子显示：
   - 原文
   - 分词、词性标注和命名实体识别结果
   - 处理耗时

输出示例：

```
使用模型路径:
  - ../../LTP/legacy/cws_model.bin
  - ../../LTP/legacy/pos_model.bin
  - ../../LTP/legacy/ner_model.bin

正在加载模型...
  加载分词模型: ../../LTP/legacy/cws_model.bin
  加载词性标注模型: ../../LTP/legacy/pos_model.bin
  加载命名实体识别模型: ../../LTP/legacy/ner_model.bin
模型加载完成，耗时: 0.15秒

=== 句子 1/1 ===
原文: 他叫汤姆去拿外衣。
结果:
  他/r
  叫/v
  汤姆/nh/[B-PER]
  去/v
  拿/v
  外衣/n
  。/wp
处理耗时: 2.50毫秒

总共处理 1 个句子，总耗时: 0.15秒，平均: 2.50毫秒/句
```

## 代码说明

主要功能模块：

- `load_models()`: 加载三个模型文件
- `process_sentence()`: 处理单个句子
- `process_sentences()`: 批量处理多个句子
- `read_sentences_from_file()`: 从文件读取句子

## 注意事项

1. 模型文件较大，首次加载需要一些时间
2. 命名实体识别（NER）需要先进行分词和词性标注
3. 如果模型文件不存在，程序会显示错误信息并提示下载地址
4. 程序会自动查找模型文件，支持多个可能的路径（`LTP/legacy/`、`data/legacy-models/` 等）

## 与 Python 版本的对比

这个 Rust 示例对应 Python 版本的以下代码：

```python
from ltp import LTP

ltp = LTP("LTP/legacy")
cws, pos, ner = ltp.pipeline(["他叫汤姆去拿外衣。"], tasks=["cws", "pos", "ner"]).to_tuple()
print(cws, pos, ner)
```

Rust 版本的优势：
- 更快的执行速度
- 更低的内存占用
- 更好的并发性能（支持多线程）
