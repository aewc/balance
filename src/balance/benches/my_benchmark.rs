use candid::{Decode, Encode};
use criterion::{criterion_group, criterion_main, Criterion};
use garcon::Delay;
use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal,
    identity::BasicIdentity, Agent,
};
use tokio::runtime::Runtime;

fn local_agent() -> Agent {
    let identity =
        BasicIdentity::from_pem_file("/Users/flyq/.config/dfx/identity/icp4/identity.pem")
            .expect("Could not read the key pair.");
    Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create("http://127.0.0.1:8000")
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(identity)
        .build()
        .expect("Failed to build the Agent")
}
#[allow(dead_code)]
fn mainnet_agent() -> Agent {
    let identity =
        BasicIdentity::from_pem_file("/Users/flyq/.config/dfx/identity/icp4/identity.pem")
            .expect("Could not read the key pair.");
    Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create("https://ic0.app")
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(identity)
        .build()
        .expect("Failed to build the Agent")
}

fn waiter() -> Delay {
    garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build()
}

async fn write_map_stable_1() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_map_stable",
        )
        .with_arg(&Encode!(&0u64, &100_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_map_stable_2() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_map_stable",
        )
        .with_arg(&Encode!(&100_000u64, &200_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_map_wasm_heap_1() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_map_wasm_heap",
        )
        .with_arg(&Encode!(&0u64, &100_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_map_wasm_heap_2() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_map_wasm_heap",
        )
        .with_arg(&Encode!(&100_000u64, &200_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_vec_stable_1() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_vec_stable",
        )
        .with_arg(&Encode!(&0u64, &100_000_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_vec_stable_2() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_vec_stable",
        )
        .with_arg(&Encode!(&100_000_000u64, &100_000_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_vec_wasm_heap_1() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_vec_wasm_heap",
        )
        .with_arg(&Encode!(&0u64, &100_000_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn write_vec_wasm_heap_2() {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .update(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "write_vec_wasm_heap",
        )
        .with_arg(&Encode!(&100_000_000u64, &100_000_000u64).unwrap())
        .call_and_wait(waiter())
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), ()).expect("decode error");
}

async fn read_map_stable_1() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_map_stable",
        )
        .with_arg(&Encode!(&0u64, &100_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_map_stable_2() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_map_stable",
        )
        .with_arg(&Encode!(&100_000u64, &200_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_map_wasm_heap_1() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_map_wasm_heap",
        )
        .with_arg(&Encode!(&0u64, &100_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_map_wasm_heap_2() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_map_wasm_heap",
        )
        .with_arg(&Encode!(&100_000u64, &200_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_vec_stable_1() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_vec_stable",
        )
        .with_arg(&Encode!(&0u64, &100_000_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_vec_stable_2() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_vec_stable",
        )
        .with_arg(&Encode!(&100_000_000u64, &100_000_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_vec_wasm_heap_1() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_vec_wasm_heap",
        )
        .with_arg(&Encode!(&0u64, &100_000_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

async fn read_vec_wasm_heap_2() -> u64 {
    let agent = local_agent();
    agent.fetch_root_key().await.expect("fetch_root_key err");

    let response = agent
        .query(
            &Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            "read_vec_wasm_heap",
        )
        .with_arg(&Encode!(&100_000_000u64, &100_000_000u64).unwrap())
        .call()
        .await
        .expect("faile to call");
    Decode!(response.as_slice(), u64).expect("decode error")
}

fn criterion_benchmark(c1: &mut Criterion) {
    c1.bench_function("write_map_stable 0-100_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_map_stable_1());
        })
    });
    c1.bench_function("write_map_stable 100_000-200_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_map_stable_2());
        })
    });
    c1.bench_function("write_map_wasm_heap 0-100_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_map_wasm_heap_1());
        })
    });
    c1.bench_function("write_map_wasm_heap 100_000-200_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_map_wasm_heap_2());
        })
    });
    c1.bench_function("write_vec_stable 0-100_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_vec_stable_1());
        })
    });
    c1.bench_function("write_vec_stable 100_000_000-200_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_vec_stable_2());
        })
    });
    c1.bench_function("write_vec_wasm_heap 0-100_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_vec_wasm_heap_1());
        })
    });
    c1.bench_function("write_vec_wasm_heap 100_000_000-2_000_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(write_vec_wasm_heap_2());
        })
    });
    c1.bench_function("read_map_stable 0-100_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_map_stable_1());
        })
    });
    c1.bench_function("read_map_stable 100_000-200_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_map_stable_2());
        })
    });
    c1.bench_function("read_map_wasm_heap 0-100_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_map_wasm_heap_1());
        })
    });
    c1.bench_function("read_map_wasm_heap 100_000-200_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_map_wasm_heap_2());
        })
    });
    c1.bench_function("read_vec_stable 0-100_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_vec_stable_1());
        })
    });
    c1.bench_function("read_vec_stable 100_000_000-200_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_vec_stable_2());
        })
    });
    c1.bench_function("read_vec_wasm_heap 0-100_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_vec_wasm_heap_1());
        })
    });
    c1.bench_function("read_vec_wasm_heap 100_000_000-200_000_000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(read_vec_wasm_heap_2());
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
