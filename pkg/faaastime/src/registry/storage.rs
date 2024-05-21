use anyhow::Result;
use aws_config::retry::RetryConfig;
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::ProvideCredentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use bytes::Bytes;

#[derive(Debug)]
pub struct StorageCredentials {
    access_key_id: String,
    secret_access_key: String,
}

impl StorageCredentials {
    pub fn from_env() -> Self {
        let access_key_id = std::env::var("ACCESS_KEY_ID").expect("missing $ACCESS_KEY_ID");
        let secret_access_key =
            std::env::var("SECRET_ACCESS_KEY").expect("missing $SECRET_ACCESS_KEY");

        Self {
            access_key_id,
            secret_access_key,
        }
    }

    async fn load(&self) -> aws_credential_types::provider::Result {
        Ok(Credentials::new(
            self.access_key_id.clone(),
            self.secret_access_key.clone(),
            None,
            None,
            "StaticCredentials",
        ))
    }
}

impl ProvideCredentials for StorageCredentials {
    fn provide_credentials<'a>(
        &'a self,
    ) -> aws_credential_types::provider::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        aws_credential_types::provider::future::ProvideCredentials::new(self.load())
    }
}

pub struct Storage {
    client: aws_sdk_s3::Client,
}

impl Storage {
    pub fn new() -> Self {
        let s3_endpoint_url = "http://localhost:9000";
        let s3_region = Region::new("us-west-2");
        let s3_creds = SharedCredentialsProvider::new(StorageCredentials::from_env());

        let s3_cfg = aws_sdk_s3::Config::builder()
            .endpoint_url(s3_endpoint_url)
            .region(s3_region)
            .credentials_provider(s3_creds)
            .retry_config(RetryConfig::standard().with_max_attempts(3))
            .force_path_style(true)
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_cfg);

        Self { client }
    }

    pub async fn get_component_bytes(&self, component_id: &str) -> Result<Bytes> {
        let key = Storage::get_component_uri(component_id);

        let bytes = self
            .client
            .get_object()
            .bucket("universe")
            .key(key)
            .send()
            .await?
            .body
            .collect()
            .await?
            .into_bytes();

        Ok(bytes)
    }

    fn get_component_uri(component_id: &str) -> &str {
        match component_id {
            "faaas:runjs" => "runjs.wasm",
            s => s,
        }
    }
}
