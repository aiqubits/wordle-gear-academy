#![no_std]
use gstd::{msg, prelude::*, ActorId, MessageId};
use wordle_game_io::*;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
struct Session {
    target_program_id: ActorId,               // target program address
    msg_id_to_actor_id: (MessageId, ActorId), // tuple containing the identifier of a message sent to a Target program and the Id of a User initiating the action
}

static mut SESSION: Option<Session> = None;

#[no_mangle]
extern "C" fn init() {
    // Receives and stores the Wordle program's address.
    let target_program_id = msg::load().expect("Unable to decode Init");
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            msg_id_to_actor_id: (MessageId::zero(), ActorId::zero()),
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    // Manages actions: StartGame, CheckWord, CheckGameStatus. Let's examine the functionality of each action:
    // StartGame
    // The program checks if a game already exists for the user;
    // It sends a "StartGame" message to the Wordle program;
    // Utilizes the exec::wait() or exec::wait_for() function to await a response;
    // Sends a delayed message with action CheckGameStatus to monitor the game's progress (its logic will be described below);
    // A reply is sent to notify the user that the game has beeen successfully started.

    // CheckWord
    // Ensures that a game exists and is in the correct status;
    // Validates that the submitted word length is five and is in lowercase;
    // Sends a "CheckWord" message to the Wordle program;
    // Utilizes the exec::wait() or exec::wait_for() function to await a reply;
    // Sends a reply to notify the user that the move was successful.

    // CheckGameStatus
    // The game should have a time limit from its start, so a delayed message is sent to check the game status. If the game is not finished within the specified time limit, it ends the game by transitioning it to the desired status. Specify a delay equal to 200 blocks (10 minutes) for the delayed message.

    let action: WordleAction = msg::load().expect("Unable to decode ");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let msg_id =
        msg::send(session.target_program_id, action, 0).expect("Error in sending a message");
    session.msg_id_to_actor_id = (msg_id, msg::source());
    msg::reply(
        WordleEvent::GameStarted {
            user: msg::source(),
        },
        0,
    )
    .expect("Error in sending a reply");
}

#[no_mangle]
extern "C" fn handle_reply() {
    // Processes reply messages and updates the game status based on responses from the Wordle program.
    // Receives reply messages.
    // Utilizes msg::reply_to() to determine the message identifier, i.e., which message was replied to.

    // Processes and stores the result depending on the reply:
    // If a GameStarted response is received, it updates the game status to indicate that the game was successfully started.
    // If a WordChecked response is received, it saves the response, increments the number of tries, and checks if the word was guessed.
    // If the word has been guessed, it switches the game status to GameOver(Win).
    //If all attempts are used up and the word is not guessed, it switches the game status to GameOver(Lose).

    // Calls wake() with the identifier of the received message to acknowledge the response.

    let reply_message_id = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let (msg_id, actor) = session.msg_id_to_actor_id;
    if reply_message_id == msg_id {
        let reply: WordleEvent = msg::load().expect("Unable to decode ");
        msg::send(actor, reply, 0).expect("Error in sending a message");
    }
}

#[no_mangle]
pub extern "C" fn state() {
    // It is necessary to implement the state() function in order to get all the information about the game.
    let wordle_game = unsafe { SESSION.take().expect("Error in taking current state") };

    // Checks input data for validness

    // returns the `GameState` structure using the `msg::reply` function
    msg::reply(wordle_game, 0).expect("Failed to reply state");
}

#[cfg(test)]
mod tests {
    use gstd::*;

    #[test]
    fn test_check_user_input() {}
}
