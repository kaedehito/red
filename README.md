# red
**red** は、Rust で `ed` (Unix の行指向テキストエディタ) を再現したシンプルなエディタです。

## インストール方法

### ソースからインストール（Cargo 必須）
Rust の `cargo` を使ってインストールできます。

```sh
git clone https://github.com/kaedehito/red
cd red
cargo install --path .
```

または、リリースページからバイナリをダウンロードしてインストールできます。

---

### バイナリインストール（推奨）

#### Ubuntu
`dpkg` を使用して簡単にインストールできます。

```sh
curl -SfL https://github.com/kaedehito/red/releases/download/1.0.0/red.deb -o red.deb
sudo dpkg -i red.deb
```

#### その他の Linux
バイナリを直接ダウンロードして、実行権限を付与後 `/usr/local/bin/` に移動してください。

```sh
curl -SfL https://github.com/kaedehito/red/releases/download/1.0.0/red -o red
chmod +x ./red
sudo mv ./red /usr/local/bin/
```

#### Windows
PowerShell を管理者権限で開き、以下のコマンドを実行してください。

```powershell
$downloadUrl = "https://github.com/kaedehito/red/releases/download/1.0.0/red.exe"
$destination = "$env:TEMP\red.exe"
$installPath = "C:\Program Files\Red"
Invoke-WebRequest -Uri $downloadUrl -OutFile $destination -UseBasicParsing
if (!(Test-Path -Path $installPath)) {
    New-Item -ItemType Directory -Path $installPath | Out-Null
}
Move-Item -Path $destination -Destination "$installPath\red.exe" -Force
$oldPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if ($oldPath -notlike "*$installPath*") {
    $newPath = "$oldPath;$installPath"
    [System.Environment]::SetEnvironmentVariable("Path", $newPath, [System.EnvironmentVariableTarget]::Machine)
}
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine")
Write-Host "Installation complete. Please restart your terminal to fully apply the changes."
```

---

## 動作環境
✅ **動作確認済み**: Linux, Windows  
⚠ **未確認**: macOS（動作報告歓迎）

## フィードバック & 貢献
- バグ報告や改善提案は [Issues](https://github.com/kaedehito/red/issues) へお願いします。
- プルリクエストも歓迎します！


