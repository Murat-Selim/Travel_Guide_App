fn main() {
    let travel = create_travel(1, "Turkey".to_string(), "Istanbul".to_string(), "Travel".to_string(), "Travel is Turkey".to_string());
        
    println!("Created Travel is {:?}", travel);

    let get_travel_data = get_travel_data(travel);

    for data in get_travel_data {
        println!("{data}");
    }

}

#[derive(Debug)]

pub struct Travel {
    id: u32,
    country: String,
    city: String,
    title: String,
    desc: String
}

pub fn get_travel_data(travel: Travel) -> [String; 4]{
    let country = travel.country;
    let city = travel.city;
    let title = travel.title;
    let desc = travel.desc;

    let data: [String; 4] = [country, city, title, desc];

    data
}

pub fn create_travel(id: u32, country: String, city: String, title: String, desc: String) -> Travel {
    let travel = Travel {
        id,
        country,
        city,
        title,
        desc
    };

    travel
}



