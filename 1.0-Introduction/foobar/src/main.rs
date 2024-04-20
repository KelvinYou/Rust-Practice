// fn foobar(n: u32) {
// 	for i in 1..=n {
// 		if i % 3 == 0 && i % 5 == 0 {
// 			println!("FooBar");
// 		} else if i % 3 == 0 {
// 			println!("Foo");
// 		} else if i % 5 == 0 {
// 			println!("Bar");
// 		} else {
// 			println!("{}", i);
// 		}
// 	}
// }

fn foobar(n: u32) {
	for i in 1..=n {
		match (i % 3, i % 5) {
			(0, 0) => println!("FooBar"),
			(0, _) => println!("Foo"),
			(_, 0) => println!("Bar"),
			_ => println!("{}", i),
		}
	}
}

fn main() {
	foobar(100);
}
