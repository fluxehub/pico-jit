use pico_emit::registers;
use pico_emit::registers::types::{HighRegister, LowRegister, RegisterType};
use rand::distributions::uniform::SampleRange;
use rand::rngs::ThreadRng;
use rand::Rng;
use ux2::{u3, u5, u7};

pub struct RandomGenerator {
    rng: ThreadRng,
    selected_registers: Vec<u16>,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            selected_registers: Vec::new(),
        }
    }

    fn random_reg_number(&mut self, range: impl SampleRange<u16> + Clone) -> u16 {
        loop {
            let reg = self.rng.gen_range(range.clone());
            if !self.selected_registers.contains(&reg) {
                self.selected_registers.push(reg);
                return reg;
            }
        }
    }

    pub fn random_low_register(&mut self) -> LowRegister {
        match self.random_reg_number(0..=7) {
            0 => registers::r0,
            1 => registers::r1,
            2 => registers::r2,
            3 => registers::r3,
            4 => registers::r4,
            5 => registers::r5,
            6 => registers::r6,
            7 => registers::r7,
            _ => unreachable!(),
        }
    }

    pub fn random_high_register(&mut self) -> HighRegister {
        match self.random_reg_number(8..=12) {
            8 => registers::r8,
            9 => registers::r9,
            10 => registers::r10,
            11 => registers::r11,
            12 => registers::r12,
            _ => unreachable!(),
        }
    }

    pub fn random_register(&mut self) -> RegisterType {
        match self.rng.gen_range(0..=1) {
            0 => RegisterType::Low(self.random_low_register()),
            1 => RegisterType::High(self.random_high_register()),
            _ => unreachable!(),
        }
    }

    pub fn random_u3(&mut self) -> u3 {
        u3::new(self.rng.gen_range(0..=0b111) as u8)
    }

    pub fn random_u5(&mut self) -> u5 {
        u5::new(self.rng.gen_range(0..=0b11111) as u8)
    }

    pub fn random_u7(&mut self) -> u7 {
        u7::new(self.rng.gen_range(0..=0b1111111) as u8)
    }

    pub fn random_u8(&mut self) -> u8 {
        rand::random::<u8>()
    }

    pub fn random_u32(&mut self) -> u32 {
        rand::random::<u32>()
    }

    pub fn random_bool(&mut self) -> bool {
        rand::random::<bool>()
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Random {
    fn random(generator: &mut RandomGenerator) -> Self;
}

impl Random for LowRegister {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_low_register()
    }
}

impl Random for HighRegister {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_high_register()
    }
}

impl Random for RegisterType {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_register()
    }
}

impl Random for u3 {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_u3()
    }
}

impl Random for u5 {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_u5()
    }
}

impl Random for u7 {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_u7()
    }
}

impl Random for u8 {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_u8()
    }
}

impl Random for u32 {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_u32()
    }
}

impl Random for bool {
    fn random(generator: &mut RandomGenerator) -> Self {
        generator.random_bool()
    }
}

// Returns of a tuple of random data matching the types passed in
#[macro_export]
macro_rules! random_data (
    ($($t:ty),*) => {
        {
            let mut generator = RandomGenerator::new();
            ($(<$t>::random(&mut generator)),*)
        }
    };
);
