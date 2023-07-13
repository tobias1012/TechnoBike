
use evmap_derive::ShallowCopy;

#[derive(Clone, Eq, Hash, ShallowCopy)]
pub struct Bike {
    pub id: String,
    pub watt: u16,
    pub watt_previous: u16,
    pub watt_percentage: u8,
    pub rpm: u8,
    pub fps: u8, //FPS is Functional Threshold Power? is a percentage

    //Values not sent by the bike, this is to be handled by for speed and such to work
    pub name: String,
    pub age: u8,
    pub weight: u16, //In Kilos
    
}

impl Bike {
    pub fn new(id: String) -> Bike{
        
        Bike {
            id: id,
            watt: 0,
            watt_previous: 0,
            watt_percentage: 0,
            rpm: 0,
            fps: 0,

            name: String::from("null"),
            age: 0,
            weight: 75,
        }
    }

    pub fn new_val(id: String, watt: u16, watt_percentage: u8, rpm: u8, name: String, age: u8, weight: u16) -> Bike{
        Bike {
            id: id,
            watt: watt,
            watt_previous: 0,
            watt_percentage: watt_percentage,
            rpm: rpm,
            fps: 0,

            name: name,
            age: age,
            weight: weight
        }
    }

    pub fn set_rider(&mut self, name: String, age: u8, weight: u16) {

        self.name = name;
        self.age = age;
        self.weight = weight;
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

