pub mod bid;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::bid::BidRequest;
    use crate::pb::bid::Gender::{Female, Male};
    use prost::Message;
    use std::io::Cursor;

    #[test]
    fn test_marshal() {
        let req = &BidRequest {
            trace_id: uuid::Uuid::new_v4().to_string(),
            gender: Female.into(),
            device: None,
            tmax: 0,
            imp: vec![],
        };

        let mut buf = Vec::new();
        buf.reserve(req.encoded_len());
        req.encode(&mut buf).unwrap();

        println!("buf len {:?}", buf.len());
    }

    #[test]
    fn test_unmarshal() {
        let req = &BidRequest {
            trace_id: uuid::Uuid::new_v4().to_string(),
            gender: Male.into(),
            device: None,
            tmax: 200,
            imp: vec![],
        };

        let mut buf = Vec::new();
        buf.reserve(req.encoded_len());
        req.encode(&mut buf).unwrap();

        let decode1 = BidRequest::decode(&mut Cursor::new(buf));
        println!("decode1 {:?}", decode1);
    }
}
