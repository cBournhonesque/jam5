use bevy::prelude::*;
use geo_clipper::Clipper;
use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use lightyear::connection::netcode::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CLIPPER_SCALE: f64 = 1_000_000.0;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Zone {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl Zone {
    pub fn new(points: Vec<Vec2>, color: Color) -> Self {
        Zone { points, color }
    }

    fn to_geo_polygon(&self) -> Polygon {
        let exterior: Vec<Coord> = self
            .points
            .iter()
            .map(|p| Coord {
                x: p.x as f64,
                y: p.y as f64,
            })
            .collect();
        Polygon::new(LineString(exterior), vec![])
    }

    fn from_geo_polygon(poly: Polygon, color: Color) -> Self {
        let points: Vec<Vec2> = poly
            .exterior()
            .0
            .iter()
            .map(|p| Vec2::new(p.x as f32, p.y as f32))
            .collect();
        Zone::new(points, color)
    }
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ZoneManager {
    zones: HashMap<ClientId, Vec<Zone>>,
}

impl ZoneManager {
    pub fn new() -> Self {
        ZoneManager {
            zones: HashMap::new(),
        }
    }

    pub fn add_zone(&mut self, client_id: ClientId, new_zone: Zone) {
        let zones = self.zones.entry(client_id).or_insert_with(Vec::new);
        let mut merged_zone = new_zone;
        zones.retain(|zone| {
            if Self::zones_overlap(zone, &merged_zone) {
                merged_zone = Self::union_zones(merged_zone.clone(), zone.clone());
                false
            } else {
                true
            }
        });
        zones.push(merged_zone);
    }

    fn union_zones(zone1: Zone, zone2: Zone) -> Zone {
        let poly1 = zone1.to_geo_polygon();
        let poly2 = zone2.to_geo_polygon();

        match poly1.union(&poly2, CLIPPER_SCALE) {
            MultiPolygon(mut polys) if !polys.is_empty() => {
                polys.sort_by_key(|p| std::cmp::Reverse(p.exterior().0.len()));
                Zone::from_geo_polygon(polys.remove(0), zone1.color)
            }
            _ => zone1,
        }
    }

    fn zones_overlap(zone1: &Zone, zone2: &Zone) -> bool {
        let poly1 = zone1.to_geo_polygon();
        let poly2 = zone2.to_geo_polygon();

        match poly1.intersection(&poly2, CLIPPER_SCALE) {
            MultiPolygon(polys) => !polys.is_empty(),
        }
    }

    pub fn get_all_zones(&self) -> Vec<&Zone> {
        self.zones.values().flat_map(|v| v.iter()).collect()
    }

    pub fn get_zones_for_client(&self, client_id: &ClientId) -> Option<&Vec<Zone>> {
        self.zones.get(client_id)
    }
}
