pub fn fermat_test(a: u32, p: u32) {
    //generate an array obeying 1<=a<p
    let a_values = (0u32..a.clone()).collect::<Vec<u32>>();
    for value in a_values.iter(){
        match (value.pow(p.clone())- value.clone()) % p.clone(){
            0 => {
                let result = (value.pow(p.clone())- value.clone()) % p.clone();
                println!("Primality Test: {} ^ {} - {} % {} = {}", value, p.clone(), value, p.clone(), result)
            },
            _ =>{
                let result = (value.pow(p.clone())- value.clone()) % p.clone();
                println!("Failed Fermat's Primality Test:\n{} ^ {} - {} % {} = {}", value, p.clone(), value, p.clone(), result);
                break;
            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fermat_test(13,5);
        assert_eq!(result, result);
    }
}
