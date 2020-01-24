use clap::{App, Arg};
// Errorトレイトをスコープに入れる
use std::error::Error;

// CSVファイルのパスを引数にとり、データを分析する
fn analyze(infile: &str) -> Result<String, Box<dyn Error>> {
    #[allow(unused_variables, unused_mut)]
    // CSVリーダーを作る。失敗したときは?後置演算子の働きにより、analyze()関数から
    // すぐにリターンし、処理の失敗を表すResult::Errを返す
    let mut reader = csv::Reader::from_path(infile)?;

    // 処理に成功したので（とりあえず空の文字列を包んだ）Result::Okを返す
    Ok(String::default())
}

fn main() {
    // clap::Appを使ってコマンドライン名やバージョンなどを設定する
    let arg_matches = App::new("trip-analyzer")
        .version("1.0")
        .about("Analyze yellow cab trip records")
        // INFILEという名前のコマンドライン引数を登録する
        .arg(Arg::with_name("INFILE")
                .help("Sets the input CSV file")
                .index(1)        // 最初の引数
                .required(true)  // 引数を必須にする
        )
        // get_matches()メソッドを呼ぶとユーザが与えたコマンドライン引数が
        // パースされる
        // ユーザが必須のコマンドライン引数を与えなかったときは、この時点で
        // エラーメッセージが表示され、プログラムが終了する
        .get_matches();

    // INFILEは必須のためSome(..)しか返らない。必ずunwrap()できる
    let infile = arg_matches.value_of("INFILE").unwrap();

    match analyze(infile) {
        Ok(json) => println!("{}", json),  // 標準出力にJSON文字列を表示する
        Err(e) => {
            eprintln!("Error: {}", e);     // 標準エラー出力にエラーを表示する
            std::process::exit(1);         // ステータスコード1でプログラムを終了する
        }
    }
}
