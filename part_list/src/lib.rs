#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let input = vec!["I", "wish", "I", "hadn't", "come"];
        let output = "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)";

        assert_eq!(output, part_list(input));
    }
}

pub fn part_list(arr: Vec<&str>) -> String {
    let mut res = String::new();

    for i in 0..arr.len() - 1 {
        let mut line = String::from("(");

        for (index, word) in arr.iter().enumerate() {
            println!("{} {} {}", index, word, arr.len());
            if i == index {
                line.push_str(&format!("{}, ", word)[..]);
            } else if index == arr.len() - 1 {
                line.push_str(&format!("{})", word)[..]);
            } else {
                line.push_str(&format!("{} ", word)[..]);
            }
        }

        res.push_str(&line);
    }

    res
}
