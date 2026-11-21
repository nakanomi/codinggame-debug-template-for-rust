## 📝 Related articles (Qiita)

- **[【Rust】CodingameをVS Codeで快適にローカルデバッグする環境構築](https://qiita.com/nakano_mi/items/e88c031765705b5e51ba)**
  - **概要:** CodingameのパズルをRustで解く際、手動での標準入力やテストケース作成の手間を解消するため、VS Code（CodeLLDB）の `stdio` リダイレクト機能と、エラーログからテストデータを自動抽出するシェルスクリプトを組み合わせたデバッグ環境を構築しました。
  - **アピールポイント:** 開発環境の自動化・効率化（DX向上）と、他者が再現しやすい手順の言語化。
  - **必要環境:** 既にVSCode上でCodeLLDBを使ったデバッグの環境が整えられている必要があります。


## 📂 Folder structure (フォルダ構成)
```
codingame-debug-template-for-rust/
├── .vscode/
│   ├── launch.json        # CodeLLDBによるstdin自動入力・デバッグ設定
│   └── tasks.json         # F5キーでの自動ビルド設定
├── scripts/
│   └── extract_test.sh    # Codingameのログからtest.txtを生成するスクリプト
│   └── test.log           # 実行時に出力されるログ全文
├── test.txt               # 入力ファイルのサンプル
└── README.md              # リポジトリの目的・使い方・学んだことのまとめ
```

