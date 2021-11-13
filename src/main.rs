use sp_keyring::AccountKeyring;
use substrate_api_client::{rpc::WsRpcClient, Api};
fn main() {
    let url_remote = "wss://khala.api.onfinality.io/public-ws";
    let url_local_chao = "ws://192.168.7.10:9944";
    let url_local_node = "ws://192.168.7.11:9944";

    let num_remote = get_number(url_remote);
    let num_local_chao = get_number(url_local_chao);
    let num_local_node = get_number(url_local_node);

    println!(
        "Remote : kha={}, ksm={} \nChao   : kha={}, ksm={} \nNode   : kha={}, ksm={} ",
        num_remote.0,
        num_remote.1,
        num_local_chao.0,
        num_local_chao.1,
        num_local_node.0,
        num_local_node.1
    );
}

fn get_number(url: &str) -> (u32, u32) {
    let signer = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(url);
    let api = Api::new(client)
        .map(|api| api.set_signer(signer.clone()))
        .unwrap();
    // let meta = api.get_metadata().unwrap();
    // println!("Metadata:\n {}", Metadata::pretty_format(&meta).unwrap());
    let kh_nouce: Result<Option<u32>, _> = api.get_storage_value("System", "Number", None);
    let km_nouce: Result<Option<u32>, _> =
        api.get_storage_value("ParachainSystem", "HrmpWatermark", None);

    match (kh_nouce, km_nouce) {
        (Ok(Some(kh_num)), Ok(Some(km_num))) => return (kh_num, km_num),
        (Err(_),Err(_)) => return (2,2),
        (_, _) => return (0, 0),
    }
}
