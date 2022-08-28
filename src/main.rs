mod basic_algo;

fn main() {
    println!("Test algo KMP, quick string find algorithm");
    let source = "this is the source string, aaabaaaab, please find aaaab";
    let target = "aaaab";
    println!("the result of no kmp match is: {:?}", basic_algo::no_kmp(source, target));
    println!("the result of kmp match is: {:?}", basic_algo::kmp(source, target));
}
