use crate::util::*;

pub trait Light {
    fn set_photon_num(&mut self, num : u32);
    fn is_lighting(&self) -> bool;
    fn gen_photon(&mut self) -> Option<Photon>;
}

pub struct DotLight {
    pos: Vector3,
    num : u32
}

impl Light for DotLight {
    fn set_photon_num(&mut self, num : u32) {
        self.num = num;
    }

    fn is_lighting(&self) -> bool {
        self.num > 0
    }

    fn gen_photon(&mut self) -> Option<Photon> {
        if self.num == 0 { return None; }
        self.num -= 1;
        Some(Photon { 
            ray : Ray { o : self.pos, d : Vector3::random(), }, 
            power : Color::new(1.0, 1.0, 1.0), 
        })
    }
}

impl DotLight {
    pub fn new(pos : Vector3, num : u32) -> Self {
        DotLight { pos ,num }
    }
}
