use std::io;

fn main() {
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    let id: u32 = id.trim().parse().expect("failed to convert");

    let mut country = String::new();
    io::stdin().read_line(&mut country).expect("failed to read");

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("failed to read");

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("failed to read");

    let mut desc = String::new();
    io::stdin().read_line(&mut desc).expect("failed to read");

    let travel = create_travel(id, country, city, title, desc);
        
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



