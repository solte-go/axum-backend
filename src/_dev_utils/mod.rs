mod dev_db;

//TODO learn about "OnceCell"
use tokio::sync::OnceCell;
use tracing::info;

pub async fn must_init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_envierment", "DEV-OPERATION");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}