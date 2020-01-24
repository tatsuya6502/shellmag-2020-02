# trip-analyzer パッケージ

このCargoパッケージにはPart 2で作成するプログラムが収められています。
ニューヨーク市のタクシー＆リムジン委員会が公開している乗車記録を分析し、マンハッタン中心部からJFK国際空港までの所要時間を時間帯ごとに集計します。

このプログラムが掲載された雑誌については、本リポジトリのトップにある [README.md][top] をご覧ください。

[top]: ../README.md

## ソースファイルの構成

このパッケージの`src`ディレクトリは以下の構成になっています。

```console
src/
├── bin              # src/binディレクトリにはPart 2の内容に従って
│   ├── main01.rs    # 途中まで制作したプログラムが収められている
│   ├── main02.rs
│   ├── main03.rs
│   ├── main04.rs
│   ├── main05.rs
│   ├── main06.rs
│   ├── main07.rs
|   └── main08.rs
└── main.rs          # srcディレクトリ直下には完成したプログラムが
                     # 収められている
```

それぞれのファイルの内容は以下のようになっています。

| ファイル名 | プログラムの制作状況 | 追加された内容 |
|:--|:--|:--|
| [`src/bin/main01.rs`][main01] | 図3の内容まで反映したところ | コマンドライン引数を処理する |
| [`src/bin/main02.rs`][main02] | 図8まで反映 | コマンドライン引数を必須にする |
| [`src/bin/main03.rs`][main03] | 図11まで反映 | CSVファイルを開きエラーを処理する |
| [`src/bin/main04.rs`][main04] | 図12まで反映 | CSVファイルのレコードを順に読み込む |
| [`src/bin/main05.rs`][main05] | 図17まで反映 | CSVのレコードを表す構造体を定義し、レコードをデシリアライズする |
| [`src/bin/main06.rs`][main06] | 図21まで反映 | カウンタをまとめた構造体を定義する |
| [`src/bin/main07.rs`][main07] | 図25まで反映 | 日時を表す文字列をchronoで変換し、対象レコードを絞り込む |
| [`src/bin/main08.rs`][main08] | 図30まで反映 | 統計処理を行う |
| [`src/main.rs`][main]         | 完成 | 統計情報をJSON形式で出力する |

[main01]: ./src/bin/main01.rs
[main02]: ./src/bin/main02.rs
[main03]: ./src/bin/main03.rs
[main04]: ./src/bin/main04.rs
[main05]: ./src/bin/main05.rs
[main06]: ./src/bin/main06.rs
[main07]: ./src/bin/main07.rs
[main08]: ./src/bin/main08.rs
[main]: ./src/main.rs

## 実行方法

完成したプログラムだけでなく、`src/bin`ディレクトリ配下の製作途中のプログラムもコンパイル・実行できます。
ターミナルでこのディレクトリ（`trip-analyzer`）に移動し、以下のコマンドを実行します。

```console
# src/bin配下にある製作途中のプログラムを実行するなら
$ cargo run --release --bin main01  # → src/bin/main1.rsがコンパイル・実行される

# src直下にある完成したプログラムを実行するなら
$ cargo run --release --bin trip-analyzer
```
