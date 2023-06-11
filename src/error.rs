use std::fmt::Display;

use reqwest::Response;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Api(#[from] ApiError),
}

#[derive(Debug, Error, Clone, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub code: ErrorCode,
    pub message: String,
}

impl ApiError {
    pub(crate) async fn from_response(response: Response) -> Self {
        #[derive(Deserialize)]
        struct OuterError {
            error: ApiError,
        }

        // origin: { "error": { "status": 404, ... } }

        response.json::<OuterError>().await.unwrap().error
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: better formatting
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ErrorCode {
    /// API Key 미입력
    API000,

    /// 유효하지 않은 게임아이디
    API001,

    /// API Key  사용량 초과
    API002,

    /// 유효하지 않은 API Key
    API003,

    /// 차단된 API Key
    API004,

    /// 해당 게임으로 발급되지 않은 API Key
    API005,

    /// 유효하지 않은 HTTP 헤더 요청  
    API006,

    /// 클라이언트 소켓 통신 오류
    API007,

    /// 유효하지 않은 URL
    API900,

    /// 유효하지 않은 요청 파라미터
    API901,

    /// 시스템 오류
    API999,

    /// 유효하지 않은 서버아이디
    DNF000,

    /// 유효하지 않은 캐릭터 정보
    DNF001,

    /// 유효하지 않은 아이템 정보
    DNF003,

    /// 유효하지 않은 경매장 및 아바타마켓 상품 정보
    DNF004,

    /// 유효하지 않은 스킬 정보
    DNF005,

    /// 타임라인 검색 시간 파라미터 오류
    DNF006,

    /// 경매장 아이템 검색 갯수 제한
    DNF007,

    /// 다중 아이템 검색 갯수 제한
    DNF008,

    /// 아바타 마켓 타이틀 검색 갯수 제한
    DNF009,

    /// 유효하지 않은 URL
    DNF900,

    /// 유효하지 않은 요청 파라미터
    DNF901,

    /// 시스템 점검
    DNF980,

    /// 시스템 오류
    DNF999,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::API000 => write!(f, "API000"),
            Self::API001 => write!(f, "API001"),
            Self::API002 => write!(f, "API002"),
            Self::API003 => write!(f, "API003"),
            Self::API004 => write!(f, "API004"),
            Self::API005 => write!(f, "API005"),
            Self::API006 => write!(f, "API006"),
            Self::API007 => write!(f, "API007"),
            Self::API900 => write!(f, "API900"),
            Self::API901 => write!(f, "API901"),
            Self::API999 => write!(f, "API999"),
            Self::DNF000 => write!(f, "DNF000"),
            Self::DNF001 => write!(f, "DNF001"),
            Self::DNF003 => write!(f, "DNF003"),
            Self::DNF004 => write!(f, "DNF004"),
            Self::DNF005 => write!(f, "DNF005"),
            Self::DNF006 => write!(f, "DNF006"),
            Self::DNF007 => write!(f, "DNF007"),
            Self::DNF008 => write!(f, "DNF008"),
            Self::DNF009 => write!(f, "DNF009"),
            Self::DNF900 => write!(f, "DNF900"),
            Self::DNF901 => write!(f, "DNF901"),
            Self::DNF980 => write!(f, "DNF980"),
            Self::DNF999 => write!(f, "DNF999"),
        }
    }
}

impl ErrorCode {
    pub fn description(&self) -> &'static str {
        match self {
            Self::API000 => "API Key 미입력",
            Self::API001 => "유효하지 않은 게임아이디",
            Self::API002 => "API Key  사용량 초과",
            Self::API003 => "유효하지 않은 API Key",
            Self::API004 => "차단된 API Key",
            Self::API005 => "해당 게임으로 발급되지 않은 API Key",
            Self::API006 => "유효하지 않은 HTTP 헤더 요청  ",
            Self::API007 => "클라이언트 소켓 통신 오류",
            Self::API900 => "유효하지 않은 URL",
            Self::API901 => "유효하지 않은 요청 파라미터",
            Self::API999 => "시스템 오류",
            Self::DNF000 => "유효하지 않은 서버아이디",
            Self::DNF001 => "유효하지 않은 캐릭터 정보",
            Self::DNF003 => "유효하지 않은 아이템 정보",
            Self::DNF004 => "유효하지 않은 경매장 및 아바타마켓 상품 정보",
            Self::DNF005 => "유효하지 않은 스킬 정보",
            Self::DNF006 => "타임라인 검색 시간 파라미터 오류",
            Self::DNF007 => "경매장 아이템 검색 갯수 제한",
            Self::DNF008 => "다중 아이템 검색 갯수 제한",
            Self::DNF009 => "아바타 마켓 타이틀 검색 갯수 제한",
            Self::DNF900 => "유효하지 않은 URL",
            Self::DNF901 => "유효하지 않은 요청 파라미터",
            Self::DNF980 => "시스템 점검",
            Self::DNF999 => "시스템 오류",
        }
    }
}
