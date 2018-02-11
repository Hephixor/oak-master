fn main() {
    let c = 25;
    let f = 110;
    let cf = convert_tempf(c);
    let fc = convert_tempc(f);
    println!("{} celsius = {} farenheit, {} farenheit = {} celsius", c, cf, f, fc);

}

fn convert_tempf(c:i32) -> i32 {
    (c+32)*2    
}

fn convert_tempc(f:i32) -> i32 {
    (f-32)/2
}


