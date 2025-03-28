fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("example"),
        email: String::from("example@mail.com"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User {
        username: String::from("Boris"),
        email: String::from("boris@mail.com"),
        sign_in_count: 2,
        active: false,
    };

    user2.active = true;

    fn build_user(email: String, username: String) -> User {
        User { 
            username: username, 
            email: email, 
            sign_in_count: 1, 
            active: true, 
        }
    }

    fn build_user_short_init(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

   let user3 = User {
        email: String::from("kate@mail.com"),
        username: String::from("Kate"),
        active: user1.active,
        sign_in_count: user2.sign_in_count,
   };

   let user4 = User {
        email: String::from("john@mail.com"),
        username: String::from("John"),
        ..user1
   };

   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);

   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);
   
}



