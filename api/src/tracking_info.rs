use std::fmt;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TrackingInfo {
    pub tracking_id: String,
    pub carrier: String,
    pub delivery_date: String,
    pub status: String,
    pub location: (String, String),
    pub timestamps: Vec<(String, String)>,
    pub dimensions: [u64; 3],
    pub weight: f64,
}

impl Display for TrackingInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tracking Information:")?;
        writeln!(f, "   - Tracking ID: {}", self.tracking_id)?;
        writeln!(f, "   - Carrier: {}", self.carrier)?;
        writeln!(f, "   - Delivery Date: {}", self.delivery_date)?;
        writeln!(f, "   - Status: {}", self.status)?;
        writeln!(f, "   - Location: {}, {}", self.location.0, self.location.1)?;
        
        writeln!(f, "   - Dimensions: {}in x {}in x {}in", 
            self.dimensions[0], 
            self.dimensions[1], 
            self.dimensions[2])?;
            
        writeln!(f, "   - Weight: {} lbs", self.weight)?;
        
        if !self.timestamps.is_empty() {
            writeln!(f, "   - Tracking History:")?;
            for (date, event) in &self.timestamps {
                let (start, end) = event.split_once('T').unwrap_or((date, ""));
                writeln!(f, "       - {} - {} @ {}", date, start, end)?;
            }
        }
        
        Ok(())
    }
}