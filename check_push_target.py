#!/usr/bin/env python3
"""
检查 Git 推送目标的脚本
"""
import subprocess
import sys

def run_git_command(cmd):
    """执行 git 命令并返回结果"""
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            encoding='utf-8'
        )
        return result.stdout.strip(), result.returncode
    except Exception as e:
        return f"错误: {e}", 1

def main():
    print("=" * 60)
    print("Git 推送目标检查")
    print("=" * 60)

    # 检查当前分支
    branch, _ = run_git_command("git branch --show-current")
    print(f"\n当前分支: {branch}")

    # 检查远程仓库
    print("\n远程仓库配置:")
    remotes, _ = run_git_command("git remote -v")
    print(remotes)

    # 检查分支跟踪关系
    print("\n分支跟踪关系:")
    tracking, _ = run_git_command("git branch -vv")
    print(tracking)

    # 检查当前分支跟踪的远程
    tracking_remote, _ = run_git_command(f"git config branch.{branch}.remote")
    tracking_merge, _ = run_git_command(f"git config branch.{branch}.merge")

    if tracking_remote:
        remote_url, _ = run_git_command(f"git config remote.{tracking_remote}.url")
        print(f"\n当前分支 '{branch}' 跟踪: {tracking_remote}")
        print(f"远程 URL: {remote_url}")
        print(f"\n⚠️  执行 'git push' 会推送到: {remote_url}")
    else:
        print(f"\n⚠️  分支 '{branch}' 没有设置跟踪远程分支")
        print("   执行 'git push' 需要明确指定远程和分支")

    # 检查是否有未推送的提交
    status, _ = run_git_command("git status -sb")
    if "ahead" in status:
        print(f"\n📤 有未推送的提交")
        print(f"   状态: {status}")
    else:
        print(f"\n✅ 没有未推送的提交")

    print("\n" + "=" * 60)

if __name__ == "__main__":
    main()

