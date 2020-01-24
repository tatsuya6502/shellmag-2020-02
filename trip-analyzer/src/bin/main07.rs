use chrono::prelude::*;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::error::Error;

// ロケーションIDを示すLocId型をu16型の別名にする
type LocId = u16;

// NaiveDateTimeは長いのでDTという別名を定義する
type DT = NaiveDateTime;  // chrono::NaiveDateTimeはタイムゾーンなしの日時型

// ついでにResult型の別名を定義する
type AppResult<T> = Result<T, Box<dyn Error>>;

// Debugトレイトとserde::Deserializeトレイトを自動導出する
#[derive(Debug, Deserialize)]
struct Trip {
    // renameアトリビュートでフィールド名とCSVのカラム名を結びつける
    #[serde(rename = "tpep_pickup_datetime")]
    pickup_datetime: String,  // 乗車日時
    #[serde(rename = "tpep_dropoff_datetime")]
    dropoff_datetime: String, // 降車日時
    #[serde(rename = "PULocationID")]
    pickup_loc: LocId,        // 乗車地ID
    #[serde(rename = "DOLocationID")]
    dropoff_loc: LocId,       // 降車地ID
}

// serde_jsonでJSON文字列を生成するためにSerializeを自動導出する
#[derive(Debug, Serialize)]
struct RecordCounts {
    read: u32,    // CSVファイルから読み込んだ総レコード数
    matched: u32, // 乗車地や降車地などの条件を満たしたレコードの数
    skipped: u32, // 条件は満たしたが異常値により除外したレコードの数
}

impl Default for RecordCounts {
    fn default() -> Self {
        Self {
            read: 0,    // read: u32::default(), としてもよい
            matched: 0,
            skipped: 0,
        }
    }
}

// CSVファイルのパスを引数にとり、データを分析する
fn analyze(infile: &str) -> Result<String, Box<dyn Error>> {
    // CSVリーダーを作る。失敗したときは?後置演算子の働きにより、analyze()関数から
    // すぐにリターンし、処理の失敗を表すResult::Errを返す
    let mut reader = csv::Reader::from_path(infile)?;
    let mut rec_counts = RecordCounts::default();

        for result in reader.deserialize() {
        // どの型へデシリアライズするかをdeserialize()メソッドに教えるために
        // trip変数に型アノテーションをつける
        let trip: Trip = result?;
        rec_counts.read += 1;

        if is_jfk_airport(trip.dropoff_loc) && is_in_midtown(trip.pickup_loc) {
            let pickup = parse_datetime(&trip.pickup_datetime)?;
            if is_weekday(pickup) {
                rec_counts.matched += 1;
            }
        }
    }

    // 読み込んだレコード数を表示する
    println!("{:?}", rec_counts); // フォーマット文字列を修正する

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

// 日時を表す文字列をDT型へ変換する
fn parse_datetime(s: &str) -> AppResult<DT> {
    DT::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(|e| e.into())
}

// LocIdがミッドタウン内ならtrueを返す
fn is_in_midtown(loc: LocId) -> bool {
    // LocIdの配列を作る
    let locations = [90, 100, 161, 162, 163, 164, 186, 230, 234];
    // 配列に対してバイナリサーチする。locと同じ値があればOk(値のインデックス)が返る
    locations.binary_search(&loc).is_ok()
}

// ロケーションIDがJFK国際空港ならtrueを返す
fn is_jfk_airport(loc: LocId) -> bool {
    loc == 132
}

// 月曜から金曜なら`true`を返す
fn is_weekday(datetime: DT) -> bool {
    // 月:1, 火:2, .. 金:5, 土:6, 日:7
    datetime.weekday().number_from_monday() <= 5
}
