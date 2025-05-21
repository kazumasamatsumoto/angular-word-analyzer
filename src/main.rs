use clap::Parser;
use std::{ fs, path::Path };
use walkdir::{ DirEntry, WalkDir };

/// キーワード検索ツール
#[derive(Parser)]
#[command(author, version, about = "フォルダ内のHTMLからキーワードを検索します", long_about = None)]
struct Args {
    /// 検索対象フォルダ
    folder: String,

    /// 検索キーワード
    keyword: String,
}

/// 除外したいディレクトリ名
fn is_excluded(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() &&
        matches!(
            entry.file_name().to_string_lossy().as_ref(),
            "node_modules" | ".vscode" | ".angular" | ".git"
        )
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root = Path::new(&args.folder);
    let keyword = &args.keyword;

    // WalkDir で再帰走査しながら filter_entry で除外
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded(e))
        .filter_map(Result::ok) {
        let path = entry.path();
        // ファイルかつ拡張子が .html のものだけ
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext == "html" || ext == "ts" {
                    let content = fs::read_to_string(path)?;
                    if content.contains(keyword) {
                        println!("{}", path.display());
                    }
                }
            }
        }
    }

    Ok(())
}
