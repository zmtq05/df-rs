# df-rs

Neople OpenAPI(dungeon and fighter) client written in rust

[![](res/기술표기_가로형_color.png)](https://developers.neople.co.kr)

## Example

```rust
// make client
let client = df_rs::DfClient::new("<YOUR_API_KEY>");
// or
df_rs::initalize("<YOUR_API_KEY>"); // call it only once.
let client = df_rs::instance(); // global instance

// search auction
let auction_search_result = client.auction().item_name("haystack").search().await;

// search character
let search_character_result = client.character().name("haystack").search().await;

// get equipments, avatars, creature ... of character
let character: &Character = search_character_result.unwrap().get(0).unwrap();
let character_client = client.character().of(character);

let equipments = character_client.equipments().await?;
let avatars = character_client.avatars().await?;
let creature = character_client.creature().await?;

// get image
let image_bytes = character_client.image(1 /* zoom level */).await?;
// same
let image_bytes = client.image().character(character, 1 /* zoom level */).await?;
```