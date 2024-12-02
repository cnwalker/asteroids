
#[derive(Debug, serde::Deserialize)]
pub struct Asteroid {
    spkid: String,
    pub full_name: String,
    name: String,
    diameter: f32,
    albedo: f32,
    #[serde(rename = "e")]
    eccentricity: f32,
    #[serde(rename = "a")]
    semi_major_axis: f32,
    #[serde(rename = "i")]
    inclination: f32,
    #[serde(rename = "per")]
    period: String, // Some results in scientific notation
    #[serde(rename = "per_y")]
    period_years: String,
    #[serde(rename = "moid")]
    earth_minimum_orbit_distance: f32,
    #[serde(rename = "moid_ld")]
    earth_minimum_orbit_insertion_distance: f32,
    #[serde(rename = "sigma_e")]
    sigma_eccentricity: String,
    #[serde(rename = "sigma_a")]
    sigma_semi_major_axis: String,
    #[serde(rename = "sigma_q")]
    sigma_perihelion_distance: String,
}