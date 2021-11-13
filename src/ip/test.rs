use crate::ip::IpReturn;
use mockito::mock;
use speculate::speculate;
use tokio_test::block_on;

speculate! {
    describe "get_ip" {
        before {
            let _m = mock("GET", "/").with_body(serde_json::to_string(&IpReturn {
                ip: "192.168.1.1".to_string(),
                country: "real place".to_string(),
                cc: "RP".to_string()
            }).unwrap()).create();
        }

        it "returns the ip" {
            assert_eq!(Some([192, 168, 1, 1]), block_on(crate::ip::get_ip()));
        }
    }
}
