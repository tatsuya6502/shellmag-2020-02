// reqwestクレートを使うために、Cargo.tomlファイルのdependencies
// セクションに reqwest = { version = "0.10", features = ["blocking"] } と書く

// このプログラムは「Weather Hacks お天気Webサービス」から東京の天気予報を取得する
// サービスの説明：http://weather.livedoor.com/weather_hacks/

// main関数。プログラムが起動すると最初に呼ばれる
// この関数は引数を取らず、Result型の値を返す
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // WebサービスのURI文字列をservice_uri変数にセットする
    let service_uri =
        "http://weather.livedoor.com/forecast/webservice/json/v1?city=130010";
    // 指定したURIに対してGETリクエストを送信し、レスポンスボディを取得する
    let body = reqwest::blocking::get(service_uri)?.text()?;
    // レスポンスボディを表示する
    println!("body = {:?}", body);
    // Okを返してmain関数から戻る
    // return文は不要だがこの行だけ行末にセミコロンがないことに注意
    Ok(())
}
