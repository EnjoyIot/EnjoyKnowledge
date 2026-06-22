// EnjoyKnowledge CLI — 入口

mod cli;
mod config;
mod doctor;
mod format;
mod init;
mod knowledge;
mod record;

use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(name = "enjoyknowledge", version, about = "AI 编程场景的知识纪律层")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// 初始化 `EnjoyFlow` 项目
    Init {
        path: Option<String>,
        #[arg(long)]
        scan: bool,
        #[arg(long)]
        describe: Option<String>,
        #[arg(long)]
        link: Option<String>,
        #[arg(long)]
        ai: Option<String>,
    },
    /// 搜索知识库
    Search {
        query: String,
        #[arg(short = 'C', long)]
        class: Vec<String>,
        #[arg(short, long)]
        tag: Vec<String>,
        #[arg(long)]
        archive: bool,
    },
    /// 记录知识 (gotcha / pattern / decision)
    Record {
        #[command(subcommand)]
        sub: RecordCmd,
    },
    /// 诊断知识库健康度
    Doctor {
        #[arg(long)]
        full: bool,
    },
    /// 自动修复问题
    Fix {
        #[arg(long)]
        full: bool,
    },
}

#[derive(clap::Subcommand)]
enum RecordCmd {
    /// 记录踩坑
    Gotcha {
        #[arg(long)]
        task: Option<String>,
        #[arg(short, long)]
        tag: Vec<String>,
        #[arg(short, long)]
        content: String,
    },
    /// 记录最佳实践
    Pattern {
        #[arg(short, long)]
        tag: Vec<String>,
        #[arg(short, long)]
        content: String,
    },
    /// 记录架构决策
    Decision {
        #[arg(long)]
        task: String,
        #[arg(short, long)]
        content: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let root = Path::new(".");

    match cli.command {
        Command::Init { path, scan, describe, link, ai } => {
            cli::init::run(
                path.as_deref(),
                scan,
                describe.as_deref(),
                link.as_deref(),
                ai.as_deref(),
            )?;
        }
        Command::Search { query, class, tag, archive } => {
            cli::search::run(&query, &class, &tag, archive, root)?;
        }
        Command::Record { sub } => match sub {
            RecordCmd::Gotcha { task, tag, content } => {
                cli::record::run_gotcha(task.as_deref(), &tag, &content, root)?;
            }
            RecordCmd::Pattern { tag, content } => {
                cli::record::run_pattern(&tag, &content, root)?;
            }
            RecordCmd::Decision { task, content } => {
                cli::record::run_decision(&task, &content, root)?;
            }
        },
        Command::Doctor { full } => {
            cli::doctor::run_doctor(root, full)?;
        }
        Command::Fix { full } => {
            cli::doctor::run_fix(root, full)?;
        }
    }

    Ok(())
}
