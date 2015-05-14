use adapter::ChatAdapter;
use handler::MessageHandler;
use std::thread;
use std::sync::mpsc::Select;

pub struct Chatbot {
    name: String,
    adapters: Vec<Box<ChatAdapter>>,
    handlers: Vec<Box<MessageHandler>>
}

impl Chatbot {
    pub fn new() -> Chatbot {
        Chatbot {
            name: "computer".to_owned(),
            adapters: Vec::new(),
            handlers: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn add_adapter<T: ChatAdapter + 'static>(&mut self, adapter: Box<T>) {
        self.adapters.push(adapter)
    }

    pub fn add_handler<T: MessageHandler + 'static>(&mut self, handler: Box<T>) {
        self.handlers.push(handler)
    }

    pub fn run(&self) {

        let select = Select::new();
        let handles = HashMap::new();

        self.adapters.map(|&adapter| {
            let (send, recv) = *adapter.process_events();
            let handle = select.handle(&recv);
            unsafe { handle.add() };
            handles.insert(handle.id(), (send, recv));
        });

        loop {
            let id = select.wait();
            let (send, recv) = handles.get(id);

            match handle.recv().unwrap() {
                _ => panic!("someone forgot to write this part of the program")
            }
        }

        println!("Chatbot shutting down");
    }
}

#[cfg(test)]
mod tests {
    use chatbot::Chatbot;
    use adapter::CliAdapter;
    use handler::EchoHandler;

    #[test]
    fn test_create_chatbot() {
        let bot = Chatbot::new();
        assert_eq!(bot.get_name(), "computer");
    }

    #[test]
    fn test_chatbot_add_adapter() {
        let mut bot = Chatbot::new();
        let cli = Box::new(CliAdapter::new());
        bot.add_adapter(cli);
    }

    #[test]
    fn test_chatbot_add_handler() {
        let mut bot = Chatbot::new();
        let handler = Box::new(EchoHandler::new());
        bot.add_handler(handler);
    }
}
