import os
import subprocess

def run_examples():
    # 定义 examples 目录路径
    examples_dir = "./examples"

    # 检查目录是否存在
    if not os.path.exists(examples_dir):
        print(f"Directory {examples_dir} does not exist!")
        return

    # 遍历 examples 目录中的所有文件
    for file_name in os.listdir(examples_dir):
        # 检查文件是否以 `.rs` 结尾
        if file_name.endswith(".rs"):
            example_name = file_name[:-3]  # 去掉 `.rs` 扩展名
            print(f"Running example: {example_name}")

            # 使用 `cargo run --example` 命令运行文件
            result = subprocess.run(
                ["cargo", "run", "--example", example_name],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )

            # 打印运行结果
            print(f"Output for {example_name}:")
            print(result.stdout)
            if result.stderr:
                print(f"Errors for {example_name}:")
                print(result.stderr)

if __name__ == "__main__":
    run_examples()
