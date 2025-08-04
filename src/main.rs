fn main() {
    // let number= 10;
    // if number <= 10 {
    // println!("Hello, world!");
    // }
    // else {
    //     println!("hello!");
    // }

    // let number = 1;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let condition:bool = true;
    // let x = if condition { 10 } else { 20 };
    // println!("{x}");

    //LOOPS.........
    // loop{
    //     println!("again");
    //     break;
    // }

    // let mut counter = 10;
    // let result = loop{
    //     counter +=1;

    //     if counter ==20 {
    //         break counter * 2;
    //     }
    // };
    // println!("the result of the counter is {result}");

    //LOOP LABELS.........
    let mut count =0;
   //loop label starts...
    'counting_up : loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining ==1{
            break;
            }
            if count ==5 {
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("final count = {count}");

    let a = [10,30,40,50,20];
    for collection in a {
        println!("the value is {collection}");
    }
}