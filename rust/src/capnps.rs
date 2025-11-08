use super::message_capnp;
use crate::constants;

pub fn encode_unpacked() -> Vec<u8> {
    let arena = ::capnp::message::HeapAllocator::new().first_segment_words(512);

    let mut md_message = ::capnp::message::Builder::new(arena);
    let mut message_data = md_message.init_root::<message_capnp::message_data::Builder>();

    {
        let mut cast_add_body = message_data.reborrow().init_body().init_cast_add_body();
        cast_add_body.set_text(constants::SAMPLE_TEXT);

        let mut parent_cast_id = cast_add_body.reborrow().init_parent().init_cast_id();
        parent_cast_id.set_fid(&constants::SAMPLE_FID);
        parent_cast_id.set_ts_hash(&constants::SAMPLE_TS_HASH);
    }

    message_data.set_type(message_capnp::MessageType::CastAdd);
    message_data.set_timestamp(constants::SAMPLE_TIMESTAMP);
    message_data.set_fid(&constants::SAMPLE_FID);
    message_data.set_network(message_capnp::FarcasterNetwork::Devnet);

    let mut md_bytes = Vec::with_capacity(512);
    ::capnp::serialize::write_message(&mut md_bytes, &md_message).unwrap();

    let arena2 = ::capnp::message::HeapAllocator::new().first_segment_words(512);
    let mut final_message = ::capnp::message::Builder::new(arena2);
    let mut message_root = final_message.init_root::<message_capnp::message::Builder>();

    message_root.set_data(&md_bytes);
    message_root.set_hash(&constants::SAMPLE_HASH);
    message_root.set_hash_scheme(message_capnp::HashScheme::Blake3);
    message_root.set_signature(&constants::SAMPLE_SIGNATURE);
    message_root.set_signature_scheme(message_capnp::SignatureScheme::Ed25519);
    message_root.set_signer(&constants::SAMPLE_SIGNER);

    let mut bytes = Vec::with_capacity(512);
    ::capnp::serialize::write_message(&mut bytes, &final_message).unwrap();
    bytes
}

pub fn encode_packed() -> Vec<u8> {
    let arena = ::capnp::message::HeapAllocator::new().first_segment_words(512);

    let mut md_message = ::capnp::message::Builder::new(arena);
    let mut message_data = md_message.init_root::<message_capnp::message_data::Builder>();

    {
        let mut cast_add_body = message_data.reborrow().init_body().init_cast_add_body();
        cast_add_body.set_text(constants::SAMPLE_TEXT);

        let mut parent_cast_id = cast_add_body.reborrow().init_parent().init_cast_id();
        parent_cast_id.set_fid(&constants::SAMPLE_FID);
        parent_cast_id.set_ts_hash(&constants::SAMPLE_TS_HASH);
    }

    message_data.set_type(message_capnp::MessageType::CastAdd);
    message_data.set_timestamp(constants::SAMPLE_TIMESTAMP);
    message_data.set_fid(&constants::SAMPLE_FID);
    message_data.set_network(message_capnp::FarcasterNetwork::Devnet);

    let mut md_bytes = Vec::with_capacity(256);
    ::capnp::serialize_packed::write_message(&mut md_bytes, &md_message).unwrap();

    let arena2 = ::capnp::message::HeapAllocator::new().first_segment_words(512);
    let mut final_message = ::capnp::message::Builder::new(arena2);
    let mut message_root = final_message.init_root::<message_capnp::message::Builder>();

    message_root.set_data(&md_bytes);
    message_root.set_hash(&constants::SAMPLE_HASH);
    message_root.set_hash_scheme(message_capnp::HashScheme::Blake3);
    message_root.set_signature(&constants::SAMPLE_SIGNATURE);
    message_root.set_signature_scheme(message_capnp::SignatureScheme::Ed25519);
    message_root.set_signer(&constants::SAMPLE_SIGNER);

    let mut bytes = Vec::with_capacity(256);
    ::capnp::serialize_packed::write_message(&mut bytes, &final_message).unwrap();
    bytes
}

pub fn decode_unpacked(buf: &[u8]) {
    let message_reader =
        ::capnp::serialize::read_message(&mut &buf[..], ::capnp::message::ReaderOptions::new())
            .unwrap();

    let message = message_reader
        .get_root::<message_capnp::message::Reader>()
        .unwrap();

    let data = message.get_data().unwrap();

    let md_reader =
        ::capnp::serialize::read_message(&mut &data[..], ::capnp::message::ReaderOptions::new())
            .unwrap();

    let message_data = md_reader
        .get_root::<message_capnp::message_data::Reader>()
        .unwrap();

    let body = message_data.get_body();
    match body.which() {
        Ok(message_capnp::message_data::body::CastAddBody(cast_add_body_result)) => {
            let cast_add_body = cast_add_body_result.unwrap();
            let text = cast_add_body.get_text().unwrap();
            if text != constants::SAMPLE_TEXT {
                panic!("Unexpected decoded text");
            }
        }
        _ => panic!("Expected CastAddBody"),
    }
}

pub fn decode_packed(buf: &[u8]) {
    let message_reader = ::capnp::serialize_packed::read_message(
        &mut &buf[..],
        ::capnp::message::ReaderOptions::new(),
    )
    .unwrap();

    let message = message_reader
        .get_root::<message_capnp::message::Reader>()
        .unwrap();

    let data = message.get_data().unwrap();

    let md_reader = ::capnp::serialize_packed::read_message(
        &mut &data[..],
        ::capnp::message::ReaderOptions::new(),
    )
    .unwrap();

    let message_data = md_reader
        .get_root::<message_capnp::message_data::Reader>()
        .unwrap();

    let body = message_data.get_body();
    match body.which() {
        Ok(message_capnp::message_data::body::CastAddBody(cast_add_body_result)) => {
            let cast_add_body = cast_add_body_result.unwrap();
            let text = cast_add_body.get_text().unwrap();
            if text != constants::SAMPLE_TEXT {
                panic!("Unexpected decoded text");
            }
        }
        _ => panic!("Expected CastAddBody"),
    }
}
