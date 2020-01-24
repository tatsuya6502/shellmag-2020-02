# シェルスクリプトマガジン掲載記事 『はじめてのRust』 サンプルコード集

このリポジトリには、シェルスクリプトマガジン Vol. 64 に掲載された『はじめてのRust』で使用するサンプルコードが収録されています。

**シェルスクリプトマガジン**</br>
[Vol. 64　2020年2月号][shellmag-vol-64]</br>
特集2 『はじめてのRust』</br>
執筆者：河野 達也（かわの たつや）</br>
ページ数：24ページ

[shellmag-vol-64]: https://shell-mag.com/vol-64/

## 収録されているRustパッケージ

| パッケージ名 | 掲載箇所 | 内容 |
|:--|:--|:--|
| [otenki (reqwest 0.9)][otenki-req09] | Part 1の図1 | reqwestクレート 0.9を使って「Weather Hacks お天気サービス」から東京の天気予報を取得する。（ごく簡単なもの） |
| [otenki (reqwest 0.10)][otenki-req010] | 非掲載 | 上のプログラムを`async/.await`時代のreqwestクレート 0.10に対応させたもの |
| [sqrt][sqrt] | Part 1の「はじめてのRustプログラム」の章 | ニュートン法で平方根の近似値を求める |
| [trip-analyzer][trip-analyzer] | Part 2 「コマンドラインプログラムを作ろう」 | ニューヨーク市のタクシー＆リムジン委員会が公開している乗車記録を分析し、マンハッタン中心部からJFK国際空港までの所要時間を時間帯ごとに集計する。（利用クレート：chrono, clap, csv, hdhistogram, serde, serde_json） |

パッケージの詳しい内容や実行方法については、パッケージ名のリンクをクリックして、そのパッケージのREADME.mdを参照してください。

[otenki-req09]: ./otenki-req09
[otenki-req010]: ./otenki-req010
[sqrt]: ./sqrt
[trip-analyzer]: ./trip-analyzer

## 変更履歴（主なもの）

| 日付 | Gitタグ | 内容 |
|:--|:--|:--|
| 2020-01-24 | 1.0.0 | サンプルコードを公開しました |
