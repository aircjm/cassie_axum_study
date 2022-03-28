use crate::CASSIE_CONFIG;
use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Result;
use oss_rust_sdk::prelude::*;

use super::upload_service::IUploadService;
use cassie_common::error::Error;
use std::{collections::HashMap, sync::Arc};
/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
pub const CONTENT_TYPE: &str = "content-type";
pub struct OssService {
    access_endpoint: String,
}
impl OssService {
    /**
     * @description:  get_client 单例模式获取 ossclient
     * @param: null
     * @return:
     * @author String
     * @date: 2022/3/28 9:42
     */
    pub async fn get_client(&self) -> &OSS<'static> {
        static mut POINT: Option<Arc<OSS<'static>>> = None;
        unsafe {
            POINT.get_or_insert_with(|| {
                Arc::new(OSS::new(
                    CASSIE_CONFIG.oss.key_id.as_str(),
                    CASSIE_CONFIG.oss.key_secret.as_str(),
                    CASSIE_CONFIG.oss.endpoint.as_str(),
                    CASSIE_CONFIG.oss.bucket.as_str(),
                ))
            });
            POINT.as_ref().unwrap()
        }
    }
    pub fn new(access_endpoint: String) -> OssService {
        OssService {
            access_endpoint: access_endpoint,
        }
    }
}

#[async_trait]
impl IUploadService for OssService {
    async fn upload(&self, data: Bytes, file_name: String, content_type: String) -> Result<String> {
        let service = self.get_client().await;
        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE, content_type.as_str());
        let result = service
            .async_put_object_from_buffer(&data, file_name.clone(), headers, None)
            .await;
        match result {
            Ok(_) => {
                let path = format!("{}/{}", self.access_endpoint.clone(), file_name.clone());
                return Ok(path);
            }
            Err(e) => Err(Error::E(e.to_string())),
        }
    }
}
