
pub enum ActorType {
    Figure,
    Deity
}

pub enum DestinationType {
    None,
    Figure,
    Site
}

pub enum GoalType {
    Idle,
    Travelling,
    Razing
}
pub struct ActorPersonality {
    pub compassion: i32,
}

impl Default for ActorPersonality {
    fn default() -> Self {
        ActorPersonality {
            compassion: 100,
        }
    }
}

pub struct Actor {
    pub actor_type: ActorType,
    pub id: i32,
    pub current_site: i32,
    pub current_goal: GoalType,
    pub think_timer: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub destination: i32,
    pub destination_type: DestinationType,
    pub personality: ActorPersonality,
    pub first_name: String,
    pub text_blurb: String,
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            actor_type: ActorType::Figure,
            id: 0,
            current_site: -1,
            current_goal: GoalType::Idle,
            think_timer: 0,
            pos_x: 0,
            pos_y: 0,
            destination: -1,
            destination_type: DestinationType::None,
            personality: ActorPersonality::default(),
            first_name: "Kyle".to_string(),
            text_blurb: "I don't exist and i'm not okay with this!".to_string(),
        }
    }
}

impl Actor {
    pub fn update(&mut self) {
        match self.current_goal  {
            GoalType::Idle => {
                if self.think_timer < 0 {
                    self.think_timer = 100;
                    println!("{} is thinking..", self.id );
                }else{
                    self.think_timer -= 1;
                }
            },
            GoalType::Razing => {},
            GoalType::Travelling => {}
        }
    }
}