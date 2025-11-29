use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct LevelRange {
    min: Option<u32>,
    max: Option<u32>,
}

impl LevelRange {
    pub fn new(min: Option<u32>, max: Option<u32>) -> Result<LevelRange, String> {
        if let Some(min) = min {
            if min > 200 {
                return Err("min must be between 0 and 200".into())
            }
        }

        if let Some(max) = max {
            if max > 200 {
                return Err("max must be between 0 and 200".into())
            }
        }

        if let (Some(min), Some(max)) = (min, max) {
            if min > max {
                return Err("min cannot be greater than max".into());
            }
        }

        Ok(LevelRange { min : min, max : max })
    }

    pub fn min(&self) -> Option<u32> {
        self.min
    }

    pub fn max(&self) -> Option<u32> {
        self.max
    }

    pub fn is_valid(self, level: u32) -> bool {
        self.min.map_or(true, |m| level >= m) &&
        self.max.map_or(true, |m| level <= m)
    }
}

impl FromStr for LevelRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Supported formats:
        //   "100..180"
        //   "100.."
        //   "..180"
        //   "150"
        if let Some((min, max)) = s.split_once("..") {
            let min = if min.is_empty() {
                None
            } else {
                Some(min.parse::<u32>().map_err(|_| "Invalid min".to_string())?)
            };

            let max = if max.is_empty() {
                None
            } else {
                Some(max.parse::<u32>().map_err(|_| "Invalid max".to_string())?)
            };

            return LevelRange::new(min, max);
        }

        // Single value like "150"
        let val = s.parse::<u32>().map_err(|_| "Invalid level".to_string())?;
        LevelRange::new(Some(val), Some(val))
    }
}