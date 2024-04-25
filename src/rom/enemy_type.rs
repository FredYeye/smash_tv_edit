#[derive(PartialEq, Clone)]
pub enum EnemyType {
    Grunt,
    WallGunner,
    Worm,
    RedDroid,
    Snakes,
    CobraGrunt,
    LaserOrb,
    Tank,
    RedCluster,
    MrShrapnel,
    WormSegment,
    PurpleRedDroid,
    Droid,
    PurpleWorm,
    Mine,
    CobraDeath,
    MutoidMan,
    Scarface,
    Presents,
    QuestionMark,
    Babe,
}

impl EnemyType {
    pub fn enemy_list() -> [Self; 21] {
        [
            Self::Grunt,
            Self::WallGunner,
            Self::Worm,
            Self::RedDroid,
            Self::Snakes,
            Self::CobraGrunt,
            Self::LaserOrb,
            Self::Tank,
            Self::RedCluster,
            Self::MrShrapnel,
            Self::WormSegment,
            Self::PurpleRedDroid,
            Self::Droid,
            Self::PurpleWorm,
            Self::Mine,
            Self::CobraDeath,
            Self::MutoidMan,
            Self::Scarface,
            Self::Presents,
            Self::QuestionMark,
            Self::Babe,
        ]
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        Some(match val {
            1 => Self::Grunt,
            2 => Self::WallGunner,
            3 => Self::Worm,
            4 => Self::RedDroid,
            5 => Self::Snakes,
            6 => Self::CobraGrunt,
            7 => Self::LaserOrb,
            8 => Self::Tank,
            9 => Self::RedCluster,
            10 => Self::MrShrapnel,
            11 => Self::WormSegment,
            12 => Self::PurpleRedDroid,
            13 => Self::Droid,
            14 => Self::PurpleWorm,
            15 => Self::Mine,
            16 => Self::CobraDeath,
            17 => Self::MutoidMan,
            18 => Self::Scarface,

            20 => Self::Presents,
            21 => Self::QuestionMark,
            22 => Self::Babe,

            _ => return None,
        })
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Grunt => 1,
            Self::WallGunner => 2,
            Self::Worm => 3,
            Self::RedDroid => 4,
            Self::Snakes => 5,
            Self::CobraGrunt => 6,
            Self::LaserOrb => 7,
            Self::Tank => 8,
            Self::RedCluster => 9,
            Self::MrShrapnel => 10,
            Self::WormSegment => 11,
            Self::PurpleRedDroid => 12,
            Self::Droid => 13,
            Self::PurpleWorm => 14,
            Self::Mine => 15,
            Self::CobraDeath => 16,
            Self::MutoidMan => 17,
            Self::Scarface => 18,

            Self::Presents => 20,
            Self::QuestionMark => 21,
            Self::Babe => 22,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Grunt => "Grunt",
            Self::WallGunner => "Wall gunner",
            Self::Worm => "Worm",
            Self::RedDroid => "Red droid",
            Self::Snakes => "Snakes",
            Self::CobraGrunt => "Cobra grunt",
            Self::LaserOrb => "Laser orb",
            Self::Tank => "Tank",
            Self::RedCluster => "Red cluster",
            Self::MrShrapnel => "Mr Shrapnel",
            Self::WormSegment => "Worm segment",
            Self::PurpleRedDroid => "Purple / red droid",
            Self::Droid => "Droid",
            Self::PurpleWorm => "Purple worm",
            Self::Mine => "Mine",
            Self::CobraDeath => "Cobra Death",
            Self::MutoidMan => "Mutoid Man",
            Self::Scarface => "Scarface",

            Self::Presents => "Presents",
            Self::QuestionMark => "Question mark",
            Self::Babe => "Babe",
        }.to_string()
    }
}
