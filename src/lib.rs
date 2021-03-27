// Colo[dot]rs
// Powerful, easy-to-use Rust implementation of Colorized library.
//
// I want to..
//
// 1: directly make my texts colored:
//     function definition:
//        dot::color()
//
//    red();
//    green();
//    light_red();
//    ...
//
// Tip: Don't forget to add reset()
//
// 2: change type (Bold, Dim etc.) and color of my text:
//     macro definition:
//        colodot!(DotTypes, DotColors, Expression, Reset)
//
//     colodot!(DotTypes::Bold, DotColors::Red,        "Hello, world!"  , false);
//     colodot!(DotTypes::Light,DotColors::LightGreen, "Hello, world!\n", false);
//     colodot!(DotTypes::Bold, DotColors::Blue,       "Hello, world!\n", true);
//     ...
//
// 3: change type and color (with TrueColor support) of my text:
//     work-in-progress!
//
//
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

// TODO:
//
// Add RGB(u8, u8, u8) for TrueColor support.
// Add RESET
// Add tests, examples.

#![recursion_limit="256"]

#[macro_use]
extern crate lazy_static;

pub mod dot {
    #[derive(Copy, Clone)]
    pub enum DotColors {
        Reset                     ,
        Black                = 30 ,
        Red                       ,
        Green                     ,
        Yellow                    ,
        Blue                      ,
        Magenta                   ,
        Cyan                      ,
        White                     , // 37
        Default              = 39 ,

        // Light colors
        LightBlack           = 90 ,
        LightRed                  ,
        LightGreen                ,
        LightYellow               ,
        LightBlue                 ,
        LightMagenta              ,
        LightCyan                 ,
        LightWhite                , // 97
        LightDefault         = 99 ,

        // Background colors
        BackgroundBlack      = 40 ,
        BackgroundRed             ,
        BackgroundGreen           ,
        BackgroundYellow          ,
        BackgroundBlue            ,
        BackgroundMagenta         ,
        BackgroundCyan            ,
        BackgroundWhite           , // 47
        BackgroundDefault    = 49 ,

        // Background light colors
        BackgroundLightBlack = 100,
        BackgroundLightRed        ,
        BackgroundLightGreen      ,
        BackgroundLightYellow     ,
        BackgroundLightBlue       ,
        BackgroundLightMagenta    ,
        BackgroundLightCyan       ,
        BackgroundLightWhite      , // 107
        BackgroundLightDefault=109
    }

    pub struct DotTrueColor {
        pub r: u32,
        pub g: u32,
        pub b: u32
    }

    pub enum DotTypes {
        Light     ,
        Bold      ,
        Dim       ,
        Italic    ,
        Underlined,
        Blink     ,
        RapidBlink,
        Reverse   ,
        Hidden     // 8
    }

    macro_rules! makedot {
        ($type:expr, $color:expr) => {
            format!("\x1b[{};{}m", stringify!($type as u8), stringify!($color as u8))
        };
    }


    pub mod dot_defineds {
        lazy_static! {
            // Colors
            pub static ref BLACK_COLOR  : String = makedot!(DotTypes::Light, DotColors::Black  );
            pub static ref RED_COLOR    : String = makedot!(DotTypes::Light, DotColors::Red    );
            pub static ref GREEN_COLOR  : String = makedot!(DotTypes::Light, DotColors::Green  );
            pub static ref YELLOW_COLOR : String = makedot!(DotTypes::Light, DotColors::Yellow );
            pub static ref BLUE_COLOR   : String = makedot!(DotTypes::Light, DotColors::Blue   );
            pub static ref MAGENTA_COLOR: String = makedot!(DotTypes::Light, DotColors::Magenta);
            pub static ref CYAN_COLOR   : String = makedot!(DotTypes::Light, DotColors::Cyan   );
            pub static ref WHITE_COLOR  : String = makedot!(DotTypes::Light, DotColors::White  );
            pub static ref DEFAULT_COLOR: String = makedot!(DotTypes::Light, DotColors::Default);

            // Light colors
            pub static ref LIGHT_BLACK_COLOR  : String = makedot!(DotTypes::Light, DotColors::LightBlack  );
            pub static ref LIGHT_RED_COLOR    : String = makedot!(DotTypes::Light, DotColors::LightRed    );
            pub static ref LIGHT_GREEN_COLOR  : String = makedot!(DotTypes::Light, DotColors::LightGreen  );
            pub static ref LIGHT_YELLOW_COLOR : String = makedot!(DotTypes::Light, DotColors::LightYellow );
            pub static ref LIGHT_BLUE_COLOR   : String = makedot!(DotTypes::Light, DotColors::LightBlue   );
            pub static ref LIGHT_MAGENTA_COLOR: String = makedot!(DotTypes::Light, DotColors::LightMagenta);
            pub static ref LIGHT_CYAN_COLOR   : String = makedot!(DotTypes::Light, DotColors::LightCyan   );
            pub static ref LIGHT_WHITE_COLOR  : String = makedot!(DotTypes::Light, DotColors::LightWhite  );
            pub static ref LIGHT_DEFAULT_COLOR: String = makedot!(DotTypes::Light, DotColors::LightDefault);

            // Background colors
            pub static ref BG_BLACK_COLOR     : String = makedot!(DotTypes::Light, DotColors::BackgroundBlack  );
            pub static ref BG_RED_COLOR       : String = makedot!(DotTypes::Light, DotColors::BackgroundRed    );
            pub static ref BG_GREEN_COLOR     : String = makedot!(DotTypes::Light, DotColors::BackgroundGreen  );
            pub static ref BG_YELLOW_COLOR    : String = makedot!(DotTypes::Light, DotColors::BackgroundYellow );
            pub static ref BG_BLUE_COLOR      : String = makedot!(DotTypes::Light, DotColors::BackgroundBlue   );
            pub static ref BG_MAGENTA_COLOR   : String = makedot!(DotTypes::Light, DotColors::BackgroundMagenta);
            pub static ref BG_CYAN_COLOR      : String = makedot!(DotTypes::Light, DotColors::BackgroundCyan   );
            pub static ref BG_WHITE_COLOR     : String = makedot!(DotTypes::Light, DotColors::BackgroundWhite  );
            pub static ref BG_DEFAULT_COLOR   : String = makedot!(DotTypes::Light, DotColors::BackgroundDefault);

            // Background light colors
            pub static ref BG_LIGHT_BLACK_COLOR  : String = makedot!(DotTypes::Light, DotColors::BackgroundLightBlack  );
            pub static ref BG_LIGHT_RED_COLOR    : String = makedot!(DotTypes::Light, DotColors::BackgroundLightRed    );
            pub static ref BG_LIGHT_GREEN_COLOR  : String = makedot!(DotTypes::Light, DotColors::BackgroundLightGreen  );
            pub static ref BG_LIGHT_YELLOW_COLOR : String = makedot!(DotTypes::Light, DotColors::BackgroundLightYellow );
            pub static ref BG_LIGHT_BLUE_COLOR   : String = makedot!(DotTypes::Light, DotColors::BackgroundLightBlue   );
            pub static ref BG_LIGHT_MAGENTA_COLOR: String = makedot!(DotTypes::Light, DotColors::BackgroundLightMagenta);
            pub static ref BG_LIGHT_CYAN_COLOR   : String = makedot!(DotTypes::Light, DotColors::BackgroundLightCyan   );
            pub static ref BG_LIGHT_WHITE_COLOR  : String = makedot!(DotTypes::Light, DotColors::BackgroundLightWhite  );
            pub static ref BG_LIGHT_DEFAULT_COLOR: String = makedot!(DotTypes::Light, DotColors::BackgroundLightDefault);

            // Bold Colors
            pub static ref BOLD_BLACK_COLOR  : String = makedot!(DotTypes::Bold, DotColors::Black  );
            pub static ref BOLD_RED_COLOR    : String = makedot!(DotTypes::Bold, DotColors::Red    );
            pub static ref BOLD_GREEN_COLOR  : String = makedot!(DotTypes::Bold, DotColors::Green  );
            pub static ref BOLD_YELLOW_COLOR : String = makedot!(DotTypes::Bold, DotColors::Yellow );
            pub static ref BOLD_BLUE_COLOR   : String = makedot!(DotTypes::Bold, DotColors::Blue   );
            pub static ref BOLD_MAGENTA_COLOR: String = makedot!(DotTypes::Bold, DotColors::Magenta);
            pub static ref BOLD_CYAN_COLOR   : String = makedot!(DotTypes::Bold, DotColors::Cyan   );
            pub static ref BOLD_WHITE_COLOR  : String = makedot!(DotTypes::Bold, DotColors::White  );
            pub static ref BOLD_DEFAULT_COLOR: String = makedot!(DotTypes::Bold, DotColors::Default);

            // Light colors
            pub static ref BOLD_LIGHT_BLACK_COLOR  : String = makedot!(DotTypes::Bold, DotColors::LightBlack  );
            pub static ref BOLD_LIGHT_RED_COLOR    : String = makedot!(DotTypes::Bold, DotColors::LightRed    );
            pub static ref BOLD_LIGHT_GREEN_COLOR  : String = makedot!(DotTypes::Bold, DotColors::LightGreen  );
            pub static ref BOLD_LIGHT_YELLOW_COLOR : String = makedot!(DotTypes::Bold, DotColors::LightYellow );
            pub static ref BOLD_LIGHT_BLUE_COLOR   : String = makedot!(DotTypes::Bold, DotColors::LightBlue   );
            pub static ref BOLD_LIGHT_MAGENTA_COLOR: String = makedot!(DotTypes::Bold, DotColors::LightMagenta);
            pub static ref BOLD_LIGHT_CYAN_COLOR   : String = makedot!(DotTypes::Bold, DotColors::LightCyan   );
            pub static ref BOLD_LIGHT_WHITE_COLOR  : String = makedot!(DotTypes::Bold, DotColors::LightWhite  );
            pub static ref BOLD_LIGHT_DEFAULT_COLOR: String = makedot!(DotTypes::Bold, DotColors::LightDefault);

            // Bold Background colors
            pub static ref BOLD_BG_BLACK_COLOR     : String = makedot!(DotTypes::Bold, DotColors::BackgroundBlack  );
            pub static ref BOLD_BG_RED_COLOR       : String = makedot!(DotTypes::Bold, DotColors::BackgroundRed    );
            pub static ref BOLD_BG_GREEN_COLOR     : String = makedot!(DotTypes::Bold, DotColors::BackgroundGreen  );
            pub static ref BOLD_BG_YELLOW_COLOR    : String = makedot!(DotTypes::Bold, DotColors::BackgroundYellow );
            pub static ref BOLD_BG_BLUE_COLOR      : String = makedot!(DotTypes::Bold, DotColors::BackgroundBlue   );
            pub static ref BOLD_BG_MAGENTA_COLOR   : String = makedot!(DotTypes::Bold, DotColors::BackgroundMagenta);
            pub static ref BOLD_BG_CYAN_COLOR      : String = makedot!(DotTypes::Bold, DotColors::BackgroundCyan   );

            pub static ref BOLD_BG_WHITE_COLOR     : String = makedot!(DotTypes::Bold, DotColors::BackgroundWhite  );
            pub static ref BOLD_BG_DEFAULT_COLOR   : String = makedot!(DotTypes::Bold, DotColors::BackgroundDefault);

            // Background light colors
            pub static ref BOLD_BG_LIGHT_BLACK_COLOR  : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightBlack  );
            pub static ref BOLD_BG_LIGHT_RED_COLOR    : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightRed    );
            pub static ref BOLD_BG_LIGHT_GREEN_COLOR  : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightGreen  );
            pub static ref BOLD_BG_LIGHT_YELLOW_COLOR : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightYellow );
            pub static ref BOLD_BG_LIGHT_BLUE_COLOR   : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightBlue   );
            pub static ref BOLD_BG_LIGHT_MAGENTA_COLOR: String = makedot!(DotTypes::Bold, DotColors::BackgroundLightMagenta);
            pub static ref BOLD_BG_LIGHT_CYAN_COLOR   : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightCyan   );
            pub static ref BOLD_BG_LIGHT_WHITE_COLOR  : String = makedot!(DotTypes::Bold, DotColors::BackgroundLightWhite  );
            pub static ref BOLD_BG_LIGHT_DEFAULT_COLOR: String = makedot!(DotTypes::Bold, DotColors::BackgroundLightDefault);
        }
    }

    macro_rules! printdot {
        ($type:expr, $color:expr) => {
            print!("\x1b[{};{}m", $type as u8, $color as u8);
        };
    }

    pub fn black        () { printdot!(DotTypes::Light, DotColors::Black       ); }
    pub fn red          () { printdot!(DotTypes::Light, DotColors::Red         ); }
    pub fn green        () { printdot!(DotTypes::Light, DotColors::Green       ); }
    pub fn yellow       () { printdot!(DotTypes::Light, DotColors::Yellow      ); }
    pub fn blue         () { printdot!(DotTypes::Light, DotColors::Blue        ); }
    pub fn magenta      () { printdot!(DotTypes::Light, DotColors::Magenta     ); }
    pub fn cyan         () { printdot!(DotTypes::Light, DotColors::Cyan        ); }
    pub fn white        () { printdot!(DotTypes::Light, DotColors::White       ); }
    pub fn default      () { printdot!(DotTypes::Light, DotColors::Default     ); }

    pub fn light_black  () { printdot!(DotTypes::Light, DotColors::LightBlack  ); }
    pub fn light_red    () { printdot!(DotTypes::Light, DotColors::LightRed    ); }
    pub fn light_green  () { printdot!(DotTypes::Light, DotColors::LightGreen  ); }
    pub fn light_yellow () { printdot!(DotTypes::Light, DotColors::LightYellow ); }
    pub fn light_blue   () { printdot!(DotTypes::Light, DotColors::LightBlue   ); }
    pub fn light_magenta() { printdot!(DotTypes::Light, DotColors::LightMagenta); }
    pub fn light_cyan   () { printdot!(DotTypes::Light, DotColors::LightCyan   ); }
    pub fn light_white  () { printdot!(DotTypes::Light, DotColors::LightWhite  ); }
    pub fn light_default() { printdot!(DotTypes::Light, DotColors::LightDefault); }

    pub fn bg_black        () { printdot!(DotTypes::Light, DotColors::BackgroundBlack       ); }
    pub fn bg_red          () { printdot!(DotTypes::Light, DotColors::BackgroundRed         ); }
    pub fn bg_green        () { printdot!(DotTypes::Light, DotColors::BackgroundGreen       ); }
    pub fn bg_yellow       () { printdot!(DotTypes::Light, DotColors::BackgroundYellow      ); }
    pub fn bg_blue         () { printdot!(DotTypes::Light, DotColors::BackgroundBlue        ); }
    pub fn bg_magenta      () { printdot!(DotTypes::Light, DotColors::BackgroundMagenta     ); }
    pub fn bg_cyan         () { printdot!(DotTypes::Light, DotColors::BackgroundCyan        ); }
    pub fn bg_white        () { printdot!(DotTypes::Light, DotColors::BackgroundWhite       ); }
    pub fn bg_default      () { printdot!(DotTypes::Light, DotColors::BackgroundDefault     ); }

    pub fn bg_light_black  () { printdot!(DotTypes::Light, DotColors::BackgroundLightBlack  ); }
    pub fn bg_light_red    () { printdot!(DotTypes::Light, DotColors::BackgroundLightRed    ); }
    pub fn bg_light_green  () { printdot!(DotTypes::Light, DotColors::BackgroundLightGreen  ); }
    pub fn bg_light_yellow () { printdot!(DotTypes::Light, DotColors::BackgroundLightYellow ); }
    pub fn bg_light_blue   () { printdot!(DotTypes::Light, DotColors::BackgroundLightBlue   ); }
    pub fn bg_light_magenta() { printdot!(DotTypes::Light, DotColors::BackgroundLightMagenta); }
    pub fn bg_light_cyan   () { printdot!(DotTypes::Light, DotColors::BackgroundLightCyan   ); }
    pub fn bg_light_white  () { printdot!(DotTypes::Light, DotColors::BackgroundLightWhite  ); }
    pub fn bg_light_default() { printdot!(DotTypes::Light, DotColors::BackgroundLightDefault); }

    pub fn basic_true_color(dot_true_color   : DotTrueColor, arg: &str) {
        crate::colodot!(dot_true_color, arg);
    }

    pub fn basic_bg_true_color(dot_true_color: DotTrueColor, arg: &str) {
        crate::colodottc!(dot_true_color, arg);
    }

    pub fn true_color   (r: u32, g: u32, b: u32, args: &[&str]) {
        for arg in args {
            crate::colodot!(DotTrueColor{r: r, g: g, b: b}, arg);
        }
    }

    pub fn bg_true_color(r: u32, g: u32, b: u32, args: &[&str]) {
        for arg in args {
            crate::colodottc!(DotTrueColor{r: r, g: g, b: b}, arg);
        }
    }

    pub fn reset           () { printdot!(DotTypes::Light, DotColors::Reset                 ); }
}

#[macro_export]
macro_rules! colodot {
    ($dot_type: expr, $dot_color: expr,  $args: expr) => {
        let data: &str = $args;

        print!("\x1b[{};{}m{}", $dot_color as u8, $dot_type as u8, data);
    };

    // With reset expression.
    ($dot_type: expr, $dot_color: expr, $args: expr, $reset: expr) => {
        {
            let _force_to_bool: bool = $reset;

            colodot!($dot_type, $dot_color, $args);

            if _force_to_bool {
                crate::dot::reset();
            }
        }
    };

    ($dot_true_color: expr, $args: expr) => {
        let _type = $dot_true_color as DotTrueColor;

        // {} = r, {} = g, {} = b (u32)
        print!("\x1b[38;2;{};{};{}m{}", _type.r, _type.g, _type.b, $args);
    };
}

// Extended TrueColor macros
#[macro_export]
macro_rules! colodottc {
    // Background support
    ($dot_true_color: expr, $args: expr) => {
        let _type = $dot_true_color as DotTrueColor;

        // {} = r, {} = g, {} = b (u32)
        print!("\x1b[48;2;{};{};{}m{}", _type.r, _type.g, _type.b, $args);
    };

    ($dot_true_color: expr, $args: expr, $reset: expr) => {
        let _force_to_bool: bool = $reset;

        colodot!($dot_true_color, $args);

        if _force_to_bool {
            crate::dot::reset();
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::dot::{
        *,
        dot_defineds
    };

    #[test]
    fn play_with_colors() {
        red()  ; println!("{}", "Hello, red!"  );
        green(); println!("{}", "Hello, green!");
        blue() ; println!("{}", "Hello, blue!" );

        light_magenta(); println!("{}", "Hello light magenta!");

        reset();
    }

    #[test]
    fn play_with_types() {
        colodot!(DotTypes::Bold, DotColors::Yellow, "Hello, bold yellow!\n", true);
        colodot!(DotTypes::Blink,DotColors::BackgroundBlue, "Hello, background blink blue!\n", true);
    }

    #[test]
    fn play_with_lazys() {
        println!("{}{}\n", *dot_defineds::BG_BLUE_COLOR, "Hello, world!"    );
        println!("{}{}\n", *dot_defineds::BOLD_RED_COLOR, "Hello, bold red!");
        println!("{}{}\n", *dot_defineds::BOLD_BG_BLUE_COLOR, "Hello, bold background blue!");
        reset();
    }

    #[test]
    fn play_with_truecolor() {
        colodot!  (DotTrueColor{r: 31, g: 69, b: 100   }, "Hello, world!\n");
        colodottc!(DotTrueColor{r: 200, g: 100, b: 12}, "Hello, world!", true);
        colodottc!(DotTrueColor{r: 12, g: 50, b: 40  }, "Hello, world!");
    }

    #[test]
    fn play_with_truecolor_functions() {
        true_color(1, 1, 1, &["Hello", " , ", "world", "!"]);
        bg_true_color(230, 15, 60, &["Hello", " , ", "world", "!"]);

        basic_true_color(DotTrueColor{r: 5, g: 5, b: 5}, "Hello, world!");
        basic_bg_true_color(DotTrueColor{r: 235, g: 20, b: 65}, "Hello, world!");

        reset();
    }
}