#!/bin/bash

# 参数说明：
# - "all"：全量输出结果（包括错误和正常输出）
# - "errors"：只输出包含错误的结果
# - 如果参数是某个 example 名称（如 ex2_73），则只运行该 example

arg=${1:-all} # 默认参数为 "all"

run_example() {
    local example=$1
    echo ""
    echo "Running example: $example"
    cargo run --example $example
}

if [ "$arg" == "all" ]; then
    # 全量模式，遍历所有 example 文件
    for example in $(find examples -name "*.rs" | sed 's|examples/||;s|\.rs$||'); do
        echo ""
        echo "Running example: $example"
        cargo run --example $example
    done
elif [ "$arg" == "errors" ]; then
    # 只输出包含错误的结果
    for example in $(find examples -name "*.rs" | sed 's|examples/||;s|\.rs$||'); do
        output=$(cargo run --example $example 2>&1)
        if echo "$output" | grep -qE "error|panic"; then
            echo ""
            echo "Running example failed: $example"
            #echo "$output"
        fi
    done
else
    # 如果参数是 example 名称，则只运行该 example
    run_example "$arg"
fi

echo "Done."
