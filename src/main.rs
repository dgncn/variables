fn main() {
    ////numbers

    // let first_integer_number : i32 = 22; //32bit
    // println!("{}", first_integer_number);
    // type_of(&first_integer_number);

    // let number2 : i8 = 127; //maximum value of 8bit
    // println!("{}", number2);
    // type_of(&number2);

    // let number3 : u16 = 34333;
    // println!("{}", number3);
    // type_of(&number3);

    // let floating_point_number1 = 4.5;
    // println!("{}", floating_point_number1);
    // type_of(&floating_point_number1);

    // let floating_point_number2 = 4.51 as f32;
    // println!("{}", floating_point_number2);
    // type_of(&floating_point_number2);

    //===========================

    ////arrays, tuples / fixed size

    // let float_array : [f32;3] = [22.3,43.12,112.3];
    // println!("{}", float_array[0]);
    // type_of(&floating_point_number2);

    // let int_tuple = ("üç",4,5);
    // println!("{:#?}", int_tuple);
    // println!("int_tuple.1: {}", int_tuple.1);
    // type_of(&int_tuple);

    //===========================

    ////String / string slice

    // let first_slice: &str = "Hey";
    // // str reference stored on stack and data stored on heap
    // let example_string: String = first_slice.to_string();
    // let _example_string_2 = example_string; //move not copy (ownership transfering)
    // //print!("{}", example_string); // value borrowed here after move

    // let y = String::from("value"); //y has ownership
    // let reference_of_string: &String = &y; // memory address to 
    // let a2 = reference_of_string;
    // println!("{}",a2);

    //===========================

    ////mutable / immutable
    // let hello_world = "Hello, Worl";
    // hello_world = hello_world + "d";
    // println!("{}", hello_world); //cannot be used to concatenate two `&str` strings, cannot add `&str` to `&str`


    // let mut hello_world = String::from("Hello, Worl");
    // hello_world = hello_world + "d";
    // let sabanci_dx = " SabancıDx";
    // hello_world.push_str(sabanci_dx);
    // println!("{}", hello_world); 


    // let mut mutable_hello_world = "Hello, Worl";
    // mutable_hello_world = mutable_hello_world  + "d";
    // println!("{}",mutable_hello_world); //cannot be used to concatenate two `&str` strings, cannot add `&str` to `&str`

    // let hello = "Hello".to_owned();//borrowed data to owned
    // let world = "World";
    // let message = format!("{}, {}", hello, world);
    // println!("{}", message); 

    //===========================
    ////Casting
    
    // let number_1 = 133_f32;
    // let float_number = 44.3;
    // let division = number_1 / float_number;
    // println!("{}",division);


    //===========================
    ////Variable Shadowing
    
    // let shadowing_example = 4;
    // //shadowing_example = 4;

    // unsafe {
    //     let address = &shadowing_example as *const i32;
    //     println!("shadowing_example değişkeninin bellek adresi: {:?}", address);
    // }
    // println!("{}",shadowing_example);
    // {
    //     let shadowing_example = 8;
    //     unsafe {
    //         let address = &shadowing_example as *const i32;
    //         println!("shadowing_example değişkeninin bellek adresi: {:?}", address);
    //     }
    //     println!("variable in scope: {}",shadowing_example);
    //     let shadowing_example_2 = 88;
    // }
    // //println!("{}",shadowing_example_2); //error[E0425]: cannot find value `shadowing_example_2` in this scope
    // println!("{}",shadowing_example);


    
    //===========================
    ////Control Flows

    ////if
    // let movie = "Lotr";
    // if movie == "Lotr" {
    //     println!("yes");
    // }
    // else {
    //     println!("no");
    // }

    ////enum
    // let enum_value : Days = Days::CustomDay { name: (String::from("Holiday")), temperature: (33) };
    // match enum_value {
    //     Days::CustomDay { name, temperature } => {
    //         println!("Name: {}, temp: {}", name, temperature);
    //     }
    //     Days::Monday => {
    //         println!("Monday");
    //     }
    //     Days::Thursday => {
    //         println!("Thursday");
    //     }
    //     Days::Sunday => {
    //         println!("Sunday");
    //     }
    // }


    // let number_2: u8 = 11;

    // match number_2 {
    //     n if n < 3 => println!("Sayı 3'ten küçük."),
    //     n if 10 > n && n >= 3 => println!("Sayı 10'dan küçük ve 3e eşit veya büyük."),
    //     _ => println!("Diğer durumlar."),
    // }

    // let number_3: i32 = 30 / 10;

    // match number_3 {
    //     2..=4 => println!("Sayi 1 ile 5 arasındadir"),
    //     5..=14 => println!("Sayi 4 ile 15 arasındadir"),
    //     _ => println!("diger durumlar")
    // }

    ////Loops
    // let mut counter = 0;
    // loop {
    //     counter += 1;
    //     if counter > 10 {
    //         println!("SabancıDx {}", counter);
    //     }
    //     if counter == 15000 {
    //         break;
    //     }
    // }

    ////While loop
    // let mut counter = 0;

    // while counter < 5 {
    //     counter += 1;
    //     println!("{}",counter);
    // }

    ////For loops

    // //1..100 100 not included
    // //1..=100 included
    // for index in 1..100 { //100 not included
    //     println!("{}", index);
    // }

    // let movies : [&str; 3] = ["The Shining", "Taxi Driver", "American History X"];
    // for movie in movies {
    //     println!("{}", movie);
    // }










    ////Option type
    //// rustta null veri tipi yok None kullanılıyor. gereksiz memory tüketimi, güvenlik gibi konularla ilgili 
    //// option değer olup olmadığını kontrol eder.

    // let result_option = get_state(11);

    // //1
    // match result_option {
    //     Some(number) => println!("{}", number),
    //     None => println!("Değer 10 dan küçük veya eşit")
    // }

    // //2
    // let value = result_option.unwrap();
    // println!("Değer: {}", value);


    
    //===========================

    ////Ownership
    
    // let s1: String = String::from("Merhaba, Rust!"); // s1, bu değerin sahibi

    // let s2 = s1; // s1'in sahipliği s2'ye transfer edildi(ownership transfering)
    // //Drop trait'i tetiklenir ve s1 ramden temizlenir
    // //https://doc.rust-lang.org/nomicon/vec/vec-dealloc.html

    // // println!("s1: {}", s1); // Hata! s1 artık kullanılamaz
    // //Rust'taki Drop trait'i, özellikle kaynakların düzgün bir şekilde temizlenmesi gerektiği senaryolarda önemlidir, örneğin dosya işlemleri veya ağ bağlantıları gibi kaynak yönetimi gerektiren durumlarda kullanılır.
    // println!("s2: {}", s2); // s2, bu değeri kullanabilir 
    



    //===========================

    ////Borrowing
    
    // let mut x = String::from("hello");
    // println!("{}", x);
    // let _y = &x; //x in referansı değişkene atandı. x'in referansi _y ye ödünç verildi 
    // xte yapılacak alttaki gibi bir değişiklik yeni bir allocation yapacağından dolayı _y değişkeni eski referansta kalacağından hata oluşur
    // x = String::from("hello1"); //cannot assign to `x` because it is borrowed
    // println!("{}", x);
    // println!("{}", _y); //_y x in eski referansına baktığı için kod hata verir.


    // let mut test_number = TestNum { number: 15 };
    // let test_ref = &test_number;
    // test_number = TestNum { number: 6 };
    // // error: cannot assign to `test_number` because it is borrowed
    // println!("Num: {}, Ref: {}", test_number.number, test_ref.number);


    ////mutable borrowing 1
    // let mut x = 5;
    // let y = &mut x; // y, x değerini değiştirilebilir bir şekilde ödünç alıyor
    // *y += 1; // y pointerinın gösterdiği heap bellekteki değer => x değerini artırabiliriz
    // println!("x: {}", x);

    
    ////mutable borrowing 2
    // let mut xy = 15;
    
    // set_new_xy_value(&mut xy); // metoda xy değişkeni değişebilir şekilde ödünç verildi ve değiştirildi. 
    // //ancak xy'nin ownership'i fonksiyona transfer edilmedi.
    
    // println!("xy: {}", xy);

    ////immutable borrowing
    ////senaryo immutable olduğundan değişiklik yapılamaz. 

    // let imm = Person {name: "John Connor".to_string(), age : 10};
    // print_person_informations(&imm); //immutable borrowing

    ////mutable borrowing 3
    // let mut john = Person {
    //     name: String::from("John Connor"),
    //     age: 25,
    // };

    // println!("Önce: {} - {}", john.name, john.age);

    // change_age(&mut john, 10); //mutable borrowing

    // println!("Sonra: {} - {}", john.name, john.age);


    ////borrowing 4
    // let mut taxi_driver = Person { name: String::from("Travis Bickle"), age: 26};
    // println!("outer1:{}", taxi_driver.name);

    // {
    //     let new_driver = &mut taxi_driver; 
    //     *new_driver = Person { name: String::from("Travis Bickle_"), age: 25};
    //     println!("inner new: {}", new_driver.name);
    //     println!("inner old: {}", taxi_driver.name);
    // }
    // println!("outer:{}", taxi_driver.name);




    ////Lifetimes
    ////lifetimes yapısı sadece referenced by value larda kullanılabilir
    ////1
    // let x: i32 = 42;
    // let r: &i32; // r adında bir i32 referansı tanımladık

    // {
    //     let y: i32 = 24;
    //     r = &y; // r, y'nin ömrü boyunca geçerli //`y` dropped here while still borrowed
    //     println!("r: {}", r);
    // } // y'nin ömrü burada sona erer //`y` dropped here while still borrowed
    // println!("r: {}", r); //throws

    ////2
    let x: i32 = 42;
    let y: i32 = 24;

    let max_value: &i32;
    {
        let x_ref: &i32 = &x;
        max_value = get_max(x_ref, &y); // x_ref ve y, farklı lifetime'lara sahip
    } // x_ref'nin ömrü burada sona erer
    println!("En büyük sayı: {}", max_value);

}

// fn type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
//     println!("===================");
// }

// struct Person {
//     name: String,
//     age : u8
// }

// fn print_person_informations(person : &Person) {
//     println!("Name: {}, Age: {}", person.name, person.age);
// }

// fn change_age(person: &mut Person, new_age: u8) {
//     person.age = new_age;
// }

// struct TestNum {
//     number: u8,
// }

// enum Days {
//     Monday,
//     Thursday,
//     Sunday,
//     CustomDay {name: String, temperature: i8}
// }

// fn get_state(number: u8) -> Option<u8>{

//     match number {
//         n if  n > 10 => Some(n),
//         _ => None,
//     }
// }


// //https://doc.rust-lang.org/nomicon/vec/vec-dealloc.html drop trait
// impl<T> Drop for Vec<T> {
//     fn drop(&mut self) {
//         if self.cap != 0 {
//             while let Some(_) = self.pop() { }
//             let layout = Layout::array::<T>(self.cap).unwrap();
//             unsafe {
//                 alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
//             }
//         }
//     }
// }

// fn set_new_xy_value(num_ref : &mut u8) {
//     *num_ref = *num_ref + 1;
// }

fn get_max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}