use bollard::{Docker, image::ListImagesOptions, service::{ImageSummary, ContainerSummary}, container::{ListContainersOptions, StatsOptions, Stats}};


use futures::{TryStreamExt, TryFutureExt};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref DOCKER: Docker = Docker::connect_with_local_defaults().unwrap();
}

async fn list_containers() -> Vec<ContainerSummary> {
    let containers = &DOCKER.list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    containers.clone()
}

async fn list_images() -> Vec<ImageSummary> {

    let images = &DOCKER.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    images.clone()
}

async fn get_container_stats(id: String) -> Vec<Stats> {
    let stats = &DOCKER.stats(&id, Some(StatsOptions {
        stream: true,
        ..Default::default()
     })).try_collect::<Vec<_>>().await.unwrap();

     stats.clone()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn test_container_stats() -> Result<(), Box<dyn std::error::Error>> {
        let containers = list_containers();
        for container in containers.await.into_iter() {
            let stats = get_container_stats(container.id.unwrap()).await;
            println!("{:#?}", stats)
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_image_summary() -> Result<(), Box<dyn std::error::Error>> {
        let images = list_images();
        for image in images.await.into_iter() {
            println!("{:#?}", image)
        }
        Ok(())
    }
    
    #[tokio::test]
    async fn test_container_list() -> Result<(), Box<dyn std::error::Error>> {
        let containers = list_containers();
        for container in containers.await.into_iter() {
            println!("{:#?}", container)
        }
        Ok(())
    }
}
