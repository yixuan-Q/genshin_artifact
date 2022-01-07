use crate::common::Element;

pub struct Enemy {
    pub level: i32,
    pub def_minus: f64,
    pub electro_res: f64,
    pub pyro_res: f64,
    pub hydro_res: f64,
    pub cryo_res: f64,
    pub anemo_res: f64,
    pub geo_res: f64,
    pub dendro_res: f64,
    pub physical_res: f64,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            level: 80,
            def_minus: 0.0,
            electro_res: 0.1,
            pyro_res: 0.1,
            hydro_res: 0.1,
            cryo_res: 0.1,
            anemo_res: 0.1,
            geo_res: 0.1,
            dendro_res: 0.1,
            physical_res: 0.1,
        }
    }
}

impl Enemy {
    pub fn get_defensive_ratio(&self, character_level: i32) -> f64 {
        let def = self.level as f64 + 100.0;
        let def_minus = self.def_minus.min(1.0).max(0.0);
        (character_level as f64 + 100.0) / ((1.0 - def_minus) * def + character_level as f64 + 100.0)
    }

    pub fn get_resistance_ratio(&self, element: Element) -> f64 {
        let res = match element {
            Element::Electro => self.electro_res,
            Element::Pyro => self.pyro_res,
            Element::Hydro => self.hydro_res,
            Element::Cryo => self.cryo_res,
            Element::Anemo => self.anemo_res,
            Element::Geo => self.geo_res,
            Element::Dendro => self.dendro_res,
            Element::Physical => self.physical_res,
            _ => unreachable!(),
        };

        if res > 0.75 {
            25.0 / (25.0 + res)
        } else if res > 0.0 {
            1.0 - res
        } else {
            1.0 - res / 2.0
        }
    }
}