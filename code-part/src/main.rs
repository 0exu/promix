/*
Well guys this is the starting of my coding part where i am declearing the main where the callings will be avaliable and the runner will be created.
This will handle the entire project so be respectfull to this rs file. Now i am using rust for this entire project.
*/
// 1. use cargo for installation any library and rustc for single code compilation and debug
// 2. Please don't use ai code agents this is completely prohibited because they will put huge garbadge to this language
// 3. Always try to commit on newer branch if that is a feature and thing.
// Let's build this thing.... ;-)

mod check_os; //included chech_os fun from check_os/checkos.rs

fn main() {
    println!("Starting the engineering of promix...");
    let os = check_os::checkos::check_os();
    match os {
        1 => println!("Linux distro, Not assuming ubuntu... :)"),
        2 => println!("Windows shit... :-("),
        3 => println!("MacOs the richy viber... huh!"),
        _ => println!("Okay, I need to research about this one..."),
    }
}
