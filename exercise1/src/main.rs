use::std::collections::HashMap;

fn calc_average(vec: &Vec<i32>) -> f64{
    let mut sum = 0;
    for value in vec{
        sum += value;
    }
    f64::from(sum) / (vec.len() as f64)
}
fn get_median(vec: &Vec<i32>) -> f64{
    let mut sort_vec = vec.clone();
    sort_vec.sort();
    let index = sort_vec.len() / 2;
    if sort_vec.len() % 2 == 0{
        f64::from(sort_vec[index-1]+sort_vec[index]) / 2.0
    } else {
        f64::from(vec[index])
    }
}
fn mode_of_list(vec: &Vec<i32>) -> (i32,i32){
    let mut hashmap: HashMap<i32,i32> = HashMap::new();
    let mut max_count: (i32,i32) = (0,0);
    for value in vec{
        let count = hashmap.entry(*value).or_insert(0);
        *count += 1;
    }
    for (key,value) in hashmap{
        if value > max_count.1 {
            max_count = (key,value);
        }
    }
    max_count

}
fn main() {
    let vector = vec![1, 3, 4, 2, 6, 5, 8, 7];
    let result1 = calc_average(&vector);
    let result2 = get_median(&vector);
    let result3 = mode_of_list(&vector);
    println!("average value: {:?}", result1);
    println!("madian value: {:?}", result2);
    println!("mode_of_list: {:?}", result3);


    
}
