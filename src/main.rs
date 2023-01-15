fn main() {
    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{}", s); // This will print `hello, world!`
    }

    {
        // Stack-Only Data: Copy

        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        //println!("{}", s1); error[E0382]: borrow of moved value: `s1`
        println!("{}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        // Ownership and Functions
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x);
    }

    {
        // Return Values and Scope
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
        println!("s1 = {}, s3 = {}", s1, s3); // moves its return value into s3
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
    {
        let s1 = String::from("hello");

        let len = calculate_length_ref(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let mut s = String::from("hello");

        change(&mut s);
        println!("{}", s);
    }
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s; This fail only one mutable reference is valid
        //error[E0499]: cannot borrow `s` as mutable more than once at a time

        println!("{}", r1);
    }

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
            println!("{}", r1);
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
        println!("{}", r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
                     // let r3 = &mut s; BIG PROBLEM
                     //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

        println!("{}, {}, {}", s, r1, r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
