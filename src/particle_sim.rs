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

    pub fn get_density(&self) -> f32{
        let mut density = f32::from(self.particle_type.density);
        density -= density / ((self.get_temperature() as f32 / 100.0) + 1.0);
        return density
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

    pub fn fast_get_color(&self) {
        let blackbody_lut = [
            [255,56,0],
            [255,83,0],
            [255,101,0],
            [255,115,0],
            [255,126,0],
            [255,137,18],
            [255,147,44],
            [255,157,63],
            [255,165,79],
            [255,173,94],
            [255,180,107],
            [255,187,120],
            [255,193,132],
            [255,199,143],
            [255,204,153],
            [255,209,163],
            [255,213,173],
            [255,217,182],
            [255,221,190],
            [255,225,198],
            [255,228,206],
            [255,232,213],
            [255,235,220],
            [255,238,227],
            [255,240,233],
            [255,243,239],
            [255,245,245],
            [255,248,251],
            [254,249,255],
            [249,246,255],
            [245,243,255],
            [240,241,255],
            [237,239,255],
            [233,237,255],
            [230,235,255],
        ];
    }

    pub fn get_color(&self) -> [u8; 3]{

    
        let mut rgb = self.particle_type.color;

        //return [rgb[0], rgb [1], rgb[2]];

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
            bb_rgb[i] = bb_rgb[i] * (self.get_temperature() as f32 / 5000.0);   

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

    pub fn particle_at(&self, x: usize, y: usize) -> &Particle{
        return &(self.particles[y * self.width + x]);
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

    pub fn set_particle_energy(&mut self, x: usize, y: usize, energy: u32){
        if self.particle_exists(x, y){
            self.particles[y * self.width + x].energy = energy;
        } 

    }

    fn set_iterated(&mut self, x: usize, y: usize, iterated_over:bool){
        self.particles[y * self.width + x].iterated_over = iterated_over;
    }

    pub fn render_pixels(&self) -> Vec<[u8; 3]>{
        let mut out: Vec<[u8; 3]> = vec![[0, 0, 0]; self.width * self.height];
        for x in 0..self.width{
            for y in 0..self.height {
                out[x + y*self.height] = self.particle_at(x, y).get_color();
            }
        }
        return out
    }

    pub fn simulate_gravity(&mut self) {
        for yn in 0..self.height {
            for xn in 0..self.width {
                let x: i32 = (self.width-xn-1) as i32;
                let y: i32 = (self.height-yn-1) as i32;

                if self.particle_exists(x as usize, y as usize){
                    //println!("1, {}", self.particle_at(x as usize, y as usize).particle_type.state);
                    let state = self.particle_at(x as usize, y as usize).particle_type.state;

                    if state == 1{
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
                                self.particle_at(x as usize, y as usize).get_density() > self.particle_at(xi, yi).get_density() 
                                && self.particle_at(xi, yi).particle_type.state > 1
                                && !self.particle_at(x as usize, y as usize).iterated_over
                                {
                                    if i == 0 || self.particle_at(xi, y as usize).particle_type.state > 1{
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
        }
        for xn in 0..self.width {
            for yn in 0..self.height {
                let x = xn as i32;
                let y = yn as i32;

                if self.particle_exists(x as usize, y as usize){
                    //println!("1, {}", self.particle_at(x as usize, y as usize).particle_type.state);
                    let state = self.particle_at(x as usize, y as usize).particle_type.state;
                    
                    if state == 3 {
                        let mut xoffsets = [0, 1, -1, 1, -1];
                        if rand::random::<bool>(){
                            xoffsets = [0, -1, 1, -1, 1];
                        }
                        let yoffsets = [-1, -1, -1, 0, 0];
                        let mut moved = false;
                        for i in 0..xoffsets.len(){
                            let xi = (x + xoffsets[i]) as usize;
                            let yi = (y + yoffsets[i]) as usize;
                            if !moved && self.particle_exists(xi, yi){
                                if 
                                self.particle_at(x as usize, y as usize).get_density() > self.particle_at(xi, yi).get_density() 
                                && self.particle_at(xi, yi).particle_type.state > 1
                                && !self.particle_at(x as usize, y as usize).iterated_over
                                {
                                    if i == 0 || self.particle_at(xi, y as usize).particle_type.state > 1{
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
        }

        for x in 0..self.width {
            for y in 0..self.height {
                 if self.particle_exists(x as usize, y as usize) {
                     self.set_iterated(x, y, false);
                 }
            }
        }
    }
    pub fn simulate_heat(&mut self){
       for x in 0..self.width{
            for y in 0..self.height{
                if self.particle_exists(x, y) {
                    let xoffsets = [-1, -1, -1, 0, 0, 1, 1, 1];
                    let yoffsets = [-1, 0, 1, -1, 1, -1, 0, 1];
                    let mut energy_moved: i32 = 0;

                    let particle = *self.particle_at(x, y);

                    for i in 0..xoffsets.len(){
                        let xo = x as i32 + xoffsets[i];
                        let yo = y as i32 + yoffsets[i];
                        if self.particle_exists(xo as usize, yo as usize){
                            let neighbor_particle = self.particle_at(xo as usize, yo as usize);
                            
                            let energy_delta: i32 = particle.energy as i32 - neighbor_particle.energy as i32;
                            energy_moved += energy_delta as i32 / (32 * 5);
                            let np_energy = neighbor_particle.energy as i32 + (energy_delta / (32 * 5));
                            
                            self.set_particle_energy(xo as usize, yo as usize, np_energy as u32);
                        }
                    }
                    let particle_energy = particle.energy as i32 - energy_moved;
                    self.set_particle_energy(x, y, particle_energy as u32);
                }
            }
        }
    }
}
