//! TODO: UNTESTED

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
    pub(crate) fn new(client: DfClient) -> Self {
        Self {
            client,
            search_param: None,
        }
    }

    fn param_mut(&mut self) -> &mut CharacterSearchParameter {
        self.search_param.get_or_insert_with(Default::default)
    }

    pub fn param(mut self, param: CharacterSearchParameter) -> Self {
        self.search_param = Some(param);
        self
    }

    pub fn name(mut self, character_name: impl Into<String>) -> Self {
        self.param_mut().name(character_name);
        self
    }

    pub fn server(mut self, server: Server) -> Self {
        self.param_mut().server(server);
        self
    }

    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.param_mut().job_id(job_id);
        self
    }

    pub fn job_grow_id(mut self, job_grow_id: impl Into<String>) -> Self {
        self.param_mut().job_grow_id(job_grow_id);
        self
    }

    pub fn limit(mut self, limit: u8) -> Self {
        self.param_mut().limit(limit);
        self
    }

    pub fn word_type(mut self, word_type: WordType) -> Self {
        self.param_mut().word_type(word_type);
        self
    }

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

    pub fn _of(&self, server: Server, character_id: &str) -> SpecificCharacterHandler {
        SpecificCharacterHandler::new(self.client.clone(), server, character_id)
    }

    pub fn of(&self, character: &Character) -> SpecificCharacterHandler {
        self._of(character.server, &character.id)
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

impl CharacterSearchParameter {
    pub fn server(&mut self, server: Server) -> &mut Self {
        self.server = server;
        self
    }

    pub fn job_id(&mut self, job_id: impl Into<String>) -> &mut Self {
        self.job_id = Some(job_id.into());
        self
    }

    pub fn job_grow_id(&mut self, job_grow_id: impl Into<String>) -> &mut Self {
        self.job_grow_id = Some(job_grow_id.into());
        self
    }

    pub fn word_type(&mut self, word_type: WordType) -> &mut Self {
        self.word_type = Some(word_type);
        self
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
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

    pub async fn info(&self) -> Result<CharacterInfo> {
        self.get("").await
    }

    pub async fn equipments(&self) -> Result<CharacterEquipments> {
        self.get("equip/equipment").await
    }

    pub async fn avatars(&self) -> Result<CharacterAvatars> {
        self.get("equip/avatar").await
    }

    pub async fn creature(&self) -> Result<CharacterCreature> {
        self.get("equip/creature").await
    }

    pub async fn flag(&self) -> Result<CharacterFlag> {
        self.get("equip/flag").await
    }

    pub async fn talismans(&self) -> Result<CharacterTalismans> {
        self.get("equip/talisman").await
    }

    pub fn buff(&self) -> SpecificCharacterBuffHandler {
        SpecificCharacterBuffHandler::new(self.clone())
    }

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
    pub async fn equipments(&self) -> Result<CharacterBuffEnhance> {
        self.get("equipment").await
    }

    /// [`BuffEnhance::equipments`] and [`BuffEnhance::creature`] are always `None`.
    pub async fn avatars(&self) -> Result<CharacterBuffEnhance> {
        self.get("avatar").await
    }

    /// [`BuffEnhance::equipments`] and [`BuffEnhance::avatars`] are always `None`.
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
