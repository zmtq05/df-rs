//! TODO: UNTESTED

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    model::{
        buff::BuffEnhance, Character, CharacterAvatars, CharacterCreature, CharacterEquipments,
        CharacterFlag, CharacterInfo, CharacterTalismans, Server,
    },
    DfClient, Result,
};

use super::auction::WordType;

pub struct CharacterHandler<'df> {
    client: &'df DfClient,
}

impl<'df> CharacterHandler<'df> {
    pub(crate) fn new(client: &'df DfClient) -> Self {
        Self { client }
    }

    pub fn search(&self) -> CharacterSearchBuilder<'df> {
        CharacterSearchBuilder::new(self.client)
    }

    pub fn _of(&self, server: Server, character_id: &str) -> SpecificCharacterHandler<'df> {
        SpecificCharacterHandler::new(self.client, server, character_id)
    }

    pub fn of(&self, character: &Character) -> SpecificCharacterHandler<'df> {
        self._of(character.server, &character.id)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSearchBuilder<'df> {
    #[serde(skip)]
    client: &'df DfClient,
    #[serde(skip)]
    server: Server,
    #[serde(skip)]
    name: Option<String>,

    job_id: Option<String>,
    job_grow_id: Option<String>,
    word_type: Option<WordType>,
    limit: Option<u8>,
}

impl<'df> CharacterSearchBuilder<'df> {
    fn new(client: &'df DfClient) -> Self {
        Self {
            client,
            server: Server::All,
            name: None,
            job_id: None,
            job_grow_id: None,
            word_type: None,
            limit: None,
        }
    }

    pub fn server(mut self, server: Server) -> Self {
        self.server = server;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }

    pub fn job_grow_id(mut self, job_grow_id: String) -> Self {
        self.job_grow_id = Some(job_grow_id);
        self
    }

    pub fn word_type(mut self, word_type: WordType) -> Self {
        self.word_type = Some(word_type);
        self
    }

    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// # Panics
    ///
    /// You must set `name` before calling this method.
    pub async fn execute(&self) -> Result<Vec<Character>> {
        let resp = self
            .client
            .inner
            .get(format!(
                "{BASE_URL}/servers/{server}/characters?characterName={name}",
                BASE_URL = crate::DF_BASE_URL,
                server = self.server,
                name = self.name.clone().expect("name must be set"),
            ))
            .query(self)
            .send()
            .await?;
        let resp = crate::map_api_error(resp).await?;

        let characters = unwrap_rows!(resp, Character);
        Ok(characters)
    }
}

pub struct SpecificCharacterHandler<'df> {
    client: &'df DfClient,
    server: Server,
    character_id: String,
}

impl<'df> SpecificCharacterHandler<'df> {
    fn new(client: &'df DfClient, server: Server, character_id: &str) -> Self {
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
                "{BASE_URL}/servers/{server}/characters/{id}/{dst}",
                BASE_URL = crate::DF_BASE_URL,
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

    pub fn buff(&self) -> SpecificCharacterBuffHandler<'_> {
        SpecificCharacterBuffHandler::new(self)
    }
}

pub struct SpecificCharacterBuffHandler<'df> {
    handler: &'df SpecificCharacterHandler<'df>,
}

impl<'df> SpecificCharacterBuffHandler<'df> {
    fn new(handler: &'df SpecificCharacterHandler<'df>) -> Self {
        Self { handler }
    }

    async fn get<T: DeserializeOwned>(&self, dst: &str) -> Result<T> {
        self.handler.get(&format!("skill/buff/equip/{dst}")).await
    }

    pub async fn equipments(&self) -> Result<BuffEnhance> {
        self.get("equipment").await
    }

    pub async fn avatars(&self) -> Result<BuffEnhance> {
        self.get("avatar").await
    }

    pub async fn creature(&self) -> Result<BuffEnhance> {
        self.get("creature").await
    }
}
