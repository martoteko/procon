use proconio::input;

fn main() {
    input!(
        x: String,
        y: String,
    );

    let v = vec!["Ocelot", "Serval", "Lynx"];

    let v_x = v.iter().position(|&s| s == x).unwrap();
    let v_y = v.iter().position(|&s| s == y).unwrap();

    if v_x >= v_y {
        println!("Yes");
        return;
    } else  {
        println!("No");
        return;
        
    }
}
