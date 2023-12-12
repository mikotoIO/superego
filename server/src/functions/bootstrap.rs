use std::env;

use crate::prisma::{
    service::{self},
    PrismaClient,
};

use super::session::random_base64;

pub async fn bootstrap(prisma: &PrismaClient) {
    let root_domain = env::var("ROOT_DOMAIN").expect("ROOT_DOMAIN must be set");
    // check if root service exists
    let root_service = prisma
        .service()
        .find_unique(service::name::equals(root_domain.clone()))
        .exec()
        .await
        .expect("failed to check if root service exists");

    if root_service.is_none() {
        // create root service
        let root_service = prisma
            .service()
            .create("Root Service".to_string(), root_domain.clone(), vec![])
            .exec()
            .await
            .expect("failed to create root service");

        // create service key
        let _ = prisma
            .service_key()
            .create(
                service::id::equals(root_service.id),
                "HS256".to_string(),
                root_domain.clone(),
                random_base64(64).expect("Failed to generate random secret"),
                vec![],
            )
            .exec()
            .await
            .expect("failed to create root service key");
    }
}
