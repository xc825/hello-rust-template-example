/* A Marco Polo Game

If the name Marco, the program should respod with Polo.
Otherwise, the program should respond with the message "What's your name?".
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}
