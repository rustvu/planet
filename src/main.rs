#[derive(Debug, Clone)]
struct Planet {
    name: String,
    radius: f64, // meters
    mass: f64,   // kg
}

impl Planet {
    // Associated constant: Gravitational constant (G)
    const G: f64 = 6.67e-11;

    // Associated function: "constructor"
    fn new(name: &str, radius: f64, mass: f64) -> Self {
        Self {
            name: name.to_string(),
            radius,
            mass,
        }
    }

    // Method taking a shared reference
    fn surface_gravity(&self) -> f64 {
        Self::G * self.mass / (self.radius * self.radius)
    }

    // Method taking a mutable reference
    fn shrink(&mut self, scale: f64) {
        self.radius *= scale;
    }

    // Method taking ownership
    fn annihilate(self) {
        println!("Goodbye, {}!", self.name);
    }
}

fn main() {
    let earth = Planet::new("Earth", 6.378e6, 5.972e24);
    println!("{earth:?}");

    let mut gaia = earth.clone();
    gaia.name = "Gaia".to_string();
    gaia.shrink(0.5);
    println!("{:?}", gaia);
    gaia.annihilate();

    println!("{:?}, surface gravity: {}", earth, earth.surface_gravity());
}
