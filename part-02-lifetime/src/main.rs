use std::env;

macro_rules! disable {
    ($($t:tt)*) => {};
}

macro_rules! enable {
    ($($t:tt)*) => {
        $($t)*
    };
}

macro_rules! activate_code {
    (false ; $comment:literal ; $($t:tt)*) => {
        disable!{
            $($t)*
        }
    };

    (true ; $comment:literal ; $($t:tt)*) => {
        enable!{
            $($t)*
        }
    };
}

activate_code! {
    false;
    "KO, you should speficy lifetime of the reference";
    struct Teacher {
        name: &str,
        age: u8,
    }

    impl Teacher {
        fn my_name(&self) -> &str {
            self.name
        }
    }
}

activate_code! {
    true;
    "OK, Lifetime is specified";
    struct Teacher<'a> {
        name: &'a str,
        age: u8,
    }

    impl<'a> Teacher<'a> {
        fn my_name(&self) -> &'a str {
            self.name
        }
    }
}

fn make_10<'a>() -> &'a u64 {
    &10
}

fn result_reference_10<'b>() -> &'b u64 {
    make_10()
}

activate_code! {
    false;
    "Reference can have two lifetime (a or b)";
    fn longest(a: &str, b: &str) -> &str {
        if a.len() >= b.len() {
            return a;
        } else {
            return b;
        }
    }
}

activate_code! {
    true;
    "Reference can have two lifetime (a or b)";
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }
}

fn main() {
    const TEACHER_NAME: &str = "Paul"; /* 'static */
    let mut teacher = Teacher {
        name: TEACHER_NAME,
        age: 25,
    };

    println!(
        "Hello {}, are you {} years old ?",
        teacher.my_name(),
        teacher.age
    );

    {
        if let Some(arg0) = env::args().next() {
            teacher.name = arg0.as_str();
        }
    }

    activate_code! {
        false;
        "teacher.name is danggling reference on arg0.as_str()";
        println!("You change name, Mr {}", teacher.my_name());
    }

    let x = result_reference_10();
    println!("result reference {}", x);

    activate_code! {
        false;
        "Error: result can reference on long which have not enough lifetime";

        let short: &str = "short";
        let mut result : &str = "";
        println!("Longest = {}", result);
        {
            let temp_str: String = "very very long".to_string();
            let long: &str = temp_str.as_str();
            result = longest(short, long);
        }

        println!("Longest = {}", result);
    }

    activate_code! {
        true;
        "Error: result can reference on long which have not enough lifetime";

        let short : &str = "short";
        let mut result : &str = "";
        println!("Longest = {}", result);
        {
            let long: &str = "very very long";
            result = longest(short, long);
        }

        println!("Longest = {}", result);
    }

    activate_code! {
        true;
        "Error: result can reference on long which have not enough lifetime";

        let mut result : &str = "";
        let temp_str: String = "very very long".to_string();
        let long: &str = temp_str.as_str();
        let short: &str = "short";
        println!("Longest = {}", result);
        {
            result = longest(short, long);
        }

        println!("Longest = {}", result);
    }
}
