use std::{
    fs::File,
    io::{BufWriter, Write},
};

use clap::Parser;
use once_cell::sync::Lazy;
use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of lines, min = 10000, must be a multiple of 10000
    lines: usize,
    /// Output file
    out_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut rng = rand::thread_rng();
    // let chunks = cli.lines/10_000_000;
    let mut buffer = BufWriter::new(File::create(cli.out_file)?);

    (0..cli.lines).into_iter().for_each(|_| {
        let station = STATIONS
            .get(rng.gen_range(0..STATIONS.len()))
            .expect("should be there");
        let measurement = format!("{};{:.1}\n", station.name, station.measurement());
        buffer
            .write_all(measurement.as_bytes())
            .expect("should write");
    });

    buffer.flush().expect("should flush");

    Ok(())
}

struct Station {
    name: String,
    normal: Normal<f64>,
}

impl Station {
    fn new(name: impl ToString, avg: f64) -> Self {
        Station {
            name: name.to_string(),
            normal: Normal::new(avg, 10.0).unwrap(),
        }
    }

    fn measurement(&self) -> f64 {
        (self.normal.sample(&mut rand::thread_rng()) * 10.0).round() / 10.0
    }
}

static STATIONS: Lazy<Vec<Station>> = Lazy::new(|| {
    [
        Station::new("Abha", 18.0),
        Station::new("Abidjan", 26.0),
        Station::new("Abéché", 29.4),
        Station::new("Accra", 26.4),
        Station::new("Addis Ababa", 16.0),
        Station::new("Adelaide", 17.3),
        Station::new("Aden", 29.1),
        Station::new("Ahvaz", 25.4),
        Station::new("Albuquerque", 14.0),
        Station::new("Alexandra", 11.0),
        Station::new("Alexandria", 20.0),
        Station::new("Algiers", 18.2),
        Station::new("Alice Springs", 21.0),
        Station::new("Almaty", 10.0),
        Station::new("Amsterdam", 10.2),
        Station::new("Anadyr", -6.9),
        Station::new("Anchorage", 2.8),
        Station::new("Andorra la Vella", 9.8),
        Station::new("Ankara", 12.0),
        Station::new("Antananarivo", 17.9),
        Station::new("Antsiranana", 25.2),
        Station::new("Arkhangelsk", 1.3),
        Station::new("Ashgabat", 17.1),
        Station::new("Asmara", 15.6),
        Station::new("Assab", 30.5),
        Station::new("Astana", 3.5),
        Station::new("Athens", 19.2),
        Station::new("Atlanta", 17.0),
        Station::new("Auckland", 15.2),
        Station::new("Austin", 20.7),
        Station::new("Baghdad", 22.77),
        Station::new("Baguio", 19.5),
        Station::new("Baku", 15.1),
        Station::new("Baltimore", 13.1),
        Station::new("Bamako", 27.8),
        Station::new("Bangkok", 28.6),
        Station::new("Bangui", 26.0),
        Station::new("Banjul", 26.0),
        Station::new("Barcelona", 18.2),
        Station::new("Bata", 25.1),
        Station::new("Batumi", 14.0),
        Station::new("Beijing", 12.9),
        Station::new("Beirut", 20.9),
        Station::new("Belgrade", 12.5),
        Station::new("Belize City", 26.7),
        Station::new("Benghazi", 19.9),
        Station::new("Bergen", 7.7),
        Station::new("Berlin", 10.3),
        Station::new("Bilbao", 14.7),
        Station::new("Birao", 26.5),
        Station::new("Bishkek", 11.3),
        Station::new("Bissau", 27.0),
        Station::new("Blantyre", 22.2),
        Station::new("Bloemfontein", 15.6),
        Station::new("Boise", 11.4),
        Station::new("Bordeaux", 14.2),
        Station::new("Bosaso", 30.0),
        Station::new("Boston", 10.9),
        Station::new("Bouaké", 26.0),
        Station::new("Bratislava", 10.5),
        Station::new("Brazzaville", 25.0),
        Station::new("Bridgetown", 27.0),
        Station::new("Brisbane", 21.4),
        Station::new("Brussels", 10.5),
        Station::new("Bucharest", 10.8),
        Station::new("Budapest", 11.3),
        Station::new("Bujumbura", 23.8),
        Station::new("Bulawayo", 18.9),
        Station::new("Burnie", 13.1),
        Station::new("Busan", 15.0),
        Station::new("Cabo San Lucas", 23.9),
        Station::new("Cairns", 25.0),
        Station::new("Cairo", 21.4),
        Station::new("Calgary", 4.4),
        Station::new("Canberra", 13.1),
        Station::new("Cape Town", 16.2),
        Station::new("Changsha", 17.4),
        Station::new("Charlotte", 16.1),
        Station::new("Chiang Mai", 25.8),
        Station::new("Chicago", 9.8),
        Station::new("Chihuahua", 18.6),
        Station::new("Chișinău", 10.2),
        Station::new("Chittagong", 25.9),
        Station::new("Chongqing", 18.6),
        Station::new("Christchurch", 12.2),
        Station::new("City of San Marino", 11.8),
        Station::new("Colombo", 27.4),
        Station::new("Columbus", 11.7),
        Station::new("Conakry", 26.4),
        Station::new("Copenhagen", 9.1),
        Station::new("Cotonou", 27.2),
        Station::new("Cracow", 9.3),
        Station::new("Da Lat", 17.9),
        Station::new("Da Nang", 25.8),
        Station::new("Dakar", 24.0),
        Station::new("Dallas", 19.0),
        Station::new("Damascus", 17.0),
        Station::new("Dampier", 26.4),
        Station::new("Dar es Salaam", 25.8),
        Station::new("Darwin", 27.6),
        Station::new("Denpasar", 23.7),
        Station::new("Denver", 10.4),
        Station::new("Detroit", 10.0),
        Station::new("Dhaka", 25.9),
        Station::new("Dikson", -11.1),
        Station::new("Dili", 26.6),
        Station::new("Djibouti", 29.9),
        Station::new("Dodoma", 22.7),
        Station::new("Dolisie", 24.0),
        Station::new("Douala", 26.7),
        Station::new("Dubai", 26.9),
        Station::new("Dublin", 9.8),
        Station::new("Dunedin", 11.1),
        Station::new("Durban", 20.6),
        Station::new("Dushanbe", 14.7),
        Station::new("Edinburgh", 9.3),
        Station::new("Edmonton", 4.2),
        Station::new("El Paso", 18.1),
        Station::new("Entebbe", 21.0),
        Station::new("Erbil", 19.5),
        Station::new("Erzurum", 5.1),
        Station::new("Fairbanks", -2.3),
        Station::new("Fianarantsoa", 17.9),
        Station::new("Flores,  Petén", 26.4),
        Station::new("Frankfurt", 10.6),
        Station::new("Fresno", 17.9),
        Station::new("Fukuoka", 17.0),
        Station::new("Gabès", 19.5),
        Station::new("Gaborone", 21.0),
        Station::new("Gagnoa", 26.0),
        Station::new("Gangtok", 15.2),
        Station::new("Garissa", 29.3),
        Station::new("Garoua", 28.3),
        Station::new("George Town", 27.9),
        Station::new("Ghanzi", 21.4),
        Station::new("Gjoa Haven", -14.4),
        Station::new("Guadalajara", 20.9),
        Station::new("Guangzhou", 22.4),
        Station::new("Guatemala City", 20.4),
        Station::new("Halifax", 7.5),
        Station::new("Hamburg", 9.7),
        Station::new("Hamilton", 13.8),
        Station::new("Hanga Roa", 20.5),
        Station::new("Hanoi", 23.6),
        Station::new("Harare", 18.4),
        Station::new("Harbin", 5.0),
        Station::new("Hargeisa", 21.7),
        Station::new("Hat Yai", 27.0),
        Station::new("Havana", 25.2),
        Station::new("Helsinki", 5.9),
        Station::new("Heraklion", 18.9),
        Station::new("Hiroshima", 16.3),
        Station::new("Ho Chi Minh City", 27.4),
        Station::new("Hobart", 12.7),
        Station::new("Hong Kong", 23.3),
        Station::new("Honiara", 26.5),
        Station::new("Honolulu", 25.4),
        Station::new("Houston", 20.8),
        Station::new("Ifrane", 11.4),
        Station::new("Indianapolis", 11.8),
        Station::new("Iqaluit", -9.3),
        Station::new("Irkutsk", 1.0),
        Station::new("Istanbul", 13.9),
        Station::new("İzmir", 17.9),
        Station::new("Jacksonville", 20.3),
        Station::new("Jakarta", 26.7),
        Station::new("Jayapura", 27.0),
        Station::new("Jerusalem", 18.3),
        Station::new("Johannesburg", 15.5),
        Station::new("Jos", 22.8),
        Station::new("Juba", 27.8),
        Station::new("Kabul", 12.1),
        Station::new("Kampala", 20.0),
        Station::new("Kandi", 27.7),
        Station::new("Kankan", 26.5),
        Station::new("Kano", 26.4),
        Station::new("Kansas City", 12.5),
        Station::new("Karachi", 26.0),
        Station::new("Karonga", 24.4),
        Station::new("Kathmandu", 18.3),
        Station::new("Khartoum", 29.9),
        Station::new("Kingston", 27.4),
        Station::new("Kinshasa", 25.3),
        Station::new("Kolkata", 26.7),
        Station::new("Kuala Lumpur", 27.3),
        Station::new("Kumasi", 26.0),
        Station::new("Kunming", 15.7),
        Station::new("Kuopio", 3.4),
        Station::new("Kuwait City", 25.7),
        Station::new("Kyiv", 8.4),
        Station::new("Kyoto", 15.8),
        Station::new("La Ceiba", 26.2),
        Station::new("La Paz", 23.7),
        Station::new("Lagos", 26.8),
        Station::new("Lahore", 24.3),
        Station::new("Lake Havasu City", 23.7),
        Station::new("Lake Tekapo", 8.7),
        Station::new("Las Palmas de Gran Canaria", 21.2),
        Station::new("Las Vegas", 20.3),
        Station::new("Launceston", 13.1),
        Station::new("Lhasa", 7.6),
        Station::new("Libreville", 25.9),
        Station::new("Lisbon", 17.5),
        Station::new("Livingstone", 21.8),
        Station::new("Ljubljana", 10.9),
        Station::new("Lodwar", 29.3),
        Station::new("Lomé", 26.9),
        Station::new("London", 11.3),
        Station::new("Los Angeles", 18.6),
        Station::new("Louisville", 13.9),
        Station::new("Luanda", 25.8),
        Station::new("Lubumbashi", 20.8),
        Station::new("Lusaka", 19.9),
        Station::new("Luxembourg City", 9.3),
        Station::new("Lviv", 7.8),
        Station::new("Lyon", 12.5),
        Station::new("Madrid", 15.0),
        Station::new("Mahajanga", 26.3),
        Station::new("Makassar", 26.7),
        Station::new("Makurdi", 26.0),
        Station::new("Malabo", 26.3),
        Station::new("Malé", 28.0),
        Station::new("Managua", 27.3),
        Station::new("Manama", 26.5),
        Station::new("Mandalay", 28.0),
        Station::new("Mango", 28.1),
        Station::new("Manila", 28.4),
        Station::new("Maputo", 22.8),
        Station::new("Marrakesh", 19.6),
        Station::new("Marseille", 15.8),
        Station::new("Maun", 22.4),
        Station::new("Medan", 26.5),
        Station::new("Mek'ele", 22.7),
        Station::new("Melbourne", 15.1),
        Station::new("Memphis", 17.2),
        Station::new("Mexicali", 23.1),
        Station::new("Mexico City", 17.5),
        Station::new("Miami", 24.9),
        Station::new("Milan", 13.0),
        Station::new("Milwaukee", 8.9),
        Station::new("Minneapolis", 7.8),
        Station::new("Minsk", 6.7),
        Station::new("Mogadishu", 27.1),
        Station::new("Mombasa", 26.3),
        Station::new("Monaco", 16.4),
        Station::new("Moncton", 6.1),
        Station::new("Monterrey", 22.3),
        Station::new("Montreal", 6.8),
        Station::new("Moscow", 5.8),
        Station::new("Mumbai", 27.1),
        Station::new("Murmansk", 0.6),
        Station::new("Muscat", 28.0),
        Station::new("Mzuzu", 17.7),
        Station::new("N'Djamena", 28.3),
        Station::new("Naha", 23.1),
        Station::new("Nairobi", 17.8),
        Station::new("Nakhon Ratchasima", 27.3),
        Station::new("Napier", 14.6),
        Station::new("Napoli", 15.9),
        Station::new("Nashville", 15.4),
        Station::new("Nassau", 24.6),
        Station::new("Ndola", 20.3),
        Station::new("New Delhi", 25.0),
        Station::new("New Orleans", 20.7),
        Station::new("New York City", 12.9),
        Station::new("Ngaoundéré", 22.0),
        Station::new("Niamey", 29.3),
        Station::new("Nicosia", 19.7),
        Station::new("Niigata", 13.9),
        Station::new("Nouadhibou", 21.3),
        Station::new("Nouakchott", 25.7),
        Station::new("Novosibirsk", 1.7),
        Station::new("Nuuk", -1.4),
        Station::new("Odesa", 10.7),
        Station::new("Odienné", 26.0),
        Station::new("Oklahoma City", 15.9),
        Station::new("Omaha", 10.6),
        Station::new("Oranjestad", 28.1),
        Station::new("Oslo", 5.7),
        Station::new("Ottawa", 6.6),
        Station::new("Ouagadougou", 28.3),
        Station::new("Ouahigouya", 28.6),
        Station::new("Ouarzazate", 18.9),
        Station::new("Oulu", 2.7),
        Station::new("Palembang", 27.3),
        Station::new("Palermo", 18.5),
        Station::new("Palm Springs", 24.5),
        Station::new("Palmerston North", 13.2),
        Station::new("Panama City", 28.0),
        Station::new("Parakou", 26.8),
        Station::new("Paris", 12.3),
        Station::new("Perth", 18.7),
        Station::new("Petropavlovsk-Kamchatsky", 1.9),
        Station::new("Philadelphia", 13.2),
        Station::new("Phnom Penh", 28.3),
        Station::new("Phoenix", 23.9),
        Station::new("Pittsburgh", 10.8),
        Station::new("Podgorica", 15.3),
        Station::new("Pointe-Noire", 26.1),
        Station::new("Pontianak", 27.7),
        Station::new("Port Moresby", 26.9),
        Station::new("Port Sudan", 28.4),
        Station::new("Port Vila", 24.3),
        Station::new("Port-Gentil", 26.0),
        Station::new("Portland (OR)", 12.4),
        Station::new("Porto", 15.7),
        Station::new("Prague", 8.4),
        Station::new("Praia", 24.4),
        Station::new("Pretoria", 18.2),
        Station::new("Pyongyang", 10.8),
        Station::new("Rabat", 17.2),
        Station::new("Rangpur", 24.4),
        Station::new("Reggane", 28.3),
        Station::new("Reykjavík", 4.3),
        Station::new("Riga", 6.2),
        Station::new("Riyadh", 26.0),
        Station::new("Rome", 15.2),
        Station::new("Roseau", 26.2),
        Station::new("Rostov-on-Don", 9.9),
        Station::new("Sacramento", 16.3),
        Station::new("Saint Petersburg", 5.8),
        Station::new("Saint-Pierre", 5.7),
        Station::new("Salt Lake City", 11.6),
        Station::new("San Antonio", 20.8),
        Station::new("San Diego", 17.8),
        Station::new("San Francisco", 14.6),
        Station::new("San Jose", 16.4),
        Station::new("San José", 22.6),
        Station::new("San Juan", 27.2),
        Station::new("San Salvador", 23.1),
        Station::new("Sana'a", 20.0),
        Station::new("Santo Domingo", 25.9),
        Station::new("Sapporo", 8.9),
        Station::new("Sarajevo", 10.1),
        Station::new("Saskatoon", 3.3),
        Station::new("Seattle", 11.3),
        Station::new("Ségou", 28.0),
        Station::new("Seoul", 12.5),
        Station::new("Seville", 19.2),
        Station::new("Shanghai", 16.7),
        Station::new("Singapore", 27.0),
        Station::new("Skopje", 12.4),
        Station::new("Sochi", 14.2),
        Station::new("Sofia", 10.6),
        Station::new("Sokoto", 28.0),
        Station::new("Split", 16.1),
        Station::new("St. John's", 5.0),
        Station::new("St. Louis", 13.9),
        Station::new("Stockholm", 6.6),
        Station::new("Surabaya", 27.1),
        Station::new("Suva", 25.6),
        Station::new("Suwałki", 7.2),
        Station::new("Sydney", 17.7),
        Station::new("Tabora", 23.0),
        Station::new("Tabriz", 12.6),
        Station::new("Taipei", 23.0),
        Station::new("Tallinn", 6.4),
        Station::new("Tamale", 27.9),
        Station::new("Tamanrasset", 21.7),
        Station::new("Tampa", 22.9),
        Station::new("Tashkent", 14.8),
        Station::new("Tauranga", 14.8),
        Station::new("Tbilisi", 12.9),
        Station::new("Tegucigalpa", 21.7),
        Station::new("Tehran", 17.0),
        Station::new("Tel Aviv", 20.0),
        Station::new("Thessaloniki", 16.0),
        Station::new("Thiès", 24.0),
        Station::new("Tijuana", 17.8),
        Station::new("Timbuktu", 28.0),
        Station::new("Tirana", 15.2),
        Station::new("Toamasina", 23.4),
        Station::new("Tokyo", 15.4),
        Station::new("Toliara", 24.1),
        Station::new("Toluca", 12.4),
        Station::new("Toronto", 9.4),
        Station::new("Tripoli", 20.0),
        Station::new("Tromsø", 2.9),
        Station::new("Tucson", 20.9),
        Station::new("Tunis", 18.4),
        Station::new("Ulaanbaatar", -0.4),
        Station::new("Upington", 20.4),
        Station::new("Ürümqi", 7.4),
        Station::new("Vaduz", 10.1),
        Station::new("Valencia", 18.3),
        Station::new("Valletta", 18.8),
        Station::new("Vancouver", 10.4),
        Station::new("Veracruz", 25.4),
        Station::new("Vienna", 10.4),
        Station::new("Vientiane", 25.9),
        Station::new("Villahermosa", 27.1),
        Station::new("Vilnius", 6.0),
        Station::new("Virginia Beach", 15.8),
        Station::new("Vladivostok", 4.9),
        Station::new("Warsaw", 8.5),
        Station::new("Washington, D.C.", 14.6),
        Station::new("Wau", 27.8),
        Station::new("Wellington", 12.9),
        Station::new("Whitehorse", -0.1),
        Station::new("Wichita", 13.9),
        Station::new("Willemstad", 28.0),
        Station::new("Winnipeg", 3.0),
        Station::new("Wrocław", 9.6),
        Station::new("Xi'an", 14.1),
        Station::new("Yakutsk", -8.8),
        Station::new("Yangon", 27.5),
        Station::new("Yaoundé", 23.8),
        Station::new("Yellowknife", -4.3),
        Station::new("Yerevan", 12.4),
        Station::new("Yinchuan", 9.0),
        Station::new("Zagreb", 10.7),
        Station::new("Zanzibar City", 26.0),
        Station::new("Zürich", 9.3),
    ]
    .into_iter()
    .collect()
});
