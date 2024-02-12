use half::f16;
 
#[derive(Debug, Copy, Clone)]
pub struct ParticleType {
    pub id: u32,
    pub color: [u8; 4], // red green blue and alpha each 1 byte. i'd love to spell it colour but well for some reason i am making this code internationally readable so color it is
    pub state: u8, // 0 for solid, 1 for powdery, 2 for fluid, 3 for gas
    pub density: f16, // let's assume grams/cm^3
    pub melting_temperature: u16, // in Kelvin
    pub boiling_temperature: u16, // also Kelvin
//    ignition_temperature: u16, // you know the drill, but also no way to turn this off for now
//    ignition_energy: u16, // how much energy will the particle emit over it burning
//    burn_damage_per_second: u16, //this dictates how fast the particle will burn
//    max_durability: u16, // how strong the particle is, this includes burning
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub temperature: u16,
    pub color_noise: u8, // this gets subtracted from the color value
    //    velocity: [f16; 2],
    //    durability: u16
}

#[derive(Debug, Clone)]
pub struct ParticleSim {
    pub particles: Vec<Particle>, // a single vector index to it by [x + y * width]
    pub width: usize,
    pub height: usize,
}

impl ParticleSim{
    pub fn new(width: usize, height: usize, init_particle: Particle) -> ParticleSim{
        return ParticleSim{
            particles: vec![init_particle; width * height],
            width,
            height,
        }
    }

    pub fn particle_at(&self, x: usize, y: usize) -> Particle{
        return self.particles[x + y * self.width];
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle){
        self.particles[x + y * self.width] = particle;
    }

    pub fn get_particle_color(&self, x: usize, y: usize) -> [u8; 3]{
        
        let particle = self.particle_at(x, y);
        //let mut rgb = particle.particle_type.color;

        let mut bb_rgb = [0.0, 0.0, 0.0];

// Calculate Blackbody Radiation
        let temperature = particle.temperature as f32 / 100.0;
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
        return [bb_rgb[0] as u8, bb_rgb[1] as u8, bb_rgb[2] as u8];

        /*
        for i in 0..rgb.len()-1{
            let mut color: i32 = rgb [i] as i32;
            
            color += particle.color_noise as i32 - 128;


            if color > 255{
                color = 255;
            } else if color < 0{
                color = 0;
            }
            rgb[i] = color as u8;
        }    
        return [rgb[0], rgb[1], rgb[2]];*/
    }
}

