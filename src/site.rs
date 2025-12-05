
pub struct Site {
    pub name: String,
    pub id: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub destroyed: bool,
}

enum SiteType {
    Default
}

impl Default for Site {
    fn default() -> Self {
        Site {
            pos_x: 0,
            pos_y: 0,
            name: String::from("site :)"),
            destroyed: false,
            id: 0,
        }
    }
}