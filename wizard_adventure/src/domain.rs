use self::Location::*;
use self::Object::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Object {
    Whiskey,
    Bucket,
    Frog,
    Chain,
}

impl Object {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "whiskey" => Some(Whiskey),
            "bucket" => Some(Bucket),
            "frog" => Some(Frog),
            "chain" => Some(Chain),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Location {
    LivingLoom,
    Garden,
    Attic,
    Body,
}

type Nodes = HashMap<Location, String>;
type Edge = (Location, String);
type Edges = HashMap<Location, Vec<Edge>>;
type ObjectLocations = HashMap<Object, Location>;

pub struct Game {
    location: Location,
    nodes: Nodes,
    edges: Edges,
    object_locations: ObjectLocations,
}

impl Game {
    pub fn new() -> Game {
        let nodes = HashMap::from([
            (LivingLoom, "you are in the living-room.".to_string()),
            (Garden, "you are in the garden".to_string()),
            (Attic, "you are in the attic".to_string()),
        ]);
        let edges = HashMap::from([
            (
                LivingLoom,
                vec![
                    (Garden, "west door".to_string()),
                    (Attic, "upstairs ladder".to_string()),
                ],
            ),
            (Garden, vec![(LivingLoom, "east door".to_string())]),
            (Attic, vec![(LivingLoom, "downstairs ladder".to_string())]),
        ]);
        let object_locations = HashMap::from([
            (Whiskey, LivingLoom),
            (Bucket, LivingLoom),
            (Chain, Garden),
            (Frog, Garden),
        ]);

        Game {
            location: LivingLoom,
            nodes,
            edges,
            object_locations,
        }
    }

    pub fn look(&self) -> String {
        format!(
            "{} {} {}",
            self.describe_location(),
            self.describe_paths(),
            self.describe_objects(),
        )
    }

    pub fn walk(&mut self, direction: &str) -> String {
        let edge = self.edges.get(&self.location).unwrap();

        if let Some((loc, _)) = edge.iter().find(|(_, e)| e.contains(direction)) {
            self.location = *loc;

            self.look()
        } else {
            "you cannot go that way.".to_string()
        }
    }

    pub fn inventory(&self) -> String {
        let items = self
            .objects_at(&Body)
            .iter()
            .map(|o| format!(" {:?}", o))
            .collect::<String>();

        format!("items-{}", items)
    }

    pub fn pickup(&mut self, object: Object) -> String {
        let os = self.objects_at(&self.location);
        if os.contains(&object) {
            self.object_locations.insert(object, Body);
            format!("you are now carrying the {:?}", object)
        } else {
            String::from("you cannot get that.")
        }
    }

    fn describe_location(&self) -> String {
        String::from(self.nodes.get(&self.location).unwrap())
    }

    fn describe_paths(&self) -> String {
        // , location: &Location, edegs: &Edges
        self.edges
            .get(&self.location)
            .unwrap()
            .iter()
            .map(|edge| describe_path(edge))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn objects_at(&self, location: &Location) -> Vec<Object> {
        // location: &Location, object_locations: &ObjectLocations
        self.object_locations
            .iter()
            .filter(|(_, v)| *v == location)
            .map(|(k, _)| *k)
            .collect()
    }

    fn describe_objects(&self) -> String {
        // location: &Location, object_locations: &ObjectLocations
        self.objects_at(&self.location)
            .into_iter()
            .map(|obj| format!("you see a {:?} on the floor.", obj))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn describe_path(edge: &Edge) -> String {
    format!(
        "there is a {} going {} from here.",
        edge.1.split_whitespace().nth(1).unwrap(),
        edge.1.split_whitespace().next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_described() {
        let game = Game::new(
            Attic,
            setup_nodes(),
            setup_edges(),
            setup_object_locations(),
        );

        assert_eq!("you are in the attic", game.describe_location());
    }

    #[test]
    fn path_described() {
        assert_eq!(
            "there is a door going west from here.",
            describe_path(&(Garden, "west door".to_string()))
        );
    }

    #[test]
    fn paths_described() {
        let game = Game::new(
            LivingLoom,
            setup_nodes(),
            setup_edges(),
            setup_object_locations(),
        );

        assert_eq!(
            "there is a door going west from here. there is a ladder going upstairs from here.",
            game.describe_paths()
        );
    }

    #[test]
    fn find_current_locations_object() {
        let game = Game::new(
            LivingLoom,
            setup_nodes(),
            setup_edges(),
            setup_object_locations(),
        );

        assert_eq!(vec![Whiskey, Bucket], game.objects_at(&LivingLoom))
    }

    #[test]
    fn objects_described() {
        let game = Game::new(
            LivingLoom,
            setup_nodes(),
            setup_edges(),
            setup_object_locations(),
        );

        assert_eq!(
            "you see a Whiskey on the floor. you see a Bucket on the floor.",
            game.describe_objects()
        )
    }
}
