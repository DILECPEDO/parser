

#[derive(Clone,Debug)]
struct Number{
    value : String,
    position : usize
}

impl Number{
    fn to_numero(string : String)
    // -> Vec<Number>
    {
        let mut number = vec![];
        let mut result:i32 = 0;
        for (i, c) in string.chars().enumerate() {
            number.push(Number{value : c.to_string(), position : (string.len()-1)-i as usize});
        }
        for i in number {
            result += Number::number_to_int(i.value)*(i32::pow(10,i.position as u32));
            
        }
        
        println!("the result is of {} is {}", string, result);
        println!("1 + {} = {}",result, result +1);
    }

    fn number_to_int(number: String) -> i32 {
        match number.as_str() {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "0" => 0,
            _ => panic!("the input is not a valid number")
        }
    }
}











fn main() {

    Number::to_numero("1542".to_string());




}
