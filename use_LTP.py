import torch
from ltp import LTP
import time

# 默认 huggingface 下载，可能需要代理

start_time = time.time()
# ltp = LTP("LTP/small")  # 默认加载 Small 模型
ltp = LTP("data/LTP/base1")  # 默认加载 Small 模型
                        # 也可以传入模型的路径，ltp = LTP("/path/to/your/model")
                        # /path/to/your/model 应当存在 config.json 和其他模型文件

# 将模型移动到 GPU 上
if torch.cuda.is_available():
    # ltp.cuda()
    ltp.to("cuda")

# 自定义词表
ltp.add_word("汤姆去", freq=2)
ltp.add_words(["外套", "外衣"], freq=2)

#  分词 cws、词性 pos、命名实体标注 ner、语义角色标注 srl、依存句法分析 dep、语义依存分析树 sdp、语义依存分析图 sdpg
output = ltp.pipeline(["美狄亚这一天很不幸。美狄娅的奶奶从太阳神庙回来后就病倒了。（太阳神庙现在是全国重点文物保护单位——不叫全国文物重点保护单位。"], tasks=["cws", "pos", "ner", "srl", "dep", "sdp", "sdpg"])
# 使用字典格式作为返回结果
print(output.cws)  # print(output[0]) / print(output['cws']) # 也可以使用下标访问
print(output.pos)
print(output.sdp)
end_time = time.time()
print(f"LTP base1耗时：{end_time - start_time}秒")


#
# 使用感知机算法实现的分词、词性和命名实体识别，速度比较快，但是精度略低
#
start_time = time.time()
ltp = LTP("data/LTP/legacy")
# cws, pos, ner = ltp.pipeline(["他叫汤姆去拿外衣。"], tasks=["cws", "ner"]).to_tuple() # error: NER 需要 词性标注任务的结果
cws, pos, ner = ltp.pipeline(["美狄亚这一天很不幸。美狄娅的奶奶从太阳神庙回来后就病倒了。（太阳神庙现在是全国重点文物保护单位——不叫全国文物重点保护单位。"], tasks=["cws", "pos", "ner"]).to_tuple()  # to tuple 可以自动转换为元组格式
# 使用元组格式作为返回结果
print(cws, pos, ner)
end_time = time.time()
print(f"LTP legacy耗时：{end_time - start_time}秒")
