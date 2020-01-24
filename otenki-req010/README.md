# otenki-req010 パッケージ （reqwest 0.10使用）

このCargoパッケージには「Weather Hacks お天気サービス」から東京の天気予報を取得するプログラムが収められています。
（ごく簡単なプログラムです）　
Part 1の図1で紹介したプログラムを、reqwestクレートのバージョン0.10に対応させたものです。

このプログラムが掲載された雑誌については、本リポジトリのトップにある [README.md][top] をご覧ください。

[top]: ../README.md

## reqwestクレートのバージョン0.10について

特集記事の執筆後にRust 1.39.0がリリースされ、非同期処理を行うための`async/.await`構文が安定化されました。

このパッケージではreqwestクレートのバージョン0.10を使用しています。
reqwestはこのバージョンから`async/.await`に対応しただけでなく、ビルド時間の短縮のためにfeaturesが細かく設定できるようになりました。

| feature | 内容 |
|:--|:--|
| blocking | 同期式（ブロッキング）のAPIを提供。（`async/.await`が不要なときに便利） |
| cookies | Cookieベースのセッションのサポート |
| gzip | レスポンスボディのgzip圧縮をサポート |
| json | JSONボディのシリアライズ、デシリアライズをサポート |
| stream | `futures::Stream` をサポート |
| socks | SOCKS5プロキシをサポート |

その他にもHTTPS接続に必要なTLSの実装を切り替えるfeatureなどが用意されています。
詳しくはReqwestのドキュメントの「[Optional Features][reqwest-features]」の項を参照してください。

[reqwest-features]: https://docs.rs/reqwest/0.10.1/reqwest/#optional-features

## プログラムの変更点

Reqwest 0.9は`async/.await`による非同期処理をサポートしていないため、`reqwest::get()`などのAPIはブロッキングする関数（同期式の関数）になっていました。

```rust:src/main.rs
    # Reqwest 0.9でブロッキング処理
    let body = reqwest::get(service_uri)?.text()?;
```

Reqwest 0.10からはそれらのAPIが`async/.await`による非同期処理をベースにしたものに変わりました。
`reqwest::get()`関数を呼ぶと、処理が実行されるのではなく、futureが返されます。
そして、そのfutureをtokioやasync-stdなどの非同期ランタイムで実行することで、実際の処理が行われてレスポンスが得られます。

非同期処理は複数のタスクを並列して実行したいときに便利ですが、今回のプログラムのように一つのタスクを単純に同期式に実行したいときにはコードが不必要に複雑になってしまいます。
Reqwest 0.10以降で従来どおりの同期処理を行うときは、`Cargo.toml`でReqwestの`blocking` featureをオンにして、以下のように書きます。

```rust:src/main.rs
    # Reqwest 0.10でブロッキング処理。（blocking featureが必要）
    let body = reqwest::blocking::get(service_uri)?.text()?;
```

## 実行方法

ターミナルでこのディレクトリ（`otenki-010`）に移動し、以下のコマンドを実行します。

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
もし興味があったら、[trip-analyzer][trip-analyzer]で使用したserde_jsonクレートやreqwestクレートのjson featureなどを使用して、天気予報情報のパースに挑戦してみてください。

[trip-analyzer]: ../trip-analyzer
