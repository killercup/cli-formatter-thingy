#[macro_use] extern crate trender;
use trender::{Bold, Color, CliOutput, CliColor, HumanCliOutput};

struct Message {
    code: i32,
    message: String,
}

impl HumanCliOutput for Message {
    fn render(&self, stream: &mut CliOutput) -> Result<(), trender::Error> {
        render!(stream,
            Bold()[ Color(fg={ CliColor::Red }) [ { self.code } ] ]
            {" "}
            { self.message }
        )
    }
}

fn main() {
    let msg = Message { code: 42, message: "Foobar".into() };
    msg.render(&mut trender::output_stream()).unwrap();
}
