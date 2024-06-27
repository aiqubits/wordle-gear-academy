use gstd::ActorId;
use gtest::{Log, Program, System};
use wordle_game_io::{WordleAction, WordleEvent};

const USER: u64 = 3;
const TARGET_PROGRAM_ADDRESS: u64 = 2;

#[test]
fn success_test() {
    // Create a new testing environment.
    let system = System::new();

    // Get proxy program of the root crate with provided system.
    let proxy_program = Program::current(&system);
    // Get target program
    let target_program = Program::from_file(
        &system,
        "target/wasm32-unknown-unknown/release/session_proxy.opt.wasm",
    );
    // The target program is initialized with an empty payload message
    let result = target_program.send_bytes(USER, []);
    assert!(!result.main_failed());
    let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
    // The proxy program is initialized using target_program in the payload message
    let res = proxy_program.send(USER, target_program_address);
    assert!(!res.main_failed());

    // Send with the message we want to receive back
    let result = proxy_program.send(
        USER,
        WordleAction::StartGame {
            user: target_program_address,
        },
    );
    assert!(!result.main_failed());

    // check that the proxy message has arrived,
    // which means that the message was successfully sent to the target program
    let log = Log::builder()
        .source(1)
        .dest(3)
        .payload(WordleEvent::GameStarted {
            user: target_program_address,
        });
    assert!(result.contains(&log));

    // check that the target message has arrived at the mailbox,
    // which means that a reply has been received.
    let mailbox = system.get_mailbox(USER);
    let log = Log::builder()
        .source(1)
        .dest(3)
        .payload(WordleEvent::GameStarted {
            user: target_program_address,
        });

    assert!(mailbox.contains(&log));
}
