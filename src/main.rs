use coderunner::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    coderunner::main().await
}
