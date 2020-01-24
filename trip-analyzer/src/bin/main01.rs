// Clapで定義されているclap::Appとclap::Argの2つの型をスコープに入れる
use clap::{App, Arg};

fn main() {
    // clap::Appを使ってコマンドライン名やバージョンなどを設定する
    let arg_matches = App::new("trip-analyzer")
        .version("1.0")
        .about("Analyze yellow cab trip records")
        // INFILEという名前のコマンドライン引数を登録する
        .arg(Arg::with_name("INFILE")
                .help("Sets the input CSV file")
                .index(1)  // 最初の引数
        )
        // get_matches()メソッドを呼ぶとユーザが与えたコマンドライン引数が
        // パースされる
        .get_matches();

    // とりあえずいまはINFILEの文字列を表示する。"{:?}"はデバッグ用文字列を表示する
    println!("INFILE: {:?}", arg_matches.value_of("INFILE"));
}
