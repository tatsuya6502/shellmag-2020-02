use clap::{App, Arg};

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
    println!("INFILE: {}", infile);
}
