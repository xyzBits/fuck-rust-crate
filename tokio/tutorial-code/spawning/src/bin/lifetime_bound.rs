
#[tokio::main]
async fn main() {

    let v = vec![1, 2, 3, 4];

    tokio::task::spawn(async move {
        println!("here is a vec: {:?}", v);
    });

    // println!("{:?}", v);

}


#[cfg(test)]
mod static_lifetime {
    // make a constant with 'static lifetime

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }
     static NUM: i32 = 18;
    /// As a reference lifetime 'static indicates that the data pointed to by the reference
    /// lives for the entire lifetime of the running program. It can still be coerced to a
    /// shorter lifetime.
    ///
    ///
    #[test]
    fn test_1() {
        {
            // make a string literal and print it
            let static_string: &'static str = "read-only memory";
            println!("{}", static_string);

            // when static_string goes out of scope, the reference
            // can no longer be used, but the data remains in the binary
        }


        {
            let lifetime_num = 9;

            let coerced_static: &'static i32 = coerce_static(&NUM);
            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);



    }
}