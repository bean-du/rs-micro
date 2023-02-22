use async_trait::async_trait;
use etcd_client::PutOptions;
use std::time::Duration;
use super::*;


static REGISTRY_KEY_PREFIX: &'static str = "/micro/registry/";

#[async_trait]
pub trait Registry {
    async fn register(&mut self, service: &Service) -> IResult<()>;
    fn deregister(&self) -> IResult<()>;
    fn get_service(&self, name: String) -> IResult<Vec<Service>>;
    fn string(&self) -> String;
}


pub struct Etcd {
    client: etcd_client::Client,
    addr: String,
    registered: HashMap<String, String>,
}

impl Etcd {
    pub async fn new(addr: String) -> IResult<Self> {
        let client = etcd_client::Client::connect([addr.clone()], None).await?;
        let e = Etcd {
            client,
            addr,
            registered: HashMap::new(),
        };
        Ok(e)
    }

    pub async fn register_node(&mut self, s: &Service, node: &Node) -> IResult<()> {
        let grant = self.client.lease_grant(10, None).await?;
        let lease_id = grant.id();

        let key = format!("{}{}/{}", REGISTRY_KEY_PREFIX, s.name.clone(), node.id.clone());
        let json = s.clone().to_json_string()?;


        let options = PutOptions::new().with_lease(lease_id);
        self.client.put(key.clone(), json, Some(options)).await?;

        let (mut keeper, _) = self.client.lease_keep_alive(lease_id).await?;
        self.registered.insert(key, lease_id.to_string());

        tokio::spawn(async move {
            loop {
                match keeper.keep_alive().await {
                    Ok(_) => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        return;
                    }
                }

            }
        });
        Ok(())
    }
}

#[async_trait]
impl Registry for Etcd {
    async fn register(&mut self, service: &Service) -> IResult<()> {
        if service.nodes.len() == 0 {
            return Err(Box::try_from("Require at least one node".to_string()).unwrap());
        }

        for node in service.nodes.iter() {
            let res = self.register_node(service, &node).await;
            match res {
                Ok(_) => println!("{}", "register success".to_string()),
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }

    fn deregister(&self) -> IResult<()> {
        Ok(())
    }
    fn get_service(&self, name: String) -> IResult<Vec<Service>> {
        Ok(Vec::new())
    }
    fn string(&self) -> String {
        String::from("etcd")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_register() {
        let node = Node::new("test-2312sqedq21312eqw".to_string(), "127.0.0.1:50051".to_string(), HashMap::new());
        let service = Service::new("test_register".to_string(), "1.1.1".to_string(), HashMap::new(), Vec::new(), vec![node]);

        let mut etcd_resp = Etcd::new("127.0.0.1:2379".to_string()).await;
        match etcd_resp {
            Ok(mut etcd) => {
                let res = etcd.register(&service).await;
                match res {
                    Ok(_) => println!("{}", "ok"),
                    Err(e) => println!("{}", e)
                }
            }
            Err(e) => println!("{}", e)
        }
    }
}