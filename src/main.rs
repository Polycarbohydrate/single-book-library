use std::io;

struct Book {
    title: String,
    author: String,
    id: u64,
    status: bool,
}

impl Book {
    fn create_book(&mut self) {
        println!("---------------------");
        println!("Input your book name.");
        println!("---------------------");
        let mut bookname = String::new();
        io::stdin().read_line(&mut bookname).expect("Was not able to read line.");
        let bookname = bookname.trim().to_string();
        println!("-----------------------");
        println!("Input your book author.");
        println!("-----------------------");
        let mut bookauthor = String::new();
        io::stdin().read_line(&mut bookauthor).expect("Was not able to read line.");
        let bookauthor = bookauthor.trim().to_string();
        println!("------------------------------------------------------------------------------------");
        println!("You sucessfully created a book! View the details of the book to see the information.");
        println!("------------------------------------------------------------------------------------");
        println!("Press enter to continue.");
        let mut placehold = String::new();
        io::stdin().read_line(&mut placehold).expect("Was not able to read line.");

        self.title = bookname;
        self.author = bookauthor;
    }

    fn checkout_book(&mut self)  {
        println!("-----------------");
        println!("Checked out book.");
        println!("-----------------");
        println!("Press enter to continue.");
        let mut placehold1 = String::new();
        io::stdin().read_line(&mut placehold1).expect("Was not able to read line.");
        
        self.status = false;
        println!("Book Status after checkout: {}", self.status);
    }

    fn return_book(&mut self)    {
        println!("--------------");
        println!("Returned book.");
        println!("--------------");
        self.status = true;
        println!("Press enter to continue.");
        let mut placehold2 = String::new();
        io::stdin().read_line(&mut placehold2).expect("Was not able to read line.");
    }

    fn book_info(&self) {
        println!("--------------------------------------------------------------");
        println!("Book Title: {}", self.title);
        println!("Book Author: {}", self.author);
        println!("Book ID: {}", self.id);
        println!("Book Status (false means checked out): {}", self.status);
        println!("--------------------------------------------------------------");
        println!("Press enter to continue.");
        let mut placehold3 = String::new();
        io::stdin().read_line(&mut placehold3).expect("Was not able to read line.");
    }
}
fn main() {
    let mut x = Book    {
        title: String::new(),
        author: String::new(),
        id: 6834353065289126,
        status: true,
    };

    loop{
        println!("--------------------------------------------------------------");
        println!("Welcome to a simple library system disigned for a single book.");
        println!("Press the appropriate key for the action you would like to do.");
        println!("------------------Press '1' to create a book.-----------------");
        println!("-----------------Press '2' to checkout a book.----------------");
        println!("------------------Press '3' to return a book.-----------------");
        println!("-------------Press '4' to view details of the book.-----------");
        println!("-------Press '0' or 'ctrl + c' to exit the application.-------");
        println!("--------------------------------------------------------------");

        let mut decision = String::new();
        io::stdin().read_line(&mut decision).expect("Was not able to read line.");
        let decision: u32 = match decision.trim().parse()    {
            Ok(num) => num,
            Err(_) => {
                println!("----------------------");
                println!("Enter a number please.");
                println!("----------------------");
                continue
            }
        };

        if decision == 1    {
            x.create_book();
        } else if decision == 2 {
            x.checkout_book();
        } else if decision == 3 {
            x.return_book();
        } else if decision == 4 {
            x.book_info();
        } else if decision == 0 {
            break
        } else  {
            println!("-------------------------------------");
            println!("Enter a valid number. (1, 2, 3, 4, 0)");
        }
    }
}