pub trait Area {
    fn calc_area(&self) -> u32;
}

struct Triangle {
    x: u32,
    y: u32
}

impl Area for Triangle {
    fn calc_area(&self) -> u32{
        let area = self.x * self.y / 2;
        println!("{}", area);
        area
    }
}


fn main() {
    let triangle = Triangle{x: 50, y: 50};
    show_area(&triangle);
}

fn show_area<T>(graphic: &T) 
where T: Area
{
    graphic.calc_area();
}
