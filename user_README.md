## TODO

* [ ] 检查相似实体名称，找出错误的命名（个别字词有异）
    * [ ] 计算相似性
    * [ ] 绘制关系图

## [LTP 4](https://github.com/HIT-SCIR/ltp)

```shell
cd rust/ltp
cargo run --example example --features serialization,parallel
# 或者指定文本
cargo run --example example --features serialization,parallel -- "他叫汤姆去拿外衣。"
```

```shell
# 查看当前分支的跟踪关系
git branch -vv

# 1. 推送到你的 fork（这是默认行为）
git push
# 或
git push origin main

# 2. 同步原始库的最新更改
git fetch upstream
git merge upstream/main

# 3. 在 GitHub 上创建 Pull Request 向原始库
# 推送到原始库（通常没有权限，除非你是维护者）
git push upstream main
```

最佳实践

* 向原始库贡献：先推送到你的 fork，然后在 GitHub 上创建 Pull Request
* 检查推送目标：使用 git push --dry-run 查看会推送到哪里
* 同步原始库更新：
```shell
   git fetch upstream
   git merge upstream/main
   # 或使用 rebase
   git rebase upstream/main
```
