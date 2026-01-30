use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod preset;
mod symlink;

#[derive(Parser)]
#[command(name = "stamp")]
#[command(about = "프리셋 기반 폴더/파일 구조를 심링크로 배포하는 CLI 도구")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 프리셋을 대상 경로에 심링크로 생성
    Link {
        /// 적용할 프리셋 이름
        preset: String,
        /// 대상 경로 (기본: 현재 디렉토리)
        target: Option<String>,
        /// 확인 없이 진행
        #[arg(short, long)]
        yes: bool,
    },
    /// 생성된 심링크 제거
    Unlink {
        /// 대상 경로 (기본: 현재 디렉토리)
        target: Option<String>,
    },
    /// 현재 구조를 프리셋으로 저장
    Commit {
        /// 프리셋 이름
        name: String,
        /// Glob 패턴으로 파일 필터 (예: "*.rs", "src/**/*.toml")
        #[arg(short, long)]
        patterns: Option<Vec<String>>,
        /// 이미 link된 위치들에 변경사항 동기화
        #[arg(short, long)]
        sync: bool,
    },
    /// 저장된 프리셋 목록
    List,
    /// 프리셋 내용 확인
    Show {
        /// 확인할 프리셋 이름
        preset: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Link { preset, target, yes } => {
            commands::link::run(&preset, target.as_deref(), yes)?;
        }
        Commands::Unlink { target } => {
            commands::unlink::run(target.as_deref())?;
        }
        Commands::Commit {
            name,
            patterns,
            sync,
        } => {
            commands::commit::run(&name, patterns.as_deref(), sync)?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Show { preset } => {
            commands::list::show(&preset)?;
        }
    }

    Ok(())
}
