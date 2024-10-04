// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use slog::{Drain, Logger};
use std::collections::HashMap;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use raft::prelude::*;
use raft::storage::MemStorage;

use slog::{info, o};

type ProposeCallback = Box<dyn Fn() + Send>;

enum Msg {
    Propose {
        id: u8,
        propose_call_back: ProposeCallback,
    },
    // Here we don't use Raft Message, so use dead_code to
    // avoid the compiler warning.
    #[allow(dead_code)]
    Raft(Message),
}

// A simple example about how to use the Raft library in Rust.
fn main() {
    // Create a storage for Raft, and here we just use a simple memory storage.
    // You need to build your own persistent storage in your production.
    // Please check the Storage trait in src/storage.rs to see how to implement one.
    let storage = MemStorage::new_with_conf_state(ConfState::from((vec![1], vec![])));

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .chan_size(4096)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();
    let logger = slog::Logger::root(drain, o!("tag" => format!("[{}]", 1)));

    // Create the configuration for the Raft node.
    let cfg = Config {
        // The unique ID for the Raft node.
        id: 1,
        // Election tick is for how long the follower may campaign again after
        // it doesn't receive any message from the leader.
        election_tick: 10,
        // Heartbeat tick is for how long the leader needs to send
        // a heartbeat to keep alive.
        heartbeat_tick: 3,
        // The max size limits the max size of each appended message. Mostly, 1 MB is enough.
        max_size_per_msg: 1024 * 1024 * 1024,
        // Max inflight msgs that the leader sends messages to follower without
        // receiving ACKs.
        max_inflight_msgs: 256,
        // The Raft applied index.
        // You need to save your applied index when you apply the committed Raft logs.
        applied: 0,
        ..Default::default()
    };

    // Create the Raft node.
    let mut raft_node = RawNode::new(&cfg, storage, &logger).unwrap();

    // 这个通道用来在 client 和 raft node 状态机之间传递消息使用的
    let (sender, receiver) = mpsc::channel();

    // Use another thread to propose a Raft request.
    send_propose(logger.clone(), sender);

    // Loop forever to drive the Raft.
    let mut now = Instant::now();
    let mut timeout = Duration::from_millis(100);

    // Use a HashMap to hold the `propose` callbacks.
    let mut callback_map = HashMap::new();

    loop {
        match receiver.recv_timeout(timeout) {
            // 接收到消息，则进行处理
            Ok(Msg::Propose {
                id,
                propose_call_back: cb,
            }) => {
                callback_map.insert(id, cb);
                raft_node.propose(vec![], vec![id]).unwrap();
            }
            Ok(Msg::Raft(message)) => raft_node.step(message).unwrap(),
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => return,
        }

        // 返回已经过去的时间数量
        let elapsed = now.elapsed();
        now = Instant::now();
        if elapsed >= timeout {
            timeout = Duration::from_millis(100);
            // We drive Raft every 100ms.
            raft_node.tick();
        } else {
            timeout -= elapsed;
        }

        // 尝试本次驱动是否有需要处理的消息
        on_ready(&mut raft_node, &mut callback_map);
    }
}

fn on_ready(raft_group: &mut RawNode<MemStorage>, callback_map: &mut HashMap<u8, ProposeCallback>) {
    if !raft_group.has_ready() {
        return;
    }
    let store = raft_group.raft.raft_log.store.clone();

    // Get the `Ready` with `RawNode::ready` interface.
    // 这一次需要批处理的数据
    let mut ready = raft_group.ready();

    let handle_messages = |messages: Vec<Message>| {
        for _msg in messages {
            // Send messages to other peers.
        }
    };

    if !ready.messages().is_empty() {
        // Send out the messages come from the node.
        handle_messages(ready.take_messages());
    }

    if !ready.snapshot().is_empty() {
        // This is a snapshot, we need to apply the snapshot at first.
        store.wl().apply_snapshot(ready.snapshot().clone()).unwrap();
    }

    let mut _last_apply_index = 0;
    let mut handle_committed_entries = |committed_entries: Vec<Entry>| {
        for entry in committed_entries {
            // Mostly, you need to save the last apply index to resume applying
            // after restart. Here we just ignore this because we use a Memory storage.
            _last_apply_index = entry.index;

            if entry.data.is_empty() {
                // Empty entry, when the peer becomes Leader it will send an empty entry.
                continue;
            }

            if entry.get_entry_type() == EntryType::EntryNormal {
                if let Some(cb) = callback_map.remove(entry.data.first().unwrap()) {
                    cb();
                }
            }

            // TODO: handle EntryConfChange
        }
    };

    // 处理已经能够被 apply 的消息，
    // apply 已经被多个节点处理成功的数据
    handle_committed_entries(ready.take_committed_entries());

    if !ready.entries().is_empty() {
        // Append entries to the Raft log.
        store.wl().append(ready.entries()).unwrap();
    }

    if let Some(hs) = ready.hs() {
        // Raft HardState changed, and we need to persist it.
        store.wl().set_hardstate(hs.clone());
    }

    if !ready.persisted_messages().is_empty() {
        // Send out the persisted messages come from the node.
        handle_messages(ready.take_persisted_messages());
    }

    // Advance the Raft.
    let mut light_rd = raft_group.advance(ready);
    // Update commit index.
    if let Some(commit) = light_rd.commit_index() {
        store.wl().mut_hard_state().set_commit(commit);
    }
    // Send out the messages.
    handle_messages(light_rd.take_messages());
    // Apply all committed entries.
    handle_committed_entries(light_rd.take_committed_entries());
    // Advance the apply index.
    raft_group.advance_apply();
}

fn send_propose(logger: Logger, sender: mpsc::Sender<Msg>) {
    thread::spawn(move || {
        // Wait some time and send the request to the Raft.
        thread::sleep(Duration::from_secs(10));

        let (s1, r1) = mpsc::channel::<u8>();

        info!(logger, "propose a request");

        // Send a command to the Raft, wait for the Raft to apply it
        // and get the result.
        sender
            .send(Msg::Propose {
                id: 1,
                propose_call_back: Box::new(move || {
                    s1.send(0).unwrap();
                }),
            })
            .unwrap();

        let n = r1.recv().unwrap();
        assert_eq!(n, 0);

        info!(logger, "receive the propose callback");
    });
}
