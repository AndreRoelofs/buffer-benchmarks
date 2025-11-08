use criterion::{Criterion, criterion_group, criterion_main};

use buffer_benchmarks::{capnps, flatbuffers, prost, protobuf};

fn bench_protobuf(c: &mut Criterion) {
    c.bench_function("encode_protobuf", |b| b.iter(protobuf::encode));
    let buf = protobuf::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_protobuf", |b| b.iter(|| protobuf::decode(&buf)));
}

fn bench_prost(c: &mut Criterion) {
    c.bench_function("encode_prost", |b| b.iter(prost::encode));
    let buf = prost::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_prost", |b| b.iter(|| prost::decode(&buf)));
}

fn bench_flatbuffers(c: &mut Criterion) {
    c.bench_function("encode_flatbuffers", |b| b.iter(flatbuffers::encode));
    let buf = flatbuffers::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_flatbuffers", |b| {
        b.iter(|| flatbuffers::decode(&buf))
    });
}

fn bench_capnp_unpacked(c: &mut Criterion) {
    c.bench_function("encode_capnp_unpacked", |b| b.iter(capnps::encode_unpacked));
    let buf = capnps::encode_unpacked();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_capnp_unpacked", |b| {
        b.iter(|| capnps::decode_unpacked(&buf))
    });
}

fn bench_capnp_packed(c: &mut Criterion) {
    c.bench_function("encode_capnp_packed", |b| b.iter(capnps::encode_packed));
    let buf = capnps::encode_packed();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_capnp_packed", |b| {
        b.iter(|| capnps::decode_packed(&buf))
    });
}

criterion_group!(
    benches,
    bench_protobuf,
    bench_prost,
    bench_flatbuffers,
    bench_capnp_unpacked,
    bench_capnp_packed
);
criterion_main!(benches);
