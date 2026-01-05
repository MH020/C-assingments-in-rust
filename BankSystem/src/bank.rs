enum Transtype {
    IN,
    OUT
}

struct Transaction {
    tans_type: Transtype, 
    amount: i32,
    desc: String,  
}

pub struct Account {
    transactions: Vec<Transaction>
}


pub fn crate_account() -> Account {

    Account { transactions: Vec::new()}
}


pub fn add_transaction(account:  &mut Account, input: Transtype, amount: i32, desc: String){
    let new = Transaction { tans_type: input, amount, desc }; 
    account.transactions.push(new); 
}

pub fn balence(account: &Account) -> i32{
    let mut total = 0; 
    for transaction in &account.transactions {
        match transaction.tans_type {
            Transtype::IN => {total += transaction.amount},
            Transtype::OUT => {}
        };
    };

    total
}