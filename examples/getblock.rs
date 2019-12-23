use nem2::block;

fn main() {
    let client = block::BlockHttp::new("http://localhost:3000");

    // let resp = client.get_block_by_height(1);
    // let resp = client.get_block_transactions(1);
    // let resp = client.get_blocks_by_height_with_limit(1, 25);
    let resp = client.get_merkle_transaction(1, "a");
    match resp {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("error: {:?}", e),
    }
}
