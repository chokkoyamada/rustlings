struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
    // TODO: `regular_structs`テストが想定するように構造体のフィールドを定義してください。
    // 構造体に適切な型はなんでしょうか？
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 基本的な構造体のインスタンスを作成してください。
        let green: ColorRegularStruct = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: タプル構造体のインスタンスを作成してください。
        let green: ColorTupleStruct = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: ユニット構造体のインスタンスを作成してください。
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
