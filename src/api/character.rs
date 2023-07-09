use bytes::Bytes;
use futures::join;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::PrimitiveDateTime;
use urlencoding::encode;

use crate::{
    model::{
        buff::CharacterBuffEnhance, Character, CharacterAvatars, CharacterCreature,
        CharacterEquipments, CharacterFlag, CharacterInfo, CharacterTalismans, CharacterTimeline,
        Server,
    },
    DfClient, Result,
};

use super::WordType;

#[derive(Clone)]
pub struct CharacterHandler {
    client: DfClient,
    param: CharacterSearchParameter,
}

/// # Constructor
impl CharacterHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        Self {
            client,
            param: Default::default(),
        }
    }
}

/// # Send Request
impl CharacterHandler {
    /// Search characters by name.
    ///
    /// # Panics
    /// Panics if [`CharacterHandler::name`] is not called.
    pub async fn search(&self) -> Result<Vec<Character>> {
        let param = &self.param;
        let resp = self
            .client
            .get_with_query(
                &format!(
                    "/servers/{server}/characters?characterName={name}",
                    server = param.server,
                    name = encode(&param.name),
                ),
                Some(param),
            )
            .await?;

        let characters = unwrap_rows!(resp, Character);
        Ok(characters)
    }
}

/// # Parameter
impl CharacterHandler {
    pub fn name(&mut self, character_name: impl Into<String>) -> &mut Self {
        self.param.name = character_name.into();
        self
    }

    pub fn server(&mut self, server: Server) -> &mut Self {
        self.param.server = server;
        self
    }

    pub fn job_id(&mut self, job_id: impl Into<String>) -> &mut Self {
        self.param.job_id = Some(job_id.into());
        self
    }

    pub fn job_grow_id(&mut self, job_grow_id: impl Into<String>) -> &mut Self {
        self.param.job_grow_id = Some(job_grow_id.into());
        self
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.param.limit = Some(limit);
        self
    }

    pub fn word_type(&mut self, word_type: WordType) -> &mut Self {
        self.param.word_type = Some(word_type);
        self
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSearchParameter {
    #[serde(skip)]
    pub server: Server,
    #[serde(skip)]
    pub name: String,
    pub job_id: Option<String>,
    pub job_grow_id: Option<String>,
    pub word_type: Option<WordType>,
    pub limit: Option<u8>,
}

/// # Constructor of [`SpecificCharacterHandler`]
impl CharacterHandler {
    pub fn of(&self, character: &Character) -> SpecificCharacterHandler {
        self._of(character.server, &character.id)
    }

    pub fn _of(&self, server: Server, character_id: &str) -> SpecificCharacterHandler {
        SpecificCharacterHandler::new(self.client.clone(), server, character_id)
    }
}

#[derive(Clone)]
pub struct SpecificCharacterHandler {
    client: DfClient,
    pub server: Server,
    pub character_id: String,
}

impl SpecificCharacterHandler {
    fn new(client: DfClient, server: Server, character_id: &str) -> Self {
        Self {
            client,
            server,
            character_id: character_id.to_owned(),
        }
    }

    pub fn buff(&self) -> SpecificCharacterBuffHandler {
        SpecificCharacterBuffHandler::new(self.clone())
    }
}

/// # Send Request
impl SpecificCharacterHandler {
    async fn get<T: DeserializeOwned>(&self, dst: &str) -> Result<T> {
        let resp = self
            .client
            .get(&format!(
                "/servers/{server}/characters/{id}/{dst}", // trailing slash is allowed
                server = self.server,
                id = self.character_id,
                dst = dst,
            ))
            .await?;

        Ok(resp.json().await.unwrap())
    }

    /// Get character information.
    pub async fn info(&self) -> Result<CharacterInfo> {
        self.get("").await
    }

    pub async fn timeline(&self, param: Option<&TimelineParameter>) -> Result<CharacterTimeline> {
        let resp = self
            .client
            .get_with_query(
                &format!(
                    "/servers/{server}/characters/{id}/timeline",
                    server = self.server,
                    id = self.character_id,
                ),
                param,
            )
            .await?;

        Ok(resp.json().await.unwrap())
    }

    /// Get character equipments.
    pub async fn equipments(&self) -> Result<CharacterEquipments> {
        self.get("equip/equipment").await
    }

    /// Get character avatars.
    pub async fn avatars(&self) -> Result<CharacterAvatars> {
        self.get("equip/avatar").await
    }

    /// Get character creature.
    pub async fn creature(&self) -> Result<CharacterCreature> {
        self.get("equip/creature").await
    }

    /// Get character flag.
    pub async fn flag(&self) -> Result<CharacterFlag> {
        self.get("equip/flag").await
    }

    /// Get character talismans.
    pub async fn talismans(&self) -> Result<CharacterTalismans> {
        self.get("equip/talisman").await
    }

    /// Get character image.
    ///
    /// # Arguments
    ///
    /// * `zoom` - Zoom level. 1 to 3.
    pub async fn image(&self, zoom: u8) -> Result<Bytes> {
        self.client
            .image()
            ._character(self.server, &self.character_id, zoom)
            .await
    }
}

#[derive(Clone)]
pub struct SpecificCharacterBuffHandler {
    handler: SpecificCharacterHandler,
}

impl SpecificCharacterBuffHandler {
    fn new(handler: SpecificCharacterHandler) -> Self {
        Self { handler }
    }

    async fn get(&self, dst: &str) -> Result<CharacterBuffEnhance> {
        self.handler.get(&format!("skill/buff/equip/{dst}")).await
    }

    /// [`BuffEnhance::avatars`] and [`BuffEnhance::creature`] are always `None`.
    ///
    /// [`BuffEnhance::avatars`]: crate::model::buff::BuffEnhance#avatars
    /// [`BuffEnhance::creature`]: crate::model::buff::BuffEnhance#creature
    pub async fn equipments(&self) -> Result<CharacterBuffEnhance> {
        self.get("equipment").await
    }

    /// [`BuffEnhance::equipments`] and [`BuffEnhance::creature`] are always `None`.
    ///
    /// [`BuffEnhance::equipments`]: crate::model::buff::BuffEnhance#equipments
    /// [`BuffEnhance::creature`]: crate::model::buff::BuffEnhance#creature
    pub async fn avatars(&self) -> Result<CharacterBuffEnhance> {
        self.get("avatar").await
    }

    /// [`BuffEnhance::equipments`] and [`BuffEnhance::avatars`] are always `None`.
    ///
    /// [`BuffEnhance::equipments`]: crate::model::buff::BuffEnhance#equipments
    /// [`BuffEnhance::avatars`]: crate::model::buff::BuffEnhance#avatars
    pub async fn creature(&self) -> Result<CharacterBuffEnhance> {
        self.get("creature").await
    }

    /// Convenience method. using [`futures::join`].
    pub async fn all(&self) -> Result<CharacterBuffEnhance> {
        let (e, a, c) = join![self.equipments(), self.avatars(), self.creature()];
        let mut e = e?;
        match &mut e.buff {
            None => {
                return Ok(e);
            }
            Some(buff) => {
                buff.avatars = a?.buff.unwrap().avatars;
                buff.creature = c?.buff.unwrap().creature;
            }
        }
        Ok(e)
    }
}

impl Default for CharacterSearchParameter {
    fn default() -> Self {
        Self {
            server: Server::All,
            name: Default::default(),
            job_id: Default::default(),
            job_grow_id: Default::default(),
            word_type: Default::default(),
            limit: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineParameter {
    #[serde(with = "timeline_format::option")]
    pub start_date: Option<PrimitiveDateTime>,
    #[serde(with = "timeline_format::option")]
    pub end_date: Option<PrimitiveDateTime>,
    pub limit: Option<u8>,
    pub code: Option<String>,
    pub next: Option<String>,
}

time::serde::format_description!(
    timeline_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]"
);
