enum traffic_light {
    Red,
    Green,
    Yellow,
}

impl traffic_light {
    fn time(&self) -> u8 {
        match &self {
            traffic_light::Red => {
                50
            },
            traffic_light::Green => {
                60
            },
            traffic_light::Yellow => {
                70
            },
            _ => {
                10
            }
        }
    }
}
fn main() {
    let light = traffic_light :: Red;
    println!("light is {}",light.time());

    let light = traffic_light :: Yellow;
    println!("light is {}",light.time());

    let light = traffic_light :: Green;
    println!("light is {}",light.time());

}
