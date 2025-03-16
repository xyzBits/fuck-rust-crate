// use std::pin;
// use std::pin::Pin;
//
// #[derive(Debug)]
// struct SelfReference {
//     a: String,
//     b: *const String,
// }
//
// impl !Unpin for SelfReference {}
//
// impl SelfReference {
//     fn new(msg: &str) -> Self {
//         Self {
//             a: msg.to_string(),
//             b: std::ptr::null(),
//         }
//     }
//
//     fn init(self: Pin<&mut Self>) {
//         let ptr_to_a = &self.a as *const _;
//         unsafe {
//             self.get_unchecked_mut().b = ptr_to_a;
//         }
//     }
//
//     fn get_a(self: pin::Pin<&Self>) -> &str {
//         &self.get_ref().a
//     }
//
//     fn get_b(self: pin::Pin<&Self>) -> &str {
//         unsafe { &*self.b }
//     }
// }
//
// #[test]
// fn test_01() {
//     let mut sr_1 = SelfReference::new("hello");
//     let mut sr_1 = unsafe { Pin::new_unchecked(&mut sr_1) };
//     sr_1.as_mut().init();
//
//     let mut sr_2 = SelfReference::new("world");
//     let mut sr_2 = unsafe { Pin::new_unchecked(&mut sr_2) };
//     sr_2.as_mut().init();
//
//     println!("before swap");
//     println!(
//         "sr_1 a: {}, b: {}",
//         sr_1.as_ref().get_a(),
//         sr_1.as_ref().get_b()
//     );
//     println!(
//         "sr_2 a: {}, b: {}",
//         sr_2.as_ref().get_a(),
//         sr_2.as_ref().get_b()
//     );
//
//     println!("If we want to swap:");
//     std::mem::swap(&mut sr_1, &mut sr_2);
// }
