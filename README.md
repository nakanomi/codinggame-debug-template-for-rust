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

## How to Use
- stdinの自動入力について
  - VSCodeでCodeLLDBを使ってデバッグできるようにする部分については、あなたの手で整えてください。
  - main.rsの```eprintln!("#input:{}", input_line.trim_end());```の行にブレークポイントを設定してください。
  - デバッグ実行してください。通常であればブレークポイントの上```io::stdin().read_line(&mut input_line).unwrap();```の行でターミナルから入力しなければならないところですが、その必要はなくブレークポイントの地点で止まっているはずです。
- codinggameのログ抽出について
  - main.rsの内容をcodinggameのお好きなパズルのソースコードエリアに貼り付け、実行させてください。失敗時のログがゲームページ左下のConsole outputに出力されるはずです。例えば[no-spoon-episod-1](https://www.codingame.com/ide/puzzle/there-is-no-spoon-episode-1)では下記のようになります。
```
Console outputGame informationAction (Output Stream)Debug (Errors Stream)
Game information:
Initializing APU weaponry...
Standard Error Stream:
#input:2
#input:2
Standard Output Stream:
area is 4
Game information:
invalid input. Expected 'x1 y1 x2 y2 x3 y3' but found 'area is 4'
```
  - 上記ログをscripts_and_logフォルダのtext.logにコピーアンドペーストしてください。
  - 下記のようにフォルダの中にあるextract_test.shを実行してください。
```
sh extract_test.sh
```
  - ターミナルに最初の2つの入力を取り出したものが出力されます。
```
2
2
```
  - つまりmain.rsに書かれているように、stdinの直後に```eprintln!("#input:{}", input_line.trim_end());```を書いていけば、stdinの自動入力に使うtest.txtの内容を作成できるという事です。
