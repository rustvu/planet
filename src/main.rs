#[derive(Debug, Clone)]
struct Planet {
    name: String,
    radius: f64, // meters
    mass: f64,   // kg
}

impl Planet {
    fn new(name: &str, radius: f64, mass: f64) -> Self {
        Self {
            name: name.to_string(),
            radius,
            mass,
        }
    }

    fn surface_gravity(&self) -> f64 {
        6.67e-11 * self.mass / (self.radius * self.radius)
    }

    fn shrink(&mut self, scale: f64) {
        self.radius *= scale;
    }

    fn annihilate(self) {
        println!("Good bye, {}!", self.name);
    }
}

fn main() {
    let earth = Planet::new("Earth", 6.378e6, 5.972e24);

    let mut gaia = earth.clone();
    gaia.shrink(0.5);
    println!("{:?}", gaia);
    gaia.annihilate();

    println!("{:?}, surface gravity: {}", earth, earth.surface_gravity());
}
