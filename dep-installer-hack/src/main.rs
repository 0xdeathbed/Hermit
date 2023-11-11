use std::net::SocketAddr;
use shuttle_runtime::{Service, async_trait};
use tokio::time;

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<MyService,shuttle_runtime::Error> {
    Ok(MyService)
}

struct MyService;

#[async_trait]
impl Service for MyService {
    async fn bind(self, _adrr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        time::sleep(time::Duration::from_secs(2)).await;

        Ok(())
    }
}
