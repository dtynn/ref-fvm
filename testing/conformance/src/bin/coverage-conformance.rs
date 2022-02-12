use std::env;
use std::path::PathBuf;

use conformance_tests::vector::{MessageVector, Selector, Variant};
use conformance_tests::vm::{TestKernel, TestMachine};
use fvm::executor::{ApplyKind, DefaultExecutor, Executor};
use fvm::machine::Engine;
use fvm_shared::address::Protocol;
use fvm_shared::blockstore::MemoryBlockstore;
use fvm_shared::crypto::signature::SECP_SIG_LEN;
use fvm_shared::encoding::Cbor;
use fvm_shared::message::Message;
// use wasmtime::{Engine, Module};

pub fn main() {
    println!("good");

    // let binary = fvm::fvm_actor_market::wasm::WASM_BINARY_BLOATY.expect("get binary");
    // let binary = fvm::fvm_actor_market::wasm::WASM_BINARY.expect("get binary");
    // std::fs::write("actor_market.wasm", binary).expect("write binary");

    // let engine = Engine::default();
    // let module = Module::from_binary(&engine, binary).expect("load wasm");

    let vec_path = env::var("VECTOR")
        .map(|s| PathBuf::from(s))
        .expect("get vector path from env");

    let vector = MessageVector::from_file(&vec_path).expect("construct MessageVector");

    let skip = !vector.selector.as_ref().map_or(true, Selector::supported);
    if skip {
        println!("skipping because selector not supported");
        return;
    }

    let engine = Engine::default();

    let (bs, _) = async_std::task::block_on(vector.seed_blockstore()).unwrap();

    for variant in vector.preconditions.variants.iter().take(1) {
        let machine = TestMachine::new_for_vector(&vector, variant, bs.clone(), engine.clone());
        let mut exec: DefaultExecutor<TestKernel> = DefaultExecutor::new(machine);

        for m in vector.apply_messages.iter() {
            let msg = Message::unmarshal_cbor(&m.bytes).unwrap();

            // Execute the message.
            let mut raw_length = m.bytes.len();
            if msg.from.protocol() == Protocol::Secp256k1 {
                // 65 bytes signature + 1 byte type + 3 bytes for field info.
                raw_length += SECP_SIG_LEN + 4;
            }

            unsafe {
                exec.execute_message(msg, ApplyKind::Explicit, raw_length)
                    .expect("failed to execute a message");
            }
        }
    }

    println!("done");
}
