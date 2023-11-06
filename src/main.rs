use std::io;
struct Book {
    book_id: u32,
    book_title: String,
    author: String,
    price: f64,
    stock: u32
}

// struct Customer {
//     name: String,
//     orders: Vec<String>,
//     total_cost: f64
// }

fn print_menu(){
    println!("[1] Add Book information\n[2] - Buy a Book\n[3] - Edit Book\n[4] Delete Book\n[5] View all Books\n[6] View all Customers\n[7] - Exit");
}

fn get_input() -> i8 {
    let mut choice = String::new();
    println!("Enter Choice: ");
    io::stdin().read_line(&mut choice).expect("Error");
    println!("\n");
    let choice:i8 = choice.trim().parse().expect("Error");
    choice
}

fn readln(placeholder:String) -> String {
    let mut inp: String = String::new();
    println!("Input {}: ", placeholder);
    io::stdin().read_line(&mut inp).expect("Error");
    inp.pop();
    inp
}

fn add_book(books:&Vec<Book>, id:u32) {
    let new_book = Book {
        book_id: id,
        book_title: readln("Book Title".to_string()),
        author: readln("Author".to_string()),
        price: readln("Price".to_string()).trim().parse().expect("Error"),
        stock: readln("Stock".to_string()).trim().parse().expect("Error"),
    };

    *books.push(new_book);
}

fn buy_book() {
    println!("Buy Book\n");
}

fn edit_book() {
    println!("Edit Book\n");
}

//ask for book id to be delete
//look for book id in list
//remove book if found
//can't delete if no books are found
fn delete_book(mut books:Vec<Book>) {
    //ask for what id to look for
    let id:u32 = readln("Book id".to_string()).trim().parse().expect("Wrong data type!");
    
    //look for book id in list
    if let Some(pos) = books.iter().position(|book| book.book_id == id){
        books.remove(pos);
    }

    println!("Delete Book\n");
}

fn view_books(books:&Vec<Book>) {
    println!("View Books\n");
    for book in books{
        println!("Id: {}", book.book_id);
        println!("Title: {}", book.book_title);
        println!("Author: {}", book.author);
        println!("Price: {}", book.price);
        println!("Stock: {}", book.stock);
        println!("=============\n");
    }
}

fn view_customers() {
    println!("View Customers\n");
}

fn main(){
    let mut choice;
    let mut books:Vec<Book> = Vec::new();
    // let mut customers:Vec<Customer> = Vec::new();
    let mut book_id:u32 = 0;

    loop {
        print_menu();
        choice = get_input();
        //ask for input
        
        match choice{
            1 => {add_book(&books, book_id);
                book_id = book_id + 1;},
            2 => buy_book(),
            3 => edit_book(),
            4 => delete_book(books),
            5 => view_books(&books),
            6 => view_customers(),
            7 => break,
            _ => println!("Choice not found in menu!"),
        }
    }
}