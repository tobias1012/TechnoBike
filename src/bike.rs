
use evmap_derive::ShallowCopy;

#[derive(Clone, Eq, Hash, ShallowCopy)]
pub struct Bike {
    pub id: String,
    pub watt: u16,
    pub watt_previous: u16,
    pub watt_percentage: u8,
    pub rpm: u8,
    pub fps: u8, //FPS is Functional Threshold Power? is a percentage
    
    
}

impl Bike {
    pub fn new(id: String) -> Bike{
        
        Bike {
            id: id,
            watt: 0,
            watt_previous: 0,
            watt_percentage: 0,
            rpm: 0,
            fps: 0
        }
    }

    pub fn new_val(id: String, watt: u16, watt_percentage: u8, rpm: u8) -> Bike{
        Bike {
            id: id,
            watt: watt,
            watt_previous: 0,
            watt_percentage: watt_percentage,
            rpm: rpm,
            fps: 0,
        }
    }

    pub fn update(&mut self, watt: u16, watt_percentage: u8, rpm: u8,) {
        self.watt = watt;
        self.watt_percentage = watt_percentage;
        self.rpm = rpm;

        //Calculate rpm
    }


}


impl PartialEq for Bike {
    fn eq(&self, other: &Bike) -> bool {
        self.id == other.id
    }
}

