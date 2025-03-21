use rfd::FileDialog;
use std::fs;
use std::path::{Path};
use std::process::Command;
use regex::Regex;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::{Read, Write};
use std::fs::File;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::{Path, PathBuf};

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

        // 変換成功後、元のWAVファイルを削除
        if let Err(e) = fs::remove_file(input_path) {
            println!("元のWAVファイルの削除に失敗しました: {:?}, エラー: {}", input_path, e);
        } else {
            println!("元のWAVファイルを削除しました: {:?}", input_path);
        }
    } else {
        println!("FLACファイルの変換に失敗しました。");
    }
}

fn update_cue_file(cue_path: &Path) {
    println!("CUEファイルを更新: {:?}", cue_path);

    // Shift-JIS で読み込む
    let mut file = File::open(cue_path).expect("CUEファイルの読み込みに失敗しました");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("CUEファイルの読み込み中にエラーが発生しました");

    let (content, _, had_errors) = SHIFT_JIS.decode(&buffer);
    if had_errors {
        println!("警告: Shift-JIS のデコード中にエラーが発生しましたが、可能な限り処理を続行します。");
    }

    // `.wav` を `.flac` に置換
    let re = Regex::new(r#"(?i)(FILE \".*?)(\.wav)(\"\s*WAVE)"#).unwrap();
    let updated_content = re.replace_all(&content, "$1.flac$3").to_string();

    // Shift-JIS にエンコードして保存
    let (encoded_content, _, _) = SHIFT_JIS.encode(&updated_content);
    let mut output_file = File::create(cue_path).expect("CUEファイルの書き込みに失敗しました");
    output_file.write_all(&encoded_content).expect("CUEファイルの保存に失敗しました");

    println!("CUEファイルを更新しました: {:?}", cue_path);
}
