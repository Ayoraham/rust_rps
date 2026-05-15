use std::io;

#[derive(Debug)]
struct AccountProfile{
    name: String,
    phone: u64,
    balance: f32,
    tier: i8,
    limit: f32,
}

impl AccountProfile{
    fn credit(&mut self, credit_amt:f32){
        if self.balance + credit_amt > self.limit{println!("Amount too large! Upgrade account limit to deposit more")}
        else{
            self.balance += credit_amt;
            println!("Succesfully credited amt ${}", credit_amt);
        }
    }

    fn upgrade_limit(&mut self, tier:i8){
        match tier{
            1 => {self.limit = 50000 as f32;self.tier = 1 as i8; println!("Succesfully upgraded to TIER 1\nNew limit is 50000")}
            2 => {self.limit = 1_000_000 as f32;self.tier = 2 as i8; println!("Succesfully upgraded to TIER 2\nNew limit is 1,000000 ")}
            _ => {println!("Invalid Tier")}
        }
    }

    fn debit(&mut self, debit_amt:f32){
        if self.balance - debit_amt < 0.0{println!("ERROR! | Account balance cannot be negative")}
        else{
            self.balance -= debit_amt;
            println!("Succesfully debited {}",debit_amt)
        }
    }

    fn view_account_info(&self){
        println!("------------------------------------ACCOUNT INFORMATION-------------------------------------\nName: {0}Phone Number: {4}\nAccount Balance: {1:.2}\nAccount Tier: {2}\nAccount Limit: ${3:.0}",self.name,self.balance,self.tier,self.limit,self.phone)
    }
}

fn build_account(name:String,phone:u64) -> AccountProfile{
    AccountProfile {
        name,
        phone,
        balance:0.0,
        tier:0,
        limit:1000.0
    }
}

fn main() {
    // Begin Op
    println!("--------------------------------------------------------------------------BANKING APP--------------------------------------------------------------------------");
    println!("-------------------Create Account------------------");println!("Account Name: ");
    let mut acc_name = String::new();
    io::stdin().read_line(&mut acc_name).unwrap();
    println!("Phone Number: ");
    let mut acc_no = String::new();
    io::stdin().read_line(&mut acc_no).unwrap(); let acc_no: u64 = acc_no.trim().parse().unwrap();

    let mut account = build_account(acc_name, acc_no);
    println!("{:?}",account);

    loop{
        println!("Enter 's' to perform banking operations\nEnter 'q' to quit");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = &user_input.trim().to_lowercase();
        match user_input.as_str(){
            "s" => {
                println!("Perform Actions\n'c' - Top-up Account Bal\n'd' - Withdraw\n'u' - Upgrade Limit\n'i' - View Account Info\n");
                let mut chosen_op = String::new();
                io::stdin().read_line(&mut chosen_op).unwrap();
                let chosen_op = chosen_op.trim().to_lowercase();

                match chosen_op.as_str(){
                    "c" => {let mut cr_amt = String::new();
                            println!("Enter Credit Amount: $");
                            io::stdin().read_line(&mut cr_amt).unwrap();let cr_amt: f32= cr_amt.trim().parse().unwrap();
                            account.credit(cr_amt);},

                    "d" => {let mut dr_amt = String::new();
                            println!("Enter Credit Amount: $");
                            io::stdin().read_line(&mut dr_amt).unwrap();let dr_amt: f32= dr_amt.trim().parse().unwrap();
                            account.debit(dr_amt);},
                    
                    "u" => {let mut tier = String::new();
                            println!("What tier would you like to upgrade to?: ");
                            io::stdin().read_line(&mut tier).unwrap();let tier: i8= tier.trim().parse().unwrap();
                            account.upgrade_limit(tier);},
                    
                    "i" => account.view_account_info(),
                    _ => println!("Invalid Operation")
                }

            },

            "q" => break,
            _ => println!("Invalid Operation")

            
        }
    }
    //match user_input{
        //'s' => println!("What Operation do you want to perform?\n ")
    //}

}
