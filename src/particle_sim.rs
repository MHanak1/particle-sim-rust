use half::f16;
#[derive(Debug, Copy, Clone)]
pub struct ParticleType {
    pub id: u32,
    pub color: [u8; 4], // red green blue and alpha each 1 byte. i'd love to spell it colour but well for some reason i am making this code internationally readable so color it is
    pub state: u8, // 0 for solid, 1 for powdery, 2 for fluid, 3 for gas
    pub density: f16, // let's assume grams/cm^3, 1 pixel = 5cm
    pub melting_temperature: u16, // in Kelvin
    pub boiling_temperature: u16, // also Kelvin
    pub heat_capacity: u32, // How much energy (in joules) is needed to raise the temperature of 1
                            // kg of substance by 1 degree celcius
//    ignition_temperature: u16, // you know the drill, but also no way to turn this off for now
//    burning_energy: u16, // how much energy will the particle emit over it burning
//    burn_damage_per_second: u16, //this dictates how fast the particle will burn
//    max_durability: u16, // how strong the particle is, this includes burning
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub energy: u32, // in Joules.
    pub color_noise: u8, // this gets subtracted from the color value
    iterated_over: bool,
    //    velocity: [f16; 2],
    //    durability: u16
}

#[derive(Debug, Clone)]
pub struct ParticleSim {
    pub particles: Vec<Particle>, // a single vector index to it by [x + y * width]
    pub width: usize,
    pub height: usize,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Particle{
        return Particle{
            particle_type,
            energy: 0,
            color_noise: 128,
            iterated_over: false,
        }
    }

    pub fn set_noise_value(&mut self, value: u8) -> Particle {
        self.color_noise = value;
        return *self
    }

    pub fn get_temperature(&self) -> u32 {
        return self.energy / self.particle_type.heat_capacity as u32;
    }

    pub fn set_temperature(&mut self, temperature: u32) -> Particle {
        self.energy = self.particle_type.heat_capacity * temperature;
        return *self
    }

    pub fn get_color(&self) -> [u8; 3]{
    
        let mut rgb = self.particle_type.color;

        let mut bb_rgb = [0.0, 0.0, 0.0];

// Calculate Blackbody Radiation
        let temperature = self.get_temperature() as f32 / 100.0;
        if temperature <= 66.0{
            bb_rgb[0] = 255.0;
            bb_rgb[1] = 99.4708025861 * temperature.ln() - 161.1195681661;
            bb_rgb[2] = 138.5177312231 * (temperature - 10.0).ln() - 305.0447927307;
        }
        else {
            bb_rgb[0] = 329.698727446 * ((temperature - 60.0).powf( -0.1332047592));
            bb_rgb[1] = 288.1221695283 * ((temperature - 60.0).powf( -0.0755148492));
            bb_rgb[2] = 255.0;

        }

       
            
        //println!("{}, {}, {}", bb_rgb[0] as u8, bb_rgb[1] as u8, bb_rgb[2] as u8);
        //return [bb_rgb[0] as u8, bb_rgb[1] as u8, bb_rgb[2] as u8];

        for i in 0..rgb.len()-1{
            bb_rgb[i] = bb_rgb[i] * (temperature / 50.0);   

            let mut color: i32 = rgb [i] as i32;
            
            color += self.color_noise as i32 - 128;
            color += bb_rgb[i] as i32;
            if color > 255{
                color = 255;
            } else if color < 0{
                color = 0;
            }
            rgb[i] = color as u8;
        }    
        return [rgb[0], rgb[1], rgb[2]];

    }
}


impl ParticleSim{
    pub fn new(width: usize, height: usize, init_particle: Particle) -> ParticleSim{
        return ParticleSim{
            particles: vec![init_particle; width * height],
            width,
            height,
        }
    }

    pub fn particle_exists(&self, x: usize, y: usize) -> bool {
        return x < (self.width - 1) && y < (self.height - 1)
    }

    pub fn particle_at(&self, x: usize, y: usize) -> Particle{
        return self.particles[y * self.width + x];
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle){
        if self.particle_exists(x, y){
            self.particles[y * self.width + x] = particle;
        } 
    }

    pub fn swap_particles(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if self.particle_exists(x1, y1) && self.particle_exists(x2, y2) {
            self.particles.swap(x1 + y1 * self.width, x2 + y2 * self.width); 
        }
    }

    pub fn get_particle_color(&self, x: usize, y: usize) -> [u8; 3]{
        if self.particle_exists(x, y){
            return self.particle_at(x, y).get_color();
        }
        else {
            return [0, 0, 0];
        }
    }

    fn set_iterated(&mut self, x: usize, y: usize, iterated_over:bool){
        self.particles[y * self.width + x].iterated_over = iterated_over;
    }

    pub fn simulate_gravity(&mut self) {
        for xn in 0..self.width {
            for yn in 0..self.height {
                let x: i32 = (self.width-xn-1) as i32;
                let y: i32 = (self.height-yn-1) as i32;
                //let x = xn as i32;
                //let y = yn as i32;


                if self.particle_exists(x as usize, y as usize){
                    //println!("1, {}", self.particle_at(x as usize, y as usize).particle_type.state);
                    if self.particle_at(x as usize, y as usize).particle_type.state == 1{
                        let mut xoffsets = [0, 1, -1];
                        if rand::random::<bool>(){
                            xoffsets = [0, -1, 1];
                        }
                        let yoffsets = [1, 1, 1];
                        let mut moved = false;
                        for i in 0..xoffsets.len(){
                            let xi = (x + xoffsets[i]) as usize;
                            let yi = (y + yoffsets[i]) as usize;
                            if !moved && self.particle_exists(xi, yi){
                                if 
                                self.particle_at(x as usize, y as usize).particle_type.density > self.particle_at(xi, yi).particle_type.density 
                                && self.particle_at(xi, yi).particle_type.state > 1
                                && !self.particle_at(x as usize, y as usize).iterated_over
                                {
                                    
                                        self.set_iterated(x as usize, y as usize, true);
                                    moved = true;
                                    self.swap_particles(x as usize, y as usize, xi, yi)
                                }
                            }
                        }
                    }
                }
            }
        }
        
        for x in 0..self.width {
            for y in 0..self.height {
                 if self.particle_exists(x as usize, y as usize) {
                     self.set_iterated(x, y, false);
                 }
            }
        }
    }
}
