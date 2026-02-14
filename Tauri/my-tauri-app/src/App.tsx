import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
  const [folder, setFolder] = useState("");
  const [message, setMessage] = useState("");

  async function selectFolder() {
    const selected = await open({
      directory: true,
      multiple: false
    });

    if (selected) {
      setFolder(selected as string);
    }
  }

  async function runProcess() {
    try {
      const result = await invoke<string>("process_folder", {
        folderPath: folder
      });
      setMessage(result);
    } catch (e) {
      setMessage("エラー: " + e);
    }
  }

  return (
    <div>
      <button onClick={selectFolder}>フォルダ選択</button>
      <p>{folder}</p>

      <button onClick={runProcess}>実行</button>
      <p>{message}</p>
    </div>
  );
}

export default App;
