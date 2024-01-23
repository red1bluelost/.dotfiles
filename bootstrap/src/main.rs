use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[cfg(not(unix))]
compile_error!("non-unix environment not yet supported");

fn home_dir() -> anyhow::Result<&'static Path> {
    lazy_static::lazy_static! {
        static ref HOME_DIR: Option<PathBuf> = home::home_dir();
    }
    Ok(HOME_DIR
        .as_deref()
        .ok_or(anyhow::anyhow!("cannot find home directory"))?)
}

fn cwd() -> anyhow::Result<&'static Path> {
    lazy_static::lazy_static! {
        static ref CWD: io::Result<PathBuf> = env::current_dir();
    }
    Ok(CWD.as_deref()?)
}

fn ensure_parent(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let path_parent = path.parent().ok_or(anyhow::anyhow!(
        "file destination ({}) is missing a parent",
        path.display()
    ))?;
    fs::create_dir_all(path_parent)?;
    Ok(())
}

fn link(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    println!("Linking {} ...", path.display());
    let src = cwd()?.join(path);
    let dst = home_dir()?.join(path);
    ensure_parent(&dst)?;
    if dst.exists() {
        fs::remove_file(&dst)?;
    }
    fs::hard_link(src, dst)?;
    Ok(())
}

fn plug_download() -> anyhow::Result<()> {
    let file = "plug.vim";
    let mut file_path = home_dir()?.to_owned();
    file_path.push(".local");
    file_path.push("share");
    file_path.push("nvim");
    file_path.push("site");
    file_path.push("autoload");
    file_path.push(file);
    ensure_parent(&file_path)?;
    let mut config_file = File::create(&file_path)?;

    let mut handle = curl::easy::Easy::new();
    handle.url(&format!(
        "https://raw.githubusercontent.com/junegunn/vim-plug/master/{}",
        file
    ))?;

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        config_file.write_all(data).map_or_else(
            |err| {
                eprintln!("error writing data to file");
                eprintln!("file: {}", file_path.display());
                eprintln!("error: {}", err);
                Ok(0)
            },
            |()| Ok(data.len()),
        )
    })?;
    transfer.perform()?;
    Ok(())
}

fn llvm_download_file(prefix: &str, file: &str) -> anyhow::Result<()> {
    let file_path = home_dir()?.join(&*CONFIG_NVIM_PATH).join(file);
    ensure_parent(&file_path)?;
    let mut config_file = File::create(&file_path)?;

    let mut handle = curl::easy::Easy::new();
    handle.url(&format!(
        "https://raw.githubusercontent.com/llvm/llvm-project/main/{}/{}",
        prefix, file
    ))?;

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        config_file.write_all(data).map_or_else(
            |err| {
                eprintln!("error writing data to file");
                eprintln!("file: {}", file_path.display());
                eprintln!("error: {}", err);
                Ok(0)
            },
            |()| Ok(data.len()),
        )
    })?;
    transfer.perform()?;
    Ok(())
}

lazy_static! {
    static ref CONFIG_NVIM_PATH: &'static Path = Path::new(".config/nvim");
}

const LINK_PATHS: &[&str] = &[".gitconfig", ".bashrc"];

const CONFIG_NVIM_LINK_PATHS: &[&str] = &[
    "init.vim",
    "coc-settings.json",
    "lua/config.lua",
];

const LLVM_VIM_PATHS: &[&str] = &[
    "ftdetect/llvm-lit.vim",
    "ftdetect/llvm.vim",
    "ftdetect/mir.vim",
    "ftdetect/tablegen.vim",
    "ftplugin/llvm.vim",
    "ftplugin/mir.vim",
    "ftplugin/tablegen.vim",
    "indent/llvm.vim",
    "syntax/llvm.vim",
    "syntax/machine-ir.vim",
    "syntax/mir.vim",
    "syntax/tablegen.vim",
];

const MLIR_VIM_PATHS: &[&str] = &[
    "ftdetect/mlir.vim",
    "ftplugin/mlir.vim",
    "indent/mlir.vim",
    "syntax/mlir.vim",
];

fn main() -> anyhow::Result<()> {
    for p in LINK_PATHS {
        link(p)?;
    }
    for p in CONFIG_NVIM_LINK_PATHS {
        link(CONFIG_NVIM_PATH.join(p))?;
    }

    for p in LLVM_VIM_PATHS {
        llvm_download_file("llvm/utils/vim", p)?;
    }
    for p in MLIR_VIM_PATHS {
        llvm_download_file("mlir/utils/vim", p)?;
    }
    plug_download()?;

    Ok(())
}
