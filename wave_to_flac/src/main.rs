use rfd::FileDialog;
use std::fs;
use std::path::{Path};
use std::process::Command;
use regex::Regex;

fn main() {
    // フォルダ選択ダイアログを開く
    let folder = FileDialog::new().pick_folder();
    
    if let Some(folder_path) = folder {
        println!("選択されたフォルダ: {:?}", folder_path);
        
        // フォルダ内の.wavファイルを取得
        let entries = fs::read_dir(&folder_path).unwrap();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "wav") {
                    convert_wav_to_flac(&path);
                } else if path.extension().map_or(false, |ext| ext == "cue") {
                    update_cue_file(&path);
                }
            }
        }
    } else {
        println!("フォルダが選択されませんでした。");
    }
}

fn convert_wav_to_flac(input_path: &Path) {
    let output_path = input_path.with_extension("flac");
    println!("変換: {:?} -> {:?}", input_path, output_path);
    
    let status = Command::new("flac")
        .arg("-fo")
        .arg(output_path.as_os_str())
        .arg(input_path.as_os_str())
        .status()
        .expect("FLACコマンドの実行に失敗しました");
    
    if status.success() {
        println!("FLACファイルが作成されました: {:?}", output_path);
    } else {
        println!("FLACファイルの変換に失敗しました。");
    }
}

fn update_cue_file(cue_path: &Path) {
    println!("CUEファイルを更新: {:?}", cue_path);
    
    let content = fs::read_to_string(cue_path).expect("CUEファイルの読み込みに失敗しました");
    let re = Regex::new(r#"(?i)(FILE \".*?)(\.wav)(\"\s*WAVE)"#).unwrap();
    let updated_content = re.replace_all(&content, "$1.flac$3").to_string();
    
    fs::write(cue_path, updated_content).expect("CUEファイルの更新に失敗しました");
    println!("CUEファイルを更新しましたd: {:?}", cue_path);
}
