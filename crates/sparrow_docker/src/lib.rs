use bollard::{Docker, image::ListImagesOptions, service::ImageSummary};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DOCKER: Docker = Docker::connect_with_local_defaults().unwrap();
}

async fn list_images() -> Vec<ImageSummary> {

    let images = &DOCKER.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    images.clone()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn testing() -> Result<(), Box<dyn std::error::Error>> {
        let images = list_images();
        for image in images.await.into_iter() {
            println!("{:#?}", image)
        }
        Ok(())
    }
}
