use tokio::net::TcpListener;
use tokio::process::Command;

pub async fn create_node() -> Result<(), std::io::Error> {
  let listener = TcpListener::bind("127.0.0.1:0").await?;
  let port = listener.local_addr()?.port();
  let mut cmd = Command::new("node");
  cmd.current_dir(std::env::current_dir()?);
  cmd.arg("lib/index.js".to_string());
  cmd.arg(port.to_string());
  cmd.env_clear();
  Ok(())
}
