/*
const FREEZE: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64
{
    
    let cels = (f - FREEZE) * (5.0/9.0);
    return cels;
}

fn celsius_to_fahrenheit(f: f64) -> f64
{

    let fahr = (f * (9.0/5.0)) + FREEZE;
    return fahr;
}


fn main()
{
    let mut temp: f64 = 100.0; //100 degrees fahrenheit

    println!("F of {} to C convert: {}", temp, fahrenheit_to_celsius(temp));

    temp+= 1.0;
    let mut num = 0;
    loop{


        println!("F of {} to C convert: {}", temp, fahrenheit_to_celsius(temp));
        if(num == 4){
            break;
        }

        num+=1;
        temp+=1.0;
    }

}

*/
/*
fn is_even(n: i32) ->bool
{
    if(n%2 == 0){
        return true;
    }else{
        return false;
    }
}


fn main(){

    let numbers = [1,2,3,11,5,6,7,8,9,15];

    for num in numbers.iter() {
        if(is_even(*num)){
            println!("{} is even", num);
        }else{
            println!("{} is odd", num);
        }

        if(num%3==0){
            print!("Fizz");
        }
        if(num%5==0){
            print!("Buzz");
        }
        println!();


    }

    let mut counter1 = 0;
    let mut sum = 0;
    while counter1 < 10{
        sum = sum + numbers[counter1];
        counter1+=1;
    }
    println!("The sum is: {}", sum);

    let mut counter2=0;
    let mut greatest = numbers[0];
    while counter2 < 10{
        if(greatest < numbers[counter2]){
            greatest = numbers[counter2];
        }
        println!("This is the greatest value right now: {}", greatest);
        counter2+=1;
    }
}
    
*/

fn check_guess(guess: i32, secret: i32) -> i32
{
    if guess == secret {
        return 0;
    }else if guess < secret{
        return -1;
    }else{
        return 1;
    }
}

fn main(){
    let secret = 5;
    let guess = 5;
    let mut counter = 1;

    while guess != secret{
        
        if check_guess(guess,secret) == 0 {
            println!("Correct!");
            break;
        }else if check_guess(guess,secret)==-1 {
            println!("Guess is too low");
        }else{
            println!("Guess is too high");
        }
        counter +=1;
    }

    println!("It took you {} guesses", counter)

}


