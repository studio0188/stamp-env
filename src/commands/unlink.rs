use anyhow::Result;
use std::path::Path;

use crate::symlink::builder::SymlinkBuilder;

pub fn run(target: Option<&str>) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    let builder = SymlinkBuilder::new();
    let count = builder.unlink(target)?;

    println!("'{target_path}'에서 {count}개의 심링크를 제거했습니다.");
    Ok(())
}
