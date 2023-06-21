use bytes::Bytes;
use futures::join;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    model::{
        buff::CharacterBuffEnhance, Character, CharacterAvatars, CharacterCreature,
        CharacterEquipments, CharacterFlag, CharacterInfo, CharacterTalismans, Server,
    },
    DfClient, Result,
};

use super::WordType;

#[derive(Clone)]
pub struct CharacterHandler {
    client: DfClient,

    search_param: Option<CharacterSearchParameter>,
}

impl CharacterHandler {
    /// Search characters by name.
    ///
    /// # Panics
    /// Panics if [`CharacterHandler::name`] is not called.
    pub async fn search(&self) -> Result<Vec<Character>> {
        let param = self.search_param.as_ref().unwrap();
        let resp = self
            .client
            .get_with_query(
                &format!(
                    "/servers/{server}/characters?characterName={name}",
                    server = param.server,
                    name = param.name.as_ref().expect("must be call `name()`"),
                ),
                Some(param),
            )
            .await?;

        let characters = unwrap_rows!(resp, Character);
        Ok(characters)
    }

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
    server: Server,
    character_id: String,
}

impl SpecificCharacterHandler {
    fn new(client: DfClient, server: Server, character_id: &str) -> Self {
        Self {
            client,
            server,
            character_id: character_id.to_owned(),
        }
    }

    async fn get<T: DeserializeOwned>(&self, dst: &str) -> Result<T> {
        let resp = self
            .client
            .get(&format!(
                "/servers/{server}/characters/{id}/{dst}",
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

    pub fn buff(&self) -> SpecificCharacterBuffHandler {
        SpecificCharacterBuffHandler::new(self.clone())
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

impl CharacterHandler {
    pub(crate) fn new(client: DfClient) -> Self {
        Self {
            client,
            search_param: None,
        }
    }

    fn param_mut(&mut self) -> &mut CharacterSearchParameter {
        self.search_param.get_or_insert_with(Default::default)
    }

    pub fn name(mut self, character_name: impl Into<String>) -> Self {
        self.param_mut().name = Some(character_name.into());
        self
    }

    pub fn server(mut self, server: Server) -> Self {
        self.param_mut().server = server;
        self
    }

    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.param_mut().job_id = Some(job_id.into());
        self
    }

    pub fn job_grow_id(mut self, job_grow_id: impl Into<String>) -> Self {
        self.param_mut().job_grow_id = Some(job_grow_id.into());
        self
    }

    pub fn limit(mut self, limit: u8) -> Self {
        self.param_mut().limit = Some(limit);
        self
    }

    pub fn word_type(mut self, word_type: WordType) -> Self {
        self.param_mut().word_type = Some(word_type);
        self
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSearchParameter {
    #[serde(skip)]
    server: Server,
    #[serde(skip)]
    name: Option<String>,

    job_id: Option<String>,
    job_grow_id: Option<String>,
    word_type: Option<WordType>,
    limit: Option<u8>,
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
