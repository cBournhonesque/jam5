use bevy::prelude::*;
use geo_clipper::Clipper;
use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use lightyear::connection::netcode::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy_prototype_lyon::prelude::*;

const CLIPPER_SCALE: f64 = 1_000_000.0;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Zone>();
        app.register_type::<Zones>();

    }
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Zone {
    pub points: Vec<Vec2>,
}


#[derive(Reflect, Component, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Zones {
    pub zones: Vec<Zone>,
}

impl From<&Zones> for Path {
    fn from(zones: &Zones) -> Self {
        let mut path = PathBuilder::new();
        for zone in zones.zones.iter() {
            if zone.points.len() < 3 {
                return path.build();
            }
            path.move_to(zone.points[0]);
            for point in zone.points.iter().skip(1) {
                path.line_to(*point);
            }
        }
        path.build()
    }
}


// Convert a Zone to a Path.
impl From<&Zone> for Path {
    fn from(value: &Zone) -> Self {
        let mut path = PathBuilder::new();
        if value.points.len() < 3 {
            return path.build();
        }
        path.move_to(value.points[0]);
        for point in value.points.iter().skip(1) {
            path.line_to(*point);
        }
        // TODO: do i need to close?
        path.build()
    }
}


impl Zone {
    pub fn new(points: Vec<Vec2>) -> Self {
        Zone { points }
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

    fn from_geo_polygon(poly: Polygon) -> Self {
        let points: Vec<Vec2> = poly
            .exterior()
            .0
            .iter()
            .map(|p| Vec2::new(p.x as f32, p.y as f32))
            .collect();
        Zone::new(points)
    }
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ZoneManager {
    zones: HashMap<ClientId, Vec<Zone>>,
}

impl Zones {

    pub fn add_zone(&mut self, new_zone: Zone) {
        let mut merged_zone = new_zone;
        self.zones.retain(|zone| {
            if Self::zones_overlap(zone, &merged_zone) {
                merged_zone = Self::union_zones(merged_zone.clone(), zone.clone());
                false
            } else {
                true
            }
        });
        trace!(?merged_zone);
        self.zones.push(merged_zone);
    }

    fn union_zones(zone1: Zone, zone2: Zone) -> Zone {
        let poly1 = zone1.to_geo_polygon();
        let poly2 = zone2.to_geo_polygon();

        match poly1.union(&poly2, CLIPPER_SCALE) {
            MultiPolygon(mut polys) if !polys.is_empty() => {
                polys.sort_by_key(|p| std::cmp::Reverse(p.exterior().0.len()));
                Zone::from_geo_polygon(polys.remove(0))
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
}
