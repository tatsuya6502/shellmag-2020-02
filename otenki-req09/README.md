# otenki-req09 パッケージ （reqwest 0.9使用）

このCargoパッケージにはPart 1の図1で紹介したプログラムが収められています。
reqwestクレートのバージョン0.9を使って「Weather Hacks お天気サービス」から東京の天気予報を取得します。
（ごく簡単なプログラムです）　

このプログラムが掲載された雑誌についての情報は、本リポジトリのトップにある [README.md][top] をご覧ください。

[top]: ../README.md

## reqwestクレートのバージョン0.10について

特集記事の執筆後にRust 1.39.0がリリースされ、非同期処理を行うための`async/.await`構文が安定化されました。

このパッケージでは`async/.await`の安定化前にリリースされたreqwestクレートのバージョン0.9を使用しています。
reqwestはその次のバージョンの0.10から、`async/.await`に対応しただけでなく、ビルド時間の短縮のためにfeaturesが細かく設定できるようになりました。

本パッケージ（otenki-req09）の内容をreqwest 0.10向けに変更したものを [otenki-req010][otenki-req010] パッケージとして用意してあります。
そちらもご覧ください。

[otenki-req010]: ../otenki-req010

## 実行方法

ターミナルでこのディレクトリ（`otenki-09`）に移動し、以下のコマンドを実行します。

```console
$ cargo run --release
```

**実行例**

```console
$ cargo run --release
body = "{\"pinpointLocations\":[{\"link\":\"http://weather.livedoor.com/area/forecast/1310100\",\"name\":\"\\u5343\\u4ee3\\u7530\\u533a\"},{\"link\":\"http://weather.livedoor.com/area/forecast/1310200\",\"name\":\"\\u4e2d\\u592e\\u533a\"},{\"link\":\"http://weather.livedoor.com/area/forecast/1310300\",\"name\":\"\\u6e2f\\u533a\"},
..（略）..
```

Weather Hacks お天気サービスからは天気予報情報がJSON形式で返されますが、このプログラムでは簡単のためにJSONのパースなどはせず、レスポンスボディをそのまま表示しています。
もし興味があったら、[trip-analyzer][trip-analyzer]で使用したserde_jsonクレートやreqwestクレートのjsonサポートなどを使用して、天気予報情報のパースに挑戦してみてください。

[trip-analyzer]: ../trip-analyzer
