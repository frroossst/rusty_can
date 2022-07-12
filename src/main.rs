fn main() 
    {

    println!("Welcome to rusty_can");

    println!("pick : to pick a stock");

    let args: Vec<String> = std::env::args().collect();

    let command = args[1].clone();

    if command == "pick" 
        {
        pick();
        }
    else if command == "fetch"
        {
        fetch();
        }





    }

fn pick() 
    {
    println!("picking a stock...")
    }

fn fetch()
    {
    println!("fetching stocks data...")
    }
