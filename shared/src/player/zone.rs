use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use geo_clipper::Clipper;
use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use serde::{Deserialize, Serialize};

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
    pub exterior: Vec<Vec2>,
    pub interiors: Vec<Vec<Vec2>>,
}

#[derive(Reflect, Component, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Zones {
    pub zones: Vec<Zone>,
}

impl From<&Zones> for Path {
    fn from(zones: &Zones) -> Self {
        let mut path = PathBuilder::new();
        for zone in zones.zones.iter() {
            if zone.exterior.len() < 3 {
                continue;
            }
            path.move_to(zone.exterior[0]);
            for point in zone.exterior.iter().skip(1) {
                path.line_to(*point);
            }
            path.close();

            for interior in &zone.interiors {
                if interior.len() < 3 {
                    continue;
                }
                path.move_to(interior[0]);
                for point in interior.iter().skip(1) {
                    path.line_to(*point);
                }
                path.close();
            }
        }
        path.build()
    }
}

impl From<&Zone> for Path {
    fn from(zone: &Zone) -> Self {
        let mut path = PathBuilder::new();
        if zone.exterior.len() < 3 {
            return path.build();
        }
        path.move_to(zone.exterior[0]);
        for point in zone.exterior.iter().skip(1) {
            path.line_to(*point);
        }
        path.close();

        for interior in &zone.interiors {
            if interior.len() < 3 {
                continue;
            }
            path.move_to(interior[0]);
            for point in interior.iter().skip(1) {
                path.line_to(*point);
            }
            path.close();
        }
        path.build()
    }
}

impl Zone {
    pub fn new(exterior: Vec<Vec2>) -> Self {
        Zone {
            exterior,
            interiors: Vec::new(),
        }
    }

    fn to_geo_polygon(&self) -> Polygon {
        let exterior: Vec<Coord> = self
            .exterior
            .iter()
            .map(|p| Coord {
                x: p.x as f64,
                y: p.y as f64,
            })
            .collect();

        let interiors: Vec<LineString> = self
            .interiors
            .iter()
            .map(|ring| {
                LineString(
                    ring.iter()
                        .map(|p| Coord {
                            x: p.x as f64,
                            y: p.y as f64,
                        })
                        .collect(),
                )
            })
            .collect();

        Polygon::new(LineString(exterior), interiors)
    }

    fn from_geo_polygon(poly: Polygon) -> Self {
        let exterior: Vec<Vec2> = poly
            .exterior()
            .0
            .iter()
            .map(|p| Vec2::new(p.x as f32, p.y as f32))
            .collect();

        let interiors: Vec<Vec<Vec2>> = poly
            .interiors()
            .iter()
            .map(|ring| {
                ring.0
                    .iter()
                    .map(|p| Vec2::new(p.x as f32, p.y as f32))
                    .collect()
            })
            .collect();

        Zone {
            exterior,
            interiors,
        }
    }
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

    pub fn cut_out_zones(&mut self, cut_zone: &Zone) {
        let stencil = cut_zone.to_geo_polygon();
        let mut new_zones = vec![];
        for zone in self.zones.iter() {
            let to_be_cut = zone.to_geo_polygon();

            match to_be_cut.difference(&stencil, CLIPPER_SCALE) {
                MultiPolygon(polys) => {
                    for poly in polys {
                        if !poly.exterior().0.is_empty() {
                            new_zones.push(Zone::from_geo_polygon(poly));
                        }
                    }
                }
            }
        }
        trace!(?new_zones);
        self.zones = new_zones;
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
