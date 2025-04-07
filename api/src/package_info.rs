use chrono::{DateTime, Local};
use crate::utilities::{DeliveryStatus, TrafficCondition, WeatherCondition};

/// Represents essential details about a package's
/// delivery in order to predict delivery times and
/// monitor the package's status
pub struct PackageInfo {
    /// The unique identifier for the package
    pub tracking_id: u64,

    /// The starting point of the package
    pub origin: String,

    /// The destination of the package (where it will arrive)
    pub destination: String,

    /// The current delivery status of the package
    pub delivery_status: DeliveryStatus,

    /// The carrier/shipping provider for the shipment (FedEx, DHL, UPS, etc.)
    pub carrier: String,

    /// The most recent timestamp of the package for when the status was updated
    pub timestamp: DateTime<Local>,

    /// The current location of the package
    pub location: String,

    /// The current traffic condition at the location of this package
    pub traffic_level: TrafficCondition,

    /// The current weather condition at the location of this package
    pub weather_condition: WeatherCondition,

    /// The weight of the package (in kilograms - kg)
    pub weight: f64,

    /// The delay in minutes for the package, if any
    pub traffic_delay: Option<u64>,
}