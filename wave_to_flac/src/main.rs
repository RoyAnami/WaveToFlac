use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::{Read, Write};
use std::fs::File;
use image::{DynamicImage, GenericImageView, ImageFormat};

fn main() {
    // フォルダ選択ダイアログを開く
    let folder = FileDialog::new().pick_folder();
    
    if let Some(folder_path) = folder {
        println!("選択されたフォルダ: {:?}", folder_path);
        
        let entries = fs::read_dir(&folder_path).unwrap();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // WAV → FLAC 変換
                if path.extension().map_or(false, |ext| ext == "wav") {
                    convert_wav_to_flac(&path);
                }
                // CUEファイルの更新
                else if path.extension().map_or(false, |ext| ext == "cue") {
                    update_cue_file(&path);
                }
                // 画像処理
                else if path.extension().map_or(false, |ext| ext == "jpg" || ext == "png") {
                    process_image(&path);
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

fn process_image(image_path: &Path) {
    println!("画像処理: {:?}", image_path);

    // 画像を開く
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(e) => {
            println!("画像の読み込みに失敗しました: {:?}, エラー: {}", image_path, e);
            return;
        }
    };

    let (orig_width, orig_height) = img.dimensions();
    println!("元の解像度: {} x {}", orig_width, orig_height);

    let mut resized_img = img.clone();

    // ① リサイズ処理（元の画像が300x300を超えている場合のみ）
    if orig_width > 300 || orig_height > 300 {
        resized_img = img.resize(300, 300, image::imageops::FilterType::Lanczos3);
        println!("リサイズ処理完了");
    }

    // **ここを修正**
    // 元のファイルにリサイズ後の画像を上書き保存
    let mut original_file = File::create(image_path).expect("元の画像の保存に失敗しました");
    let original_format = if image_path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("jpg")) {
        ImageFormat::Jpeg
    } else {
        ImageFormat::Png
    };
    resized_img.write_to(&mut original_file, original_format).expect("元の画像の上書きに失敗しました");

    // ② 画像フォーマットの変換
    if let Some(ext) = image_path.extension() {
        let mut new_format: ImageFormat;
        let mut new_ext: &str;
        
        if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {
            new_format = ImageFormat::Png;
            new_ext = "png";
        } else if ext.eq_ignore_ascii_case("png") {
            new_format = ImageFormat::Jpeg;
            new_ext = "jpg";
        } else {
            println!("対応していない画像形式です: {:?}", ext);
            return;
        }

        // 変換後のファイル名を決定（同じフォルダに拡張子変更）
        let mut new_path = PathBuf::from(image_path);
        new_path.set_extension(new_ext);

        // ファイルを保存（リサイズ後の画像を新しいフォーマットで保存）
        let mut output_file = File::create(&new_path).expect("変換後の画像ファイルの作成に失敗しました");
        resized_img.write_to(&mut output_file, new_format).expect("変換後の画像の保存に失敗しました");

        println!("画像フォーマット変換完了: {:?} -> {:?}", image_path, new_path);
    }
}
