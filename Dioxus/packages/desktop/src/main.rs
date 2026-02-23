use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs::{self, File};
use std::io::{Read, Write};
use rfd::FileDialog;
// 必要に応じて regex や encoding_rs などの use を追加

// --- あなたが作ったロジック群をここに貼り付ける ---
fn convert_wav_to_flac(input_path: &Path) {
    // ... 省略 ...
}
// ----------------------------------------------

fn main() {
    // Dioxus Desktopを起動
    launch(app);
}

fn app() -> Element {
    let mut selected_path = use_signal(|| String::new());

    rsx! {
        div { style: "padding: 20px; font-family: sans-serif;",
            h1 { "Music Toolkit" }

            div { style: "display: flex; gap: 10px; margin-bottom: 20px;",
                // 選択されたパスを表示する（読み取り専用にするのが一般的）
                input {
                    placeholder: "フォルダを選択してください...",
                    value: "{selected_path}",
                    readonly: true,
                    style: "flex-grow: 1; padding: 10px;"
                }

                // フォルダ選択ダイアログを開くボタン
                button {
                    style: "padding: 10px 20px; cursor: pointer;",
                    onclick: move |_| {
                        // ダイアログを表示（同期的に実行）
                        if let Some(path) = FileDialog::new().pick_folder() {
                            // 選択されたパスを状態に保存
                            selected_path.set(path.display().to_string());
                        }
                    },
                    "フォルダを選択"
                }
            }

            // 実行ボタン（パスが空なら無効化するなどの制御も可能）
            button {
                disabled: selected_path.read().is_empty(),
                onclick: move |_| {
                    /* 前回の処理をここに記述 */
                },
                "一括変換を開始"
            }
        }
    }
}
