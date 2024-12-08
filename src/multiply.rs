pub struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let m = num1.len();
        let n = num2.len();
        let mut result: Vec<u32> = vec![0; m+n];
        let mut i1 = m - 1;
        while i1 >= 0 {
            let mut i2 = n - 1;
            while i2 >= 0{
                let mul_value = num1.chars().nth(i1).unwrap().to_digit(10).unwrap() 
                                        * num2.chars().nth(i2).unwrap().to_digit(10).unwrap();
                let p1 = i1 + i2;
                let p2 = p1 + 1;
                let sum = mul_value + result[p2];

                result[p2] = sum % 10;
                result[p1] += sum / 10;
                if i2 == 0 {
                    break;
                }
                i2 -= 1;
            }
            if i1 == 0 {
                break;
            }
            i1 -= 1;
        }

        let mut str = String::new();

        for i in result {
            if str.len() ==0 && i == 0 {
               continue; 
            } else {
                str.push_str(&i.to_string());
            }
        }
        if str.len() == 0 {
            return "0".to_string();
        } else {
            str
        }
    }
}