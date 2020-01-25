# シェルスクリプトマガジン『はじめてのRust』 サンプルコード集

このリポジトリには、シェルスクリプトマガジン Vol. 64 に掲載された『はじめてのRust』で使用するサンプルコードが収録されています。

## 掲載記事の概要

**シェルスクリプトマガジン**　Vol. 64（2020年2月号）</br>
特集1 『はじめてのRust』　河野 達也（かわの たつや）

シェルスクリプトマガジンに掲載された記事では、Rustの特徴、使いどころ、開発環境の構築方法、プログラミングのやり方などを、初心者にもわかりやすいように紹介しています。
また、CSV形式のデータを分析するという実用的なプログラムを扱っています。

掲載号（Vol. 64）について詳しくは [こちら][shellmag-vol-64] をご覧ください。 

[shellmag-vol-64]: https://shell-mag.com/vol-64/

## 収録されているサンプルコード

このリポジトリには以下のサンプルコードが収録されています。

| Cargoパッケージ名 | 掲載箇所 | 内容 |
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

| 日付 | Git Tag | 内容 |
|:--|:--|:--|
| 2020-01-24 | [1.0.0][release-1.0.0] | サンプルコードを公開しました |

[release-1.0.0]: https://github.com/tatsuya6502/shellmag-2020-02/releases/tag/1.0.0
