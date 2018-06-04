extern crate termcolor;
extern crate atty;

pub use termcolor::StandardStream as CliOutput;
pub use termcolor::Color as CliColor;

// Re-exports for macro only
#[doc(hidden)]
pub use termcolor::ColorSpec;
#[doc(hidden)]
pub use termcolor::WriteColor;

pub type Error = Box<std::error::Error>;

pub trait HumanCliOutput {
    fn render(&self, stream: &mut CliOutput) -> Result<(), Error>;
}

mod tags;
pub use tags::*;

pub fn output_stream() -> termcolor::StandardStream {
    let color_choice = if atty::is(atty::Stream::Stderr) {
        termcolor::ColorChoice::Auto
    } else {
        termcolor::ColorChoice::Never
    };
    termcolor::StandardStream::stderr(color_choice)
}

#[macro_export]
macro_rules! render {
    ($s:ident, $($tokens:tt)+) => {
        #[allow(unused_mut)]
        {
            use std::io::Write;
            use $crate::WriteColor;

            render!(@start $s $($tokens)*);
            Ok(())
        }
    };
    // end of tokens
    (@start $stream:ident) => {};
    (@start $stream:ident $tag:ident ( $($attrs:tt)* ) [ $($tokens:tt)* ] $($rest:tt)*) => {
        let mut tag = $tag::new();
        render!(@attrs tag $($attrs)* );

        let mut color_spec = $crate::ColorSpec::new();
        tag.update(&mut color_spec)?;
        $stream.set_color(&color_spec)?;

        render!(@start $stream $($tokens)*);

        $stream.reset()?;

        render!(@start $stream $($rest)*);
    };
    (@start $stream:ident { $expr:expr } $($rest:tt)*) => {
        write!($stream, "{}", $expr)?;
        render!(@start $stream $($rest)*);
    };
    (@attrs $tag:ident $attr:ident = { $expr:expr } $($rest:tt)*) => {
        $tag.set(stringify!($attr), $expr);
        render!(@attrs $tag $($rest)*);
    };
    (@attrs tag) => {};
}
