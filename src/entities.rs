
pub trait EntityClass: ToString + Clone {
    fn is_hostile(&self) -> bool;
    fn is_passive(&self) -> bool;
    fn is_projectile(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinecraftEntity {
    // Passive Mobs
    Allay,
    Axolotl,
    Bat,
    Cat,
    Chicken,
    Cod,
    Cow,
    Dolphin,
    Donkey,
    Fox,
    Frog,
    GlowSquid,
    Horse,
    Mooshroom,
    Mule,
    Ocelot,
    Parrot,
    Pig,
    Rabbit,
    Salmon,
    Sheep,
    SkeletonHorse,
    Sniffer,
    SnowGolem,
    Squid,
    Strider,
    TropicalFish,
    Turtle,
    Villager,
    WanderingTrader,

    // Neutral Mobs
    Bee,
    CaveSpider,
    Enderman,
    Goat,
    IronGolem,
    Llama,
    Panda,
    PolarBear,
    Spider,
    TraderLlama,
    Wolf,
    ZombifiedPiglin,

    // Hostile Mobs
    Blaze,
    Creeper,
    Drowned,
    ElderGuardian,
    Endermite,
    Evoker,
    Ghast,
    Guardian,
    Hoglin,
    Husk,
    MagmaCube,
    Phantom,
    Piglin,
    PiglinBrute,
    Pillager,
    Ravager,
    Shulker,
    Silverfish,
    Skeleton,
    Slime,
    Stray,
    Vex,
    Vindicator,
    Warden,
    Witch,
    WitherSkeleton,
    Zoglin,
    Zombie,
    ZombieVillager,

    // Bosses
    EnderDragon,
    WitherBoss,

    // Projectiles
    Arrow,
    DragonFireball,
    Egg,
    EnderPearl,
    ExperienceBottle,
    FireworkRocket,
    FishingBobber,
    LargeFireball,
    LlamaSpit,
    ShulkerBullet,
    SmallFireball,
    Snowball,
    SpectralArrow,
    ThrownExperienceBottle,
    ThrownPotion,
    ThrownTrident,
    WitherSkull,

    // Vehicles
    Boat,
    ChestBoat,
    Minecart,
    ChestMinecart,
    CommandBlockMinecart,
    FurnaceMinecart,
    HopperMinecart,
    SpawnerMinecart,
    TNTMinecart,

    // Other
    ArmorStand,
    EndCrystal,
    EvokerFangs,
    ExperienceOrb,
    FallingBlock,
    Item,
    LeashKnot,
    Lightning,
    Marker,
    Painting,
    PrimedTnt,

    // Display Entities (1.19.4+)
    BlockDisplay,
    ItemDisplay,
    TextDisplay,
    Interaction,

    // Technical
    Player,
}

impl EntityClass for MinecraftEntity {
    fn is_hostile(&self) -> bool {
        matches!(
            self,
            MinecraftEntity::Blaze
                | MinecraftEntity::Creeper
                | MinecraftEntity::Drowned
                | MinecraftEntity::ElderGuardian
                | MinecraftEntity::Endermite
                | MinecraftEntity::Evoker
                | MinecraftEntity::Ghast
                | MinecraftEntity::Guardian
                | MinecraftEntity::Hoglin
                | MinecraftEntity::Husk
                | MinecraftEntity::MagmaCube
                | MinecraftEntity::Phantom
                | MinecraftEntity::Piglin
                | MinecraftEntity::PiglinBrute
                | MinecraftEntity::Pillager
                | MinecraftEntity::Ravager
                | MinecraftEntity::Shulker
                | MinecraftEntity::Silverfish
                | MinecraftEntity::Skeleton
                | MinecraftEntity::Slime
                | MinecraftEntity::Stray
                | MinecraftEntity::Vex
                | MinecraftEntity::Vindicator
                | MinecraftEntity::Warden
                | MinecraftEntity::Witch
                | MinecraftEntity::WitherSkeleton
                | MinecraftEntity::Zoglin
                | MinecraftEntity::Zombie
                | MinecraftEntity::ZombieVillager
        )
    }

    /// Returns whether this entity is considered passive
    fn is_passive(&self) -> bool {
        matches!(
            self,
            MinecraftEntity::Allay
                | MinecraftEntity::Axolotl
                | MinecraftEntity::Bat
                | MinecraftEntity::Cat
                | MinecraftEntity::Chicken
                | MinecraftEntity::Cod
                | MinecraftEntity::Cow
                | MinecraftEntity::Donkey
                | MinecraftEntity::Fox
                | MinecraftEntity::Frog
                | MinecraftEntity::GlowSquid
                | MinecraftEntity::Horse
                | MinecraftEntity::Mooshroom
                | MinecraftEntity::Mule
                | MinecraftEntity::Ocelot
                | MinecraftEntity::Parrot
                | MinecraftEntity::Pig
                | MinecraftEntity::Rabbit
                | MinecraftEntity::Salmon
                | MinecraftEntity::Sheep
                | MinecraftEntity::SkeletonHorse
                | MinecraftEntity::Sniffer
                | MinecraftEntity::SnowGolem
                | MinecraftEntity::Squid
                | MinecraftEntity::Strider
                | MinecraftEntity::TropicalFish
                | MinecraftEntity::Turtle
                | MinecraftEntity::Villager
                | MinecraftEntity::WanderingTrader
        )
    }

    /// Returns whether this entity is a projectile
    fn is_projectile(&self) -> bool {
        matches!(
            self,
            MinecraftEntity::Arrow
                | MinecraftEntity::DragonFireball
                | MinecraftEntity::Egg
                | MinecraftEntity::EnderPearl
                | MinecraftEntity::ExperienceBottle
                | MinecraftEntity::FireworkRocket
                | MinecraftEntity::FishingBobber
                | MinecraftEntity::LargeFireball
                | MinecraftEntity::LlamaSpit
                | MinecraftEntity::ShulkerBullet
                | MinecraftEntity::SmallFireball
                | MinecraftEntity::Snowball
                | MinecraftEntity::SpectralArrow
                | MinecraftEntity::ThrownExperienceBottle
                | MinecraftEntity::ThrownPotion
                | MinecraftEntity::ThrownTrident
                | MinecraftEntity::WitherSkull
        )
    }
}

impl ToString for MinecraftEntity {
    fn to_string(&self) -> String {
        match self {
            // Passive Mobs
            MinecraftEntity::Allay => "minecraft:allay",
            MinecraftEntity::Axolotl => "minecraft:axolotl",
            MinecraftEntity::Bat => "minecraft:bat",
            MinecraftEntity::Cat => "minecraft:cat",
            MinecraftEntity::Chicken => "minecraft:chicken",
            MinecraftEntity::Cod => "minecraft:cod",
            MinecraftEntity::Cow => "minecraft:cow",
            MinecraftEntity::Dolphin => "minecraft:dolphin",
            MinecraftEntity::Fox => "minecraft:fox",
            MinecraftEntity::Donkey => "minecraft:donkey",
            MinecraftEntity::Frog => "minecraft:frog",
            MinecraftEntity::GlowSquid => "minecraft:glow_squid",
            MinecraftEntity::Horse => "minecraft:horse",
            MinecraftEntity::Mooshroom => "minecraft:mooshroom",
            MinecraftEntity::Mule => "minecraft:mule",
            MinecraftEntity::Ocelot => "minecraft:ocelot",
            MinecraftEntity::Parrot => "minecraft:parrot",
            MinecraftEntity::Pig => "minecraft:pig",
            MinecraftEntity::Rabbit => "minecraft:rabbit",
            MinecraftEntity::Salmon => "minecraft:salmon",
            MinecraftEntity::Sheep => "minecraft:sheep",
            MinecraftEntity::SkeletonHorse => "minecraft:skeleton_horse",
            MinecraftEntity::Sniffer => "minecraft:sniffer",
            MinecraftEntity::SnowGolem => "minecraft:snow_golem",
            MinecraftEntity::Squid => "minecraft:squid",
            MinecraftEntity::Strider => "minecraft:strider",
            MinecraftEntity::TropicalFish => "minecraft:tropical_fish",
            MinecraftEntity::Turtle => "minecraft:turtle",
            MinecraftEntity::Villager => "minecraft:villager",
            MinecraftEntity::WanderingTrader => "minecraft:wandering_trader",

            // Neutral Mobs
            MinecraftEntity::Bee => "minecraft:bee",
            MinecraftEntity::CaveSpider => "minecraft:cave_spider",
            MinecraftEntity::Enderman => "minecraft:enderman",
            MinecraftEntity::Goat => "minecraft:goat",
            MinecraftEntity::IronGolem => "minecraft:iron_golem",
            MinecraftEntity::Llama => "minecraft:llama",
            MinecraftEntity::Panda => "minecraft:panda",
            MinecraftEntity::PolarBear => "minecraft:polar_bear",
            MinecraftEntity::Spider => "minecraft:spider",
            MinecraftEntity::TraderLlama => "trader_llama",
            MinecraftEntity::Wolf => "minecraft:wolf",
            MinecraftEntity::ZombifiedPiglin => "minecraft:zombified_piglin",

            // Hostile Mobs
            MinecraftEntity::Blaze => "minecraft:blaze",
            MinecraftEntity::Creeper => "minecraft:creeper",
            MinecraftEntity::Drowned => "minecraft:drowned",
            MinecraftEntity::ElderGuardian => "minecraft:elder_guardian",
            MinecraftEntity::Endermite => "minecraft:endermite",
            MinecraftEntity::Evoker => "minecraft:evoker",
            MinecraftEntity::Guardian => "minecraft:guardian",
            MinecraftEntity::Ghast => "minecraft:ghast",
            MinecraftEntity::Hoglin => "minecraft:hoglin",
            MinecraftEntity::Husk => "minecraft:husk",
            MinecraftEntity::MagmaCube => "minecraft:magma_cube",
            MinecraftEntity::Phantom => "minecraft:phantom",
            MinecraftEntity::Piglin => "minecraft:piglin",
            MinecraftEntity::PiglinBrute => "minecraft:piglin_brute",
            MinecraftEntity::Pillager => "minecraft:pillager",
            MinecraftEntity::Ravager => "minecraft:ravager",
            MinecraftEntity::Shulker => "minecraft:shulker",
            MinecraftEntity::Silverfish => "minecraft:silverfish",
            MinecraftEntity::Skeleton => "minecraft:skeleton",
            MinecraftEntity::Slime => "minecraft:slime",
            MinecraftEntity::Stray => "minecraft:stray",
            MinecraftEntity::Vex => "minecraft:vex",
            MinecraftEntity::Vindicator => "minecraft:vindicator",
            MinecraftEntity::Warden => "minecraft:warden",
            MinecraftEntity::Witch => "minecraft:witch",
            MinecraftEntity::WitherSkeleton => "minecraft:wither_skeleton",
            MinecraftEntity::Zoglin => "minecraft:zoglin",
            MinecraftEntity::Zombie => "minecraft:zombie",
            MinecraftEntity::ZombieVillager => "minecraft:zombie_villager",

            // Bosses
            MinecraftEntity::EnderDragon => "minecraft:ender_dragon",
            MinecraftEntity::WitherBoss => "minecraft:wither",

            // Projectiles
            MinecraftEntity::Arrow => "minecraft:arrow",
            MinecraftEntity::DragonFireball => "minecraft:dragon_fireball",
            MinecraftEntity::Egg => "minecraft:egg",
            MinecraftEntity::EnderPearl => "minecraft:ender_pearl",
            MinecraftEntity::ExperienceBottle => "minecraft:experience_bottle",
            MinecraftEntity::FireworkRocket => "minecraft:firework_rocket",
            MinecraftEntity::FishingBobber => "minecraft:fishing_bobber",
            MinecraftEntity::LargeFireball => "minecraft:fireball",
            MinecraftEntity::LlamaSpit => "minecraft:llama_spit",
            MinecraftEntity::ShulkerBullet => "minecraft:shulker_bullet",
            MinecraftEntity::SmallFireball => "minecraft:small_fireball",
            MinecraftEntity::Snowball => "minecraft:snowball",
            MinecraftEntity::SpectralArrow => "minecraft:spectral_arrow",
            MinecraftEntity::ThrownExperienceBottle => "minecraft:experience_bottle",
            MinecraftEntity::ThrownPotion => "minecraft:potion",
            MinecraftEntity::ThrownTrident => "minecraft:trident",
            MinecraftEntity::WitherSkull => "minecraft:wither_skull",

            // Vehicles
            MinecraftEntity::Boat => "minecraft:boat",
            MinecraftEntity::ChestBoat => "minecraft:chest_boat",
            MinecraftEntity::Minecart => "minecraft:minecart",
            MinecraftEntity::ChestMinecart => "minecraft:chest_minecart",
            MinecraftEntity::CommandBlockMinecart => "minecraft:command_block_minecart",
            MinecraftEntity::FurnaceMinecart => "minecraft:furnace_minecart",
            MinecraftEntity::HopperMinecart => "minecraft:hopper_minecart",
            MinecraftEntity::SpawnerMinecart => "minecraft:spawner_minecart",
            MinecraftEntity::TNTMinecart => "minecraft:tnt_minecart",

            // Other
            MinecraftEntity::ArmorStand => "minecraft:armor_stand",
            MinecraftEntity::EndCrystal => "minecraft:end_crystal",
            MinecraftEntity::EvokerFangs => "minecraft:evoker_fangs",
            MinecraftEntity::ExperienceOrb => "minecraft:experience_orb",
            MinecraftEntity::FallingBlock => "minecraft:falling_block",
            MinecraftEntity::Item => "minecraft:item",
            MinecraftEntity::LeashKnot => "minecraft:leash_knot",
            MinecraftEntity::Lightning => "minecraft:lightning_bolt",
            MinecraftEntity::Marker => "minecraft:marker",
            MinecraftEntity::Painting => "minecraft:painting",
            MinecraftEntity::PrimedTnt => "minecraft:tnt",

            // Display Entities
            MinecraftEntity::BlockDisplay => "minecraft:block_display",
            MinecraftEntity::ItemDisplay => "minecraft:item_display",
            MinecraftEntity::TextDisplay => "minecraft:text_display",
            MinecraftEntity::Interaction => "minecraft:interaction",

            // Technical
            MinecraftEntity::Player => "minecraft:player",
        }
        .to_string()
    }
}

