// 执行此文件的话使用该命令
// cargo run --bin true
// 如果需要隐藏堆栈信息，可以加 --quite

fn main() {
    // 进程退出 0 表示成功，大于 0 的其它数字表示失败，一般用 1 表示
    std::process::exit(0);
    // std::process::exit(1);
    // 或者用下面这种方式进行失败退出
    // std::process::abort();
}
