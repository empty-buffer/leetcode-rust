use std::collections::HashMap;

fn main() {
    let table: Vec<Vec<String>> = vec![
        vec!["rust".to_string(), "go".to_string()],
        vec!["java".to_string(), "go".to_string()],
        vec!["java".to_string(), "rust".to_string()],
    ];

    let score: Vec<i32> = vec![0, 1, 1];

    let (mut team, mut best_score) = (String::new(), 0);

    let mut map: HashMap<&str, i32> = HashMap::new();

    for (i, &score_val) in score.iter().enumerate() {
        let score = map.entry(&table[i][score_val as usize]).or_insert(0);
        *score += 3;

        if *score > best_score {
            best_score = *score;
            team = table[i][score_val as usize].clone()
        }

        // match map.get(&table[i][score[i] as usize]) {
        //     Some(&value) => {
        //         if value + 3 > best_score {
        //             team = table[i][score[i] as usize].clone();
        //         }

        //         map.insert(table[i][score[i] as usize].clone(), value + 3)
        //     }
        //     None => map.insert(table[i][score[i] as usize].clone(), 3),
        // };
    }

    println!("{:?}", team);

    debug_assert_eq!("rust".to_string(), team);
}
