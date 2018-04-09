macro_rules! send_client_host_message {
    ($message:expr, $byte_amount:expr, $tx:expr, $threshold:expr) => {
        let mut buffer = [0; $byte_amount];
        let byte_amount = $message.encode(&mut buffer).unwrap();

        $tx.claim_mut($threshold, |tx, _| {
            for byte in buffer[..byte_amount].iter() {
                block!(tx.write(*byte)).unwrap()
            }
        })
    }
}
