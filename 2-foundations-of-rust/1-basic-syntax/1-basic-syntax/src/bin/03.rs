fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut min = input[0];
    let mut maxi = input[0];

    for x in input {
        if x < min {
            min = x;
        }
        if x > maxi {
            maxi = x;
        }
    } 

    println!("{} is largest and {} is smallest", maxi, min);
}
