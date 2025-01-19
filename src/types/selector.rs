
#[derive(Debug, Clone, PartialEq)]
pub enum TargetSelector {
    All(Option<TargetFilter>),         // @a
    AllEntities(Option<TargetFilter>), // @e
    Current(Option<TargetFilter>),     // @s
    Random(Option<TargetFilter>),      // @r
    Nearest(Option<TargetFilter>),     // @p
    Other(String),
}

impl ToString for TargetSelector {
    fn to_string(&self) -> String {
        match self {
            TargetSelector::All(filter) => format!(
                "@a[{}]",
                filter.as_ref().map(|f| f.to_string()).unwrap_or_default()
            ),
            TargetSelector::AllEntities(filter) => format!(
                "@e[{}]",
                filter.as_ref().map(|f| f.to_string()).unwrap_or_default()
            ),
            TargetSelector::Current(filter) => format!(
                "@s[{}]",
                filter.as_ref().map(|f| f.to_string()).unwrap_or_default()
            ),
            TargetSelector::Nearest(filter) => format!(
                "@p[{}]",
                filter.as_ref().map(|f| f.to_string()).unwrap_or_default()
            ),
            TargetSelector::Random(filter) => format!(
                "@r[{}]",
                filter.as_ref().map(|f| f.to_string()).unwrap_or_default()
            ),
            TargetSelector::Other(other) => format!("{other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TargetFilter {
    pub advancements: Option<Vec<crate::advancements::Advancements>>,
    pub distance: Option<super::world::Distance>,
    pub dx: Option<f32>,
    pub dy: Option<f32>,
    pub dz: Option<f32>,
    pub gamemode: Option<super::world::Gamemode>,
    pub level: Option<u32>,
    pub limit: Option<u32>,
    pub name: Option<String>,
    pub nbt: Option<String>,
    pub predicate: Option<String>,
    pub scores: Option<Vec<(String, i32, i32)>>, // Scoreboard criteria and range
    pub sort: Option<String>,                    // Sort by: nearest, furthest, random, etc.
    pub tag: Option<String>,                     // Entity tag
    pub team: Option<String>,                    // Team name
    pub x: Option<f32>,
    pub x_rotation: Option<super::world::Distance>,
    pub y: Option<f32>,
    pub y_rotation: Option<super::world::Distance>,
    pub z: Option<f32>,
    pub entity: Option<String>
}

impl Default for TargetFilter {
    fn default() -> Self {
        TargetFilter {
            advancements: None,
            distance: None,
            dx: None,
            dy: None,
            dz: None,
            gamemode: None,
            level: None,
            limit: None,
            name: None,
            nbt: None,
            predicate: None,
            scores: None,
            sort: None,
            tag: None,
            team: None,
            x: None,
            x_rotation: None,
            y: None,
            y_rotation: None,
            z: None,
            entity: None
        }
    }
}

impl ToString for TargetFilter {
    fn to_string(&self) -> String {
        let mut filters = Vec::new();

        if let Some(advancements) = &self.advancements {
            let advancements = advancements
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
            filters.push(format!("advancements={{{}}}", advancements.join(",")));
        }
        if let Some(distance) = &self.distance {
            filters.push(format!("distance={}", distance.to_string()));
        }
        if let Some(dx) = &self.dx {
            filters.push(format!("dx={}", dx));
        }
        if let Some(dy) = &self.dy {
            filters.push(format!("dy={}", dy));
        }
        if let Some(dz) = &self.dz {
            filters.push(format!("dz={}", dz));
        }
        if let Some(gamemode) = &self.gamemode {
            filters.push(format!("gamemode={}", gamemode.to_string()));
        }
        if let Some(level) = &self.level {
            filters.push(format!("level={}", level));
        }
        if let Some(limit) = &self.limit {
            filters.push(format!("limit={}", limit));
        }
        if let Some(name) = &self.name {
            filters.push(format!("name={}", name));
        }
        if let Some(nbt) = &self.nbt {
            filters.push(format!("nbt={}", nbt));
        }
        if let Some(_predicate) = &self.predicate {
            unimplemented!("I'm not sure how to handle these yet.")
        }
        if let Some(_scores) = &self.scores {
            unimplemented!("I'm not sure how to handle these yet.")
        }
        if let Some(sort) = &self.sort {
            filters.push(format!("sort={}", sort));
        }
        if let Some(tag) = &self.tag {
            filters.push(format!("tag={}", tag));
        }
        if let Some(team) = &self.team {
            filters.push(format!("team={}", team));
        }
        if let Some(x) = &self.x {
            filters.push(format!("x={}", x));
        }
        if let Some(x_rotation) = &self.x_rotation {
            filters.push(format!("x_rotation={}", x_rotation.to_string()));
        }
        if let Some(y) = &self.y {
            filters.push(format!("y={}", y));
        }
        if let Some(y_rotation) = &self.y_rotation {
            filters.push(format!("y_rotation={}", y_rotation.to_string()));
        }
        if let Some(z) = &self.z {
            filters.push(format!("z={}", z));
        }
        if let Some(entity) = &self.entity {
            filters.push(format!("type={}", entity.to_string()))
        }

        filters.join(",")
    }
}
