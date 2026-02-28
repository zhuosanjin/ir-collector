use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "主机侧应急响应信息采集工具",
    long_about = None
)]
struct Args {
    /// 输出格式
    #[arg(long, default_value = "json")]
    output: OutputFormat,

    /// 输出到文件，不指定则打印到stdout
    #[arg(long)]
    out_file: Option<String>,

    /// 指定采集模块，逗号分隔：process,network,user,persistence,fs 或 all
    #[arg(long, default_value = "all")]
    modules: String,

    /// 文件系统模块：查最近n小时修改的文件
    #[arg(long, default_value_t = 24)]
    fs_hours: u8,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Markdown,
    Html,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Module {
    All,
    Process,
    Network,
    User,
    Persistence,
    Fs,
}

fn parse_modules(input: &str) -> Result<Vec<Module>, String> {
    input
        .split(',')
        .map(|s| match s.trim() {
            "all" => Ok(Module::All),
            "process" => Ok(Module::Process),
            "network" => Ok(Module::Network),
            "user" => Ok(Module::User),
            "persistence" => Ok(Module::Persistence),
            "fs" => Ok(Module::Fs),
            other => Err(format!(
                "未知模块: '{}'，可选值: all,process,network,user,persistence,fs",
                other
            )),
        })
        .collect()
}
// let args = Args::parse();

// let modules = match parse_modules(&args.modules) {
//     Ok(m) => m,
//     Err(e) => {
//         eprintln!("错误：{}", e);
//         std::process::exit(1);
//     }
// };

// // modules里如果包含All，就展开成全部
// let modules = if modules.contains(&Module::All) {
//     vec![
//         Module::Process,
//         Module::Network,
//         Module::User,
//         Module::Persistence,
//         Module::Fs,
//     ]
// } else {
//     modules
// };

// println!("{:?}", modules);
//
