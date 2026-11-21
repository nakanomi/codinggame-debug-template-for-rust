use std::io;

fn calc_area(width : i32, height : i32) -> i32 {
    width * height
}

fn main() {
    // テスト用の数値入力サンプル1
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // ログ抽出スクリプト用のタグ出力
    eprintln!("#input:{}", input_line.trim_end());

    let width: i32 = input_line.trim().parse().unwrap_or(0);


    // テスト用の数値入力サンプル2
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // ログ抽出スクリプト用のタグ出力
    eprintln!("#input:{}", input_line.trim_end());

    let height: i32 = input_line.trim().parse().unwrap_or(0);


    // 結果の標準出力
    println!("area is {}", calc_area(width, height));
}