// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    let mut input:String = input.to_string();
    while &input[0..=0]==" " {
            input.remove(0);
    }
    for c in (0..input.len()).rev() {
        
        println!("rev: {}",c);
        if &input[c..=c] == " " {
            input.remove(c);
        } else {
            break;
        }


    }
    input
}
fn find_match(source: &str, pattern: &str)-> i64 {
    let source = source.to_string();
    let pattern = pattern.to_string();
    let mut index:i64 = -1;
    let mut charsMatching:usize = 0;
    for i in 0..source.len() {
        if source[i..=i] == pattern[charsMatching..charsMatching+1] {
                                                                 //if it works it works
            if charsMatching == 0 {
                index = i as i64;
            }
            charsMatching += 1;
            if charsMatching == pattern.len() {
                return index
            }
        } else {charsMatching = 0;}
    }
    -1
}
fn compose_me(input: &str) -> String {
    input.to_owned() + " world!"    
}

fn replace_me(input: &str) -> String {
    let pattern = "cars";
    let balloonIndex = find_match(input,pattern).try_into().unwrap();
    let mut input = input.to_string();
    for idx in (balloonIndex..balloonIndex+pattern.len()).rev() {
        input.remove(idx);
    }
    input.insert_str(balloonIndex,"balloons");
    input
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
