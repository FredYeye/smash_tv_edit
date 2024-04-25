use self::enemy_type::EnemyType;

pub mod enemy_type;

pub struct Wave {
    pub enemy: EnemyType,
    pub count: u16,
    pub spawn_limit: u8,
    pub unknown: u8, //modifier?
    pub cooldown_timer: u16,
    pub pre_spawned: u8,
    pub spawn_timer: u16,
}

pub struct LevelData {
    pub circuit: u8,
    pub arena: u8,
    pub name: String,
    pub waves: Vec<Wave>,
    pub waves_remaining: u8, // todo: rename
    pub connections: [u8; 3],
}

#[derive(Debug, Default)]
pub struct Rom {
    pub rom: Vec<u8>,
}

impl Rom {
    pub fn get_level_data(&self) -> Vec<LevelData> {

        let level_offset = if self.rom[Self::from_snes_address(0x00C1CF)] == 0x02 { // load from default location
            0x02B5F0
        } else { // load from bank 0x10
            0x108000
        };

        let mut levels = Vec::new();

        let circuit_arena_counts = [11, 18, 23];
        for (circuit, &arena_max) in circuit_arena_counts.iter().enumerate() {
            for arena in 1 ..= arena_max {
                let base = self.arena_offset(circuit as u8, arena, level_offset);

                let wave_count = self.rom[base] as usize + 1;

                let mut waves = Vec::new();
    
                for w in 0 .. wave_count {
                    let enemy_type = match EnemyType::from_u8(self.rom[base + 1 + w * 10]) {
                        Some(enemy) => enemy,
                        None => {
                            println!("unknown enemy: {:02X} at {:X}", self.rom[base + 1 + w * 10], base);
                            println!("{}, {}", circuit, arena);
                            todo!()
                        }
                    };
        
                    waves.push(Wave {
                        enemy: enemy_type,
                        count: u16::from_le_bytes([self.rom[base + 2 + w * 10], self.rom[base + 3 + w * 10]]),
                        spawn_limit: self.rom[base + 4 + w * 10],
                        unknown: self.rom[base + 5 + w * 10],
                        cooldown_timer: u16::from_le_bytes([self.rom[base + 6 + w * 10], self.rom[base + 7 + w * 10]]),
                        pre_spawned: self.rom[base + 8 + w * 10],
                        spawn_timer: u16::from_le_bytes([self.rom[base + 9 + w * 10], self.rom[base + 10 + w * 10]]),
                    });
                }
    
                let waves_remaining = self.rom[base + 1 + wave_count * 10];
    
                let circuit_connection_list_address = Self::from_snes_address(0x00AA66 + circuit as u32);
                let circuit_connection_list = Self::from_snes_address(u32::from_le_bytes([
                    self.rom[circuit_connection_list_address],
                    self.rom[circuit_connection_list_address + 3],
                    0,
                    0,
                ]));
        
                let connections = [
                    self.rom[circuit_connection_list + arena as usize * 3],
                    self.rom[circuit_connection_list + arena as usize * 3 + 1],
                    self.rom[circuit_connection_list + arena as usize * 3 + 2],
                ];

                let circuit_names_offset = Self::from_snes_address(0x00E977 + circuit as u32 * 2);
                let name_offset = Self::from_snes_address(u32::from_le_bytes([
                    self.rom[circuit_names_offset],
                    self.rom[circuit_names_offset+1],
                    0,
                    0,
                ]));
        
                let offset = name_offset + (arena - 1) as usize * 26;
                let s = std::str::from_utf8(&self.rom[offset .. offset + 26]).expect("invalid utf-8 sequence");

                levels.push(LevelData {
                    circuit: circuit as u8,
                    arena: arena,
                    name: s.to_string(),
                    waves: waves,
                    waves_remaining: waves_remaining,
                    connections: connections,
                });
            }
        }

        levels
    }

    pub fn save_level_data(&mut self, level_data: &[LevelData]) {
        if level_data.len() != 52 {
            println!("not supported atm");
            return;
        }

        self.modify_rom();

        let mut circuit_offsets = Vec::new();
        let mut arena_offsets = Vec::new();
        let mut arena_data = Vec::new();

        let mut current_circuit = 0xFF;

        for level in level_data {
            if current_circuit != level.circuit {
                // store offset to new circuit
                circuit_offsets.push((3 + arena_offsets.len()) as u16 * 2 | 0x8000);

                // insert a 0x0000 (tv studio) into the list at new circuits
                arena_offsets.push(0);

                current_circuit = level.circuit;
            }

            // store offset to current arena
            let offset = ((3 + 3 + level_data.len()) * 2) + arena_data.len();
            arena_offsets.push(offset as u16 | 0x8000);

            arena_data.extend_from_slice(&Self::serialize_level_data(level));
        }

        circuit_offsets.append(&mut arena_offsets);
        let (_, body, _) = unsafe { circuit_offsets.align_to::<u8>() };
        let mut circuit_arena_offsets_u8 = body.to_vec();
        circuit_arena_offsets_u8.append(&mut arena_data);

        let offset = Self::from_snes_address(0x108000);
        self.rom[offset .. offset + circuit_arena_offsets_u8.len()].clone_from_slice(&circuit_arena_offsets_u8);

        for level in level_data {
            // save connections
            let circuit_connection_list_address = Self::from_snes_address(0x00AA66 + level.circuit as u32);
            let circuit_connection_list = Self::from_snes_address(u32::from_le_bytes([
                self.rom[circuit_connection_list_address],
                self.rom[circuit_connection_list_address + 3],
                0,
                0,
            ]));

            for (idx, &con) in level.connections.iter().enumerate() {
                self.rom[circuit_connection_list + level.arena as usize * 3 + idx] = con;
            }
        }

        std::fs::write("Smash TV edit.sfc", &self.rom).expect("Couldn't save new rom");
    }

    fn modify_rom(&mut self) {
        self.rom[Self::from_snes_address(0x00FFD7)] = 0x0A;
        self.rom[Self::from_snes_address(0x00C1CF)] = 0x10;

        self.rom[Self::from_snes_address(0x00C1D8)] = 0x00;
        self.rom[Self::from_snes_address(0x00C1D9)] = 0x80;
        self.rom[Self::from_snes_address(0x00C1DD)] = 0x01;
        self.rom[Self::from_snes_address(0x00C1DE)] = 0x80;

        self.rom.resize(0x100000, 0);
    }

    fn serialize_level_data(level_data: &LevelData) -> Vec<u8> {
        let mut serial = Vec::new();
        serial.push(level_data.waves.len() as u8 - 1);

        for wave in &level_data.waves {
            serial.push(wave.enemy.to_u8());
            serial.extend_from_slice(&wave.count.to_le_bytes());
            serial.push(wave.spawn_limit);
            serial.push(wave.unknown);
            serial.extend_from_slice(&wave.cooldown_timer.to_le_bytes());
            serial.push(wave.pre_spawned);
            serial.extend_from_slice(&wave.spawn_timer.to_le_bytes());
        }

        serial.push(level_data.waves_remaining);

        serial
    }

    fn arena_offset(&self, circuit: u8, arena: u8, base_offset: u32) -> usize {
        let circuit_offset = Self::from_snes_address(base_offset + circuit as u32 * 2);

        let arena_base = Self::from_snes_address(u32::from_le_bytes([
            self.rom[circuit_offset],
            self.rom[circuit_offset + 1],
            base_offset.to_le_bytes()[2],
            0,
        ]));

        let arena_offset = arena_base + arena as usize * 2;

        let base = Self::from_snes_address(u32::from_le_bytes([
            self.rom[arena_offset],
            self.rom[arena_offset + 1],
            base_offset.to_le_bytes()[2],
            0,
        ]));

        base
    }

    fn from_snes_address(address: u32) -> usize {
        if address & 0x8000 == 0 {
            println!("warning: malformed address: 0x{:6X}", address);
        }

        let bank = (address >> 16) * 0x8000;
        let offset = (address - 0x8000) & 0xFFFF;

        (bank + offset) as usize
    }
}

pub fn circuit_arena_name() -> [&'static str; 52] {
    [
        "Arena 1",
        "Collect 10 keys!",
        "Collect powerups!",
        "Meet Mr. Shrapnel",
        "Bonus prizes!",
        "Eat my shrapnel",
        "Total carnage",
        "Crowd control",
        "Tank trouble",
        "Mutoid Man!",
        "Secret room #1!",

        "Orbs!",
        "Meet my twin",
        "Smash 'em",
        "Fire power is needed!",
        "Slaughter 'em",
        "Lazer death zone",
        "Meet Scarface!",
        "Rowdy droids",
        "Vacuum clean",
        "Secret room #2!",
        "Metal death",
        "Watch your step",
        "Film at 11",
        "Defend me",
        "Turtles nearby",
        "Chunks galore!",
        "These are fast!",
        "Buffalo herd nearby!",

        "No dice",
        "Temple alert",
        "Scorpion fever",
        "Cobra just ahead!",
        "Walls of pain",
        "Last arena?",
        "Cobra death!",
        "Turtles beware!",
        "Extra sauce action!",
        "Secret room #3!",
        "Secret rooms nearby!",
        "Enjoy my wealth",
        "No turtles allowed!",
        "Turtle chunks needed",
        "Dynamite cobra boss",
        "Use the buffalo gun",
        "Witness total carnage",
        "Secret rooms nearby!",
        "Almost enough keys",
        "You have enough keys!",
        "Eat my eyeballs!",
        "Pleasure dome!",
        "Not enough keys!",
    ]
}
