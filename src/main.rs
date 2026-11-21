use std::io;

fn calc_area(width : i32, height : i32) -> i32 {
    width * height
}

fn main() {
    // テスト用の数値入力サンプル:横幅
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // ログ抽出スクリプト用のタグ出力
    eprintln!("#input:{}", input_line.trim_end());

    let width: i32 = input_line.trim().parse().unwrap_or(0);


    // テスト用の数値入力サンプル:高さ
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // ログ抽出スクリプト用のタグ出力
    eprintln!("#input:{}", input_line.trim_end());

    let height: i32 = input_line.trim().parse().unwrap_or(0);


    // 結果の標準出力
    println!("area is {}", calc_area(width, height));
}
// テストの記述
// rustは通常lib.rsに機能やテストを記述すると思いますが
// codinggameでは1ソースにまとめる必要があるためテストも
// main.rsに書く方が利便性が上がります(テストごとコピーペースト可能)。
// codinggameでの動作に影響を及ぼさないことは確認済みです(2026/11/21)
// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, calc_area(4, 2));
    }
}
