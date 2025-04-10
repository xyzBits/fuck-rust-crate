// rust 中，enum 是一种强大的类型，它允许你定义一个类型
// 这个类型可以是几种不同的变体中的一种
// rust 中，enum 的变体可以携带数据，这些数据的形式可以多种多样，包括
// 无数据 None
// 元组形式
// 结构体形式，
// 这些是匿名结构体，因为这些结构体没有显式的定义为 独立 的 struct，而是直接嵌入在enum 的变体中

// clap 提供了两种主要的 API 来定义和解析命令行参数
// Builder API 手动控制来构建命令行
// Derive API 通过 rust 宏来自动生成解析代码，只需要定义一个结构体，clap 会为你处理解析逻辑

use clap::Subcommand;

#[derive(Debug, clap::Parser)] // 通过 clap::Parser 引入，用于自动解析命令行参数
#[command(name = "rnacos")] // 设置应用的元信息
#[command(version, about = "rnacos cli", long_about = None)]
pub struct Cli {
    // 声明子命令
    #[command(subcommand)] // command 用来构建一个命令行实例
    pub command: Option<Commands>,

    #[arg(short, long, default_value = "")] // 属性定义字段对应的参数，支持长短标志和帮助信息
    pub env_file: String,
}

// 定义子命令的 enum
#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help(true))]
    DataToSqlite { file: String, out: String },

    #[command(arg_required_else_help(true))]
    SqliteToData { file: String, out: String },

    #[command(arg_required_else_help(true))]
    MysqlToData { file: String, out: String },

    #[command(arg_required_else_help(true))]
    OpenapiToData {
        #[arg(short, long, default_value = "")]
        username: String,

        #[arg(short, long, default_value = "")]
        password: String,

        host: String,
        out: String,
    },
}
