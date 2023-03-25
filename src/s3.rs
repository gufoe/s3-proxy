use crate::config::Config;

pub fn get_s3_file_url(config: &Config, file: &str) -> String {
    use rusoto_credential::AwsCredentials;
    use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
    use rusoto_s3::GetObjectRequest;

    let credentials = AwsCredentials::new(&config.s3_key, &config.s3_secret, None, None);

    let options = PreSignedRequestOption {
        expires_in: std::time::Duration::from_secs(300),
    };

    // return HttpResponse::NotFound().body("404 - Not found");
    let region = rusoto_core::Region::Custom {
        name: config.s3_region.to_string(),
        endpoint: config.s3_host.to_string(),
    };

    (GetObjectRequest {
        bucket: config.s3_bucket.to_string(),
        key: file.to_owned(),
        ..Default::default()
    })
    .get_presigned_url(&region, &credentials, &options)
}
