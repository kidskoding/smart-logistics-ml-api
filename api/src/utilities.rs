extern crate reqwest;
pub enum TrafficCondition {
    Low,
    Moderate,
    High,
}

pub enum WeatherCondition {
    Clear,
    Cloudy,
    Rainy,
    Thunderstorms,
    Snowy,
    Windy,
    Foggy,
    Hail,
    Drizzly,
    Hot,
    Cold,
    Humid,
}

pub enum DeliveryStatus {
    InTransit,
    Delayed,
    Delivered,
    Cancelled,
}

pub enum Carrier {
    FedEx,
    Other(String),
}
