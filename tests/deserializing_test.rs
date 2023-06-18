use std::env;

use df_rs::{
    error::{ApiError, ErrorCode},
    Error,
};

fn client() -> df_rs::DfClient {
    df_rs::DfClient::new(&env::var("API_KEY").expect("set API_KEY env var"))
}

fn is_limit_exceeded<T>(result: &Result<T, Error>) -> bool {
    match result {
        Err(Error::Api(ApiError { code, .. })) => *code == ErrorCode::API002,
        _ => false,
    }
}

macro_rules! retry_if_limit_exceeded {
    ($e:expr) => {{
        let mut result = $e;
        while $crate::is_limit_exceeded(&result) {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            result = $e;
        }
        result
    }};
}

mod auction {
    use super::client;

    #[tokio::test]
    async fn search() {
        let result = retry_if_limit_exceeded!(
            client()
                .auction()
                .item_name("무색 큐브 조각")
                .search()
                .await
        );

        // println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn sold() {
        let result =
            retry_if_limit_exceeded!(client().auction().item_name("무색 큐브 조각").sold().await);

        // println!("{:#?}", result);
        assert!(result.is_ok());
    }
}

mod item {
    use super::client;

    #[tokio::test]
    async fn search() {
        let result = retry_if_limit_exceeded!(client().item().search("무색 큐브 조각").await);

        // println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn info() {
        let result = retry_if_limit_exceeded!(
            client()
                .item()
                .info("785e56a0ed4e3efd573da1f56a45217d")
                .await
        );

        // println!("{:#?}", result);
        assert!(result.is_ok());
    }
}

mod character {
    use df_rs::model::Character;

    use crate::client;

    #[tokio::test]
    async fn search() {
        let result = retry_if_limit_exceeded!(client().character().name("김철수").search().await);

        // println!("{:#?}", result);
        assert!(result.is_ok());
    }

    async fn get_characters() -> Result<Vec<Character>, df_rs::Error> {
        retry_if_limit_exceeded!(client().character().name("김철수").search().await)
            .map(|vec| vec[..vec.len().min(5)].to_vec())
    }

    #[tokio::test]
    async fn info() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result = retry_if_limit_exceeded!(client.character().of(character).info().await);

            println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn equipments() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/equip/equipment",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result =
                retry_if_limit_exceeded!(client.character().of(character).equipments().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn avatars() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/equip/avatar",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result = retry_if_limit_exceeded!(client.character().of(character).avatars().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn creature() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/equip/creature",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result =
                retry_if_limit_exceeded!(client.character().of(character).creature().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn flag() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/equip/flag",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result = retry_if_limit_exceeded!(client.character().of(character).flag().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn talismans() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/equip/talisman",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result =
                retry_if_limit_exceeded!(client.character().of(character).talismans().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn buff_equipments() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/skill/buff/equip/equipment",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result = retry_if_limit_exceeded!(
                client.character().of(character).buff().equipments().await
            );

            println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn buff_avatars() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/skill/buff/equip/avatar",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result =
                retry_if_limit_exceeded!(client.character().of(character).buff().avatars().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn buff_creature() {
        let client = client();
        let characters = get_characters().await.unwrap();
        // characters.iter().for_each(|character| {
        //     println!(
        //         "https://api.neople.co.kr/df/servers/{}/characters/{}/skill/buff/equip/creature",
        //         character.server, character.id
        //     );
        // });
        for character in &characters {
            let result =
                retry_if_limit_exceeded!(client.character().of(character).buff().creature().await);

            // println!("{:#?}", result);
            assert!(result.is_ok());
        }
    }
}
