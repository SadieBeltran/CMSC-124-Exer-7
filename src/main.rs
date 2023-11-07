use std::io;
struct Book {
    book_id: u32,
    book_title: String,
    author: String,
    price: f64,
    stock: u32
}

struct Customer {
    name: String,
    orders: Vec<String>,
    total_cost: f64
}

fn print_menu(){
    println!("\n=== MENU ===\n[1] Add Book information\n[2] Buy a Book\n[3] Edit Book\n[4] Delete Book\n[5] View all Books\n[6] View all Customers\n[7] Exit\n");
}

fn get_input() -> i8 {
    let mut choice = String::new();
    println!("Enter Choice: ");
    io::stdin().read_line(&mut choice).expect("Error");
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

fn add_book(books:&mut Vec<Book>) {
    println!("\n=== Adding book ===\n");
    let id:u32 = readln("Book ID".to_string()).trim().parse().expect("Error");

    if let Some(_) = books.iter().position(|book| book.book_id == id){
        println!("Book ID already exists!");
        return
    }

    let new_book = Book {
        book_id: id,
        book_title: readln("Book Title".to_string()),
        author: readln("Author".to_string()),
        price: readln("Price".to_string()).trim().parse().expect("Error"),
        stock: readln("Stock".to_string()).trim().parse().expect("Error"),
    };

    books.push(new_book);
    println!("Successfully added book!");
}

fn customer_exists(customers:&mut Vec<Customer>, name:String) -> usize{
    if let Some(pos) = customers.iter().position(|customer| customer.name == name){
        pos
    }else{
        //add customer
        let new_customer = Customer {
            name: name,
            orders: Vec::<String>::new(),
            total_cost: 0.0,
        };
        customers.push(new_customer);
        customers.len() - 1
    }
}

fn list_book_titles(books:&Vec<Book>){
    println!("=== BOOKS AVAILABLE ===");
    for book in books{
        println!("[{}] {} - {}", book.book_id, book.book_title, book.price);
    }
    println!("\n");
}

fn buy_book(books:&mut Vec<Book>, customers:&mut Vec<Customer>) {
    if books.len() == 0{
        println!("There are no books available!");
        return
    }

    let cus_name:String = readln("Customer Name".to_string());
    //look for customer
    let customer_id:usize = customer_exists(customers, cus_name);
    list_book_titles(&books);
    let id_to_buy:u32 =  readln("book id to buy".to_string()).trim().parse().expect("Error");

    if let Some(pos) = books.iter().position(|book| book.book_id == id_to_buy){
        if books[pos].stock > 0 {
            books[pos].stock = books[pos].stock - 1;
            customers[customer_id].orders.push(book_summary(&books[pos]));
            customers[customer_id].total_cost = customers[customer_id].total_cost + books[pos].price;
            println!("Successfully ordered menu item {}!", book_summary(&books[pos]));
        }else{
            println!("Failed to buy book: Book is out of stock!");
            return
        }
    }else{
        println!("Failed to buy book: Book does not exist!");
        return
    }
    
}

fn edit_book(books:&mut Vec<Book>) {
    if books.len() == 0{
        println!("There are no books available!");
        return
    }

    let id:u32 = readln("Book id".to_string()).trim().parse().expect("Wrong data type!");

    if let Some(pos) = books.iter().position(|book| book.book_id == id){
        // Edit book here
        println!("Editing ");
        books[pos].book_title = readln("Book Title".to_string());
        books[pos].author = readln("Author".to_string());
        books[pos].price = readln("Price".to_string()).trim().parse().expect("Error");
        books[pos].stock = readln("Stock".to_string()).trim().parse().expect("Error");
    }
}

//ask for book id to be delete
//look for book id in list
//remove book if found
//can't delete if no books are found
fn delete_book(books:&mut Vec<Book>) {
    if books.len() == 0{
        println!("There are no books available!");
        return
    }

    //ask for what id to look for
    let id:u32 = readln("Book id".to_string()).trim().parse().expect("Wrong data type!");
    
    //look for book id in list
    if let Some(pos) = books.iter().position(|book| book.book_id == id){
        books.remove(pos);
        println!("Successfully deleted book!\n");    
    }else{
        println!("Unable to find book!\n");}
}

fn print_book(book:&Book){
    println!("Id: {}", book.book_id);
    println!("Title: {}", book.book_title);
    println!("Author: {}", book.author);
    println!("Price: {}", book.price);
    println!("Stock: {}", book.stock);
}

fn view_books(books:&Vec<Book>) {
    println!("\n=== Current books in database ===");

    if books.len() == 0{
        println!("There are no books available!");
        return
    }
    
    for book in books{
        print_book(book);
        println!("=============\n");
    }
}

fn book_summary(book:&Book) -> String{
    let summary = format!("{}_{}_{}", book.book_id, book.book_title, book.author);
    summary
}

fn view_customers(customers:&Vec<Customer>) {
    if customers.len() == 0{
        println!("There are no books available!");
        return
    }
    
    for customer in customers{
        println!("Name: {}", customer.name);
        for book in &customer.orders{
            println!("- {}", book);
        }
        println!("Total Cost: {}", customer.total_cost);
        println!("=============\n");
    }
    println!("View Customers\n");
}

fn main(){
    let mut choice;
    let mut books:Vec<Book> = Vec::new();
    let mut customers:Vec<Customer> = Vec::new();

    loop {
        print_menu();
        choice = get_input();
        //ask for input
        
        match choice{
            1 => {add_book(&mut books)},
            2 => buy_book(&mut books, &mut customers),
            3 => edit_book(&mut books),
            4 => delete_book(&mut books),
            5 => view_books(&books),
            6 => view_customers(&customers),
            7 => break,
            _ => println!("Choice not found in menu!"),
        }
    }
}