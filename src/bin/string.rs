fn get_greeting(hour:u8)-> &'static str{
	if hour<12{
		"Good Morning!"
	}else if hour<18{
        "Good Afternoon"
    }else{
	"Good Evening"
    }
}

fn main(){
 let greet = get_greeting(12);
println!("greet = {greet}");
}