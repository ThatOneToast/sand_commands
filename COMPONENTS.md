# Minecraft Data Components Reference

This document provides a comprehensive list of Minecraft data components that we'll implement in our Rust project. Data components are structured data used to define and store various properties for items, entities, and block entities.

## Core Components

### A-C

- [`attribute_modifiers`](https://minecraft.wiki/w/Data_component_format#attribute_modifiers) - Applies attribute modifiers onto items
- [`banner_patterns`](https://minecraft.wiki/w/Data_component_format#banner_patterns) - Applies banner patterns onto banners or shields
- [`base_color`](https://minecraft.wiki/w/Data_component_format#base_color) - Defines the base (dye) color of a banner applied to a shield
- [`bees`](https://minecraft.wiki/w/Data_component_format#bees) - Specifies the amount of bees and their time in the hive
- [`block_entity_data`](https://minecraft.wiki/w/Data_component_format#block_entity_data) - Defines the block entity NBT data applied to the block after it's placed
- [`block_state`](https://minecraft.wiki/w/Data_component_format#block_state) - Specifies block state properties applied to the block after it's placed
- [`blocks_attacks`](https://minecraft.wiki/w/Data_component_format#blocks_attacks) - Makes the item act like a shield, meaning it can block attacks when held
- [`break_sound`](https://minecraft.wiki/w/Data_component_format#break_sound) - Changes the sound that plays when the item breaks
- [`bucket_entity_data`](https://minecraft.wiki/w/Data_component_format#bucket_entity_data) - Specifies the entity NBT applied to an entity when placed down from a bucket
- [`bundle_contents`](https://minecraft.wiki/w/Data_component_format#bundle_contents) - Defines the items and their data stored inside a bundle
- [`can_break`](https://minecraft.wiki/w/Data_component_format#can_break) - When present, the player holding the item can break the specified blocks in adventure mode
- [`can_place_on`](https://minecraft.wiki/w/Data_component_format#can_place_on) - When present, the player holding the item can place the held block item on any sides of the specified blocks in adventure mode
- [`charged_projectiles`](https://minecraft.wiki/w/Data_component_format#charged_projectiles) - Defines the items and projectiles loaded into this crossbow
- [`consumable`](https://minecraft.wiki/w/Data_component_format#consumable) - If present, the item can be consumed
- [`container`](https://minecraft.wiki/w/Data_component_format#container) - Defines the items stored in a container item
- [`container_loot`](https://minecraft.wiki/w/Data_component_format#container_loot) - Defines the loot table present in a container item
- [`custom_data`](https://minecraft.wiki/w/Data_component_format#custom_data) - Store custom data on the item
- [`custom_model_data`](https://minecraft.wiki/w/Data_component_format#custom_model_data) - Specifies the custom model data
- [`custom_name`](https://minecraft.wiki/w/Data_component_format#custom_name) - Used to specify the item's custom name

### D-F

- [`damage`](https://minecraft.wiki/w/Data_component_format#damage) - Specifies the amount of durability lost (not remaining) from the item
- [`damage_resistant`](https://minecraft.wiki/w/Data_component_format#damage_resistant) - Items with this component, when thrown on the ground, are resistant to the damage types included
- [`debug_stick_state`](https://minecraft.wiki/w/Data_component_format#debug_stick_state) - Used by the debug stick item to store the selected block state properties
- [`death_protection`](https://minecraft.wiki/w/Data_component_format#death_protection) - If present, this item acts like a totem of undying
- [`dyed_color`](https://minecraft.wiki/w/Data_component_format#dyed_color) - Defines the color of this leather armor piece
- [`enchantable`](https://minecraft.wiki/w/Data_component_format#enchantable) - Defines whether the item can be enchanted in an enchanting table
- [`enchantment_glint_override`](https://minecraft.wiki/w/Data_component_format#enchantment_glint_override) - Overrides the enchantment glint effect on this item
- [`enchantments`](https://minecraft.wiki/w/Data_component_format#enchantments) - Specifies the enchantments on this item
- [`entity_data`](https://minecraft.wiki/w/Data_component_format#entity_data) - Specifies the NBT data of an entity that is spawned/created from this item
- [`equippable`](https://minecraft.wiki/w/Data_component_format#equippable) - Specifies that an item can be equipped in the specified slot
- [`firework_explosion`](https://minecraft.wiki/w/Data_component_format#firework_explosion) - The firework explosion effect stored inside of this firework star
- [`fireworks`](https://minecraft.wiki/w/Data_component_format#fireworks) - The firework explosion effects stored inside of this firework rocket
- [`food`](https://minecraft.wiki/w/Data_component_format#food) - The food stats for this consumable item

### G-M

- [`glider`](https://minecraft.wiki/w/Data_component_format#glider) - If present, this item allows players to glide when equipped
- [`instrument`](https://minecraft.wiki/w/Data_component_format#instrument) - Specifies the instrument of a goat horn
- [`intangible_projectile`](https://minecraft.wiki/w/Data_component_format#intangible_projectile) - If set, this projectile item can't be picked up by a player when fired
- [`item_model`](https://minecraft.wiki/w/Data_component_format#item_model) - Defines the item model of this item
- [`item_name`](https://minecraft.wiki/w/Data_component_format#item_name) - The default name of this item, as a text component
- [`jukebox_playable`](https://minecraft.wiki/w/Data_component_format#jukebox_playable) - Specifies a jukebox song to play when inserted into a jukebox
- [`lock`](https://minecraft.wiki/w/Data_component_format#lock) - Locks a container to a certain "key"
- [`lodestone_tracker`](https://minecraft.wiki/w/Data_component_format#lodestone_tracker) - Stores information about the lodestone this compass should point toward
- [`lore`](https://minecraft.wiki/w/Data_component_format#lore) - Specifies the item's lore - that being the description of an item
- [`map_color`](https://minecraft.wiki/w/Data_component_format#map_color) - Defines the color of the markings on the item of this map
- [`map_decorations`](https://minecraft.wiki/w/Data_component_format#map_decorations) - Contains the icons that are displayed on this filled map
- [`map_id`](https://minecraft.wiki/w/Data_component_format#map_id) - The id of the map to show
- [`max_damage`](https://minecraft.wiki/w/Data_component_format#max_damage) - The maximum amount of damage that this item can take
- [`max_stack_size`](https://minecraft.wiki/w/Data_component_format#max_stack_size) - The maximum number of items that can fit in a stack

### N-R

- [`note_block_sound`](https://minecraft.wiki/w/Data_component_format#note_block_sound) - The ID of the sound event played by a note block when this player head is placed above
- [`ominous_bottle_amplifier`](https://minecraft.wiki/w/Data_component_format#ominous_bottle_amplifier) - The amplifier amount of the Bad Omen effect on this ominous bottle
- [`pot_decorations`](https://minecraft.wiki/w/Data_component_format#pot_decorations) - Defines the sherds applied on a decorated pot
- [`potion_contents`](https://minecraft.wiki/w/Data_component_format#potion_contents) - The potion and custom effects contained in this potion
- [`potion_duration_scale`](https://minecraft.wiki/w/Data_component_format#potion_duration_scale) - The duration of the applied effects is scaled by this factor
- [`profile`](https://minecraft.wiki/w/Data_component_format#profile) - Information about the owner of this player head
- [`provides_banner_patterns`](https://minecraft.wiki/w/Data_component_format#provides_banner_patterns) - When present, this item can be placed in the pattern slot of a loom
- [`provides_trim_material`](https://minecraft.wiki/w/Data_component_format#provides_trim_material) - When present, this item provides the specified trim material when used in a trimming recipe
- [`rarity`](https://minecraft.wiki/w/Data_component_format#rarity) - Defines the rarity of the item
- [`recipes`](https://minecraft.wiki/w/Data_component_format#recipes) - The recipes that a player unlocks when this knowledge book is used
- [`repairable`](https://minecraft.wiki/w/Data_component_format#repairable) - Allows the item to be repaired, if damageable, in an anvil using the specified ingredient
- [`repair_cost`](https://minecraft.wiki/w/Data_component_format#repair_cost) - The number of experience levels to add to the base level cost

### S-Z

- [`stored_enchantments`](https://minecraft.wiki/w/Data_component_format#stored_enchantments) - Defines the stored enchantments on this item
- [`suspicious_stew_effects`](https://minecraft.wiki/w/Data_component_format#suspicious_stew_effects) - The effects applied when consuming this suspicious stew
- [`tool`](https://minecraft.wiki/w/Data_component_format#tool) - If set, this item is considered as a tool
- [`tooltip_display`](https://minecraft.wiki/w/Data_component_format#tooltip_display) - Hides any tooltips provided by the specified components on this item
- [`tooltip_style`](https://minecraft.wiki/w/Data_component_format#tooltip_style) - Changes the item's tooltip style
- [`trim`](https://minecraft.wiki/w/Data_component_format#trim) - Stores the trims applied on this piece of armor
- [`unbreakable`](https://minecraft.wiki/w/Data_component_format#unbreakable) - If set, this item doesn't lose durability when used
- [`use_cooldown`](https://minecraft.wiki/w/Data_component_format#use_cooldown) - If present, this item applies a cooldown to all items of the same type when it has been used
- [`use_remainder`](https://minecraft.wiki/w/Data_component_format#use_remainder) - If present, replaces the item with a remainder item if its stack count has decreased after use
- [`weapon`](https://minecraft.wiki/w/Data_component_format#weapon) - If present, the item acts as a weapon
- [`writable_book_content`](https://minecraft.wiki/w/Data_component_format#writable_book_content) - Defines the content written in this book and quill
- [`written_book_content`](https://minecraft.wiki/w/Data_component_format#written_book_content) - Defines the content and metadata in this written book

## Entity Variant Components

Entity variant components modify properties of entities stored within items like spawn eggs, mob buckets, etc.

- [`axolotl/variant`](https://minecraft.wiki/w/Data_component_format#axolotl/variant) - The variant of the axolotl
- [`cat/collar`](https://minecraft.wiki/w/Data_component_format#cat/collar) - The color of the collar of the cat
- [`cat/variant`](https://minecraft.wiki/w/Data_component_format#cat/variant) - The variant of the cat
- [`chicken/variant`](https://minecraft.wiki/w/Data_component_format#chicken/variant) - The variant of the chicken
- [`cow/variant`](https://minecraft.wiki/w/Data_component_format#cow/variant) - The variant of the cow
- [`fox/variant`](https://minecraft.wiki/w/Data_component_format#fox/variant) - The variant of the fox
- [`frog/variant`](https://minecraft.wiki/w/Data_component_format#frog/variant) - The variant of the frog
- [`horse/variant`](https://minecraft.wiki/w/Data_component_format#horse/variant) - The variant of the horse
- [`llama/variant`](https://minecraft.wiki/w/Data_component_format#llama/variant) - The variant of the llama
- [`mooshroom/variant`](https://minecraft.wiki/w/Data_component_format#mooshroom/variant) - The variant of the mooshroom
- [`painting/variant`](https://minecraft.wiki/w/Data_component_format#painting/variant) - The variant of the painting
- [`parrot/variant`](https://minecraft.wiki/w/Data_component_format#parrot/variant) - The variant of the parrot
- [`pig/variant`](https://minecraft.wiki/w/Data_component_format#pig/variant) - The variant of the pig
- [`rabbit/variant`](https://minecraft.wiki/w/Data_component_format#rabbit/variant) - The variant of the rabbit
- [`salmon/size`](https://minecraft.wiki/w/Data_component_format#salmon/size) - The size of the salmon
- [`sheep/color`](https://minecraft.wiki/w/Data_component_format#sheep/color) - The color of the wool of the sheep
- [`shulker/color`](https://minecraft.wiki/w/Data_component_format#shulker/color) - The color the shulker
- [`tropical_fish/base_color`](https://minecraft.wiki/w/Data_component_format#tropical_fish/base_color) - The base color of the tropical fish
- [`tropical_fish/pattern`](https://minecraft.wiki/w/Data_component_format#tropical_fish/pattern) - The pattern of the tropical fish
- [`tropical_fish/pattern_color`](https://minecraft.wiki/w/Data_component_format#tropical_fish/pattern_color) - The pattern color of the tropical fish
- [`villager/variant`](https://minecraft.wiki/w/Data_component_format#villager/variant) - The variant of the villager
- [`wolf/collar`](https://minecraft.wiki/w/Data_component_format#wolf/collar) - The color of the collar of the wolf
- [`wolf/sound_variant`](https://minecraft.wiki/w/Data_component_format#wolf/sound_variant) - The sound variant of the wolf
- [`wolf/variant`](https://minecraft.wiki/w/Data_component_format#wolf/variant) - The variant of the wolf

## Non-Encoded Components

These components are used by the game internally, but are not encoded on items. They cannot be used in commands or seen with `/data`.

- [`creative_slot_lock`](https://minecraft.wiki/w/Data_component_format#creative_slot_lock) - Used internally to lock the informational paper items in the creative inventory
- [`map_post_processing`](https://minecraft.wiki/w/Data_component_format#map_post_processing) - Used internally when a filled map has been duplicated or locked

## Implementation Notes

When implementing these components in our Rust project, we'll need to consider:

1. **Data structures** - Create appropriate Rust structs for each component
2. **Serialization/Deserialization** - Handle conversion between in-memory representation and NBT/JSON
3. **Validation** - Ensure component values meet the constraints defined in the wiki
4. **Component interactions** - Handle interactions between components (e.g., mutual exclusivity)
5. **Default values** - Properly implement default values for optional fields

## References

- [Minecraft Wiki: Data Component Format](https://minecraft.wiki/w/Data_component_format) - Main documentation page for data components
- [Minecraft Wiki: Text Component Format](https://minecraft.wiki/w/Text_component_format) - Documentation for text components used in various data components
- [Minecraft Wiki: NBT Format](https://minecraft.wiki/w/NBT_format) - Documentation for the NBT format used for data storage