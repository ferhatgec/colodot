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

        // Foreground colors
        ForegroundBlack      = 40 ,
        ForegroundRed             ,
        ForegroundGreen           ,
        ForegroundYellow          ,
        ForegroundBlue            ,
        ForegroundMagenta         ,
        ForegroundCyan            ,
        ForegroundWhite           , // 47
        ForegroundDefault    = 49 ,

        // Foreground light colors
        ForegroundLightBlack = 100,
        ForegroundLightRed        ,
        ForegroundLightGreen      ,
        ForegroundLightYellow     ,
        ForegroundLightBlue       ,
        ForegroundLightMagenta    ,
        ForegroundLightCyan       ,
        ForegroundLightWhite      , // 107
        ForegroundLightDefault=109
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
        Underlined,
        Blink     ,
        Reverse   ,
        Hidden
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

            // Foreground colors
            pub static ref FG_BLACK_COLOR     : String = makedot!(DotTypes::Light, DotColors::ForegroundBlack  );
            pub static ref FG_RED_COLOR       : String = makedot!(DotTypes::Light, DotColors::ForegroundRed    );
            pub static ref FG_GREEN_COLOR     : String = makedot!(DotTypes::Light, DotColors::ForegroundGreen  );
            pub static ref FG_YELLOW_COLOR    : String = makedot!(DotTypes::Light, DotColors::ForegroundYellow );
            pub static ref FG_BLUE_COLOR      : String = makedot!(DotTypes::Light, DotColors::ForegroundBlue   );
            pub static ref FG_MAGENTA_COLOR   : String = makedot!(DotTypes::Light, DotColors::ForegroundMagenta);
            pub static ref FG_CYAN_COLOR      : String = makedot!(DotTypes::Light, DotColors::ForegroundCyan   );
            pub static ref FG_WHITE_COLOR     : String = makedot!(DotTypes::Light, DotColors::ForegroundWhite  );
            pub static ref FG_DEFAULT_COLOR   : String = makedot!(DotTypes::Light, DotColors::ForegroundDefault);

            // Foreground light colors
            pub static ref FG_LIGHT_BLACK_COLOR  : String = makedot!(DotTypes::Light, DotColors::ForegroundLightBlack  );
            pub static ref FG_LIGHT_RED_COLOR    : String = makedot!(DotTypes::Light, DotColors::ForegroundLightRed    );
            pub static ref FG_LIGHT_GREEN_COLOR  : String = makedot!(DotTypes::Light, DotColors::ForegroundLightGreen  );
            pub static ref FG_LIGHT_YELLOW_COLOR : String = makedot!(DotTypes::Light, DotColors::ForegroundLightYellow );
            pub static ref FG_LIGHT_BLUE_COLOR   : String = makedot!(DotTypes::Light, DotColors::ForegroundLightBlue   );
            pub static ref FG_LIGHT_MAGENTA_COLOR: String = makedot!(DotTypes::Light, DotColors::ForegroundLightMagenta);
            pub static ref FG_LIGHT_CYAN_COLOR   : String = makedot!(DotTypes::Light, DotColors::ForegroundLightCyan   );
            pub static ref FG_LIGHT_WHITE_COLOR  : String = makedot!(DotTypes::Light, DotColors::ForegroundLightWhite  );
            pub static ref FG_LIGHT_DEFAULT_COLOR: String = makedot!(DotTypes::Light, DotColors::ForegroundLightDefault);

            // TODO: Bold
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

    pub fn fg_black        () { printdot!(DotTypes::Light, DotColors::ForegroundBlack       ); }
    pub fn fg_red          () { printdot!(DotTypes::Light, DotColors::ForegroundRed         ); }
    pub fn fg_green        () { printdot!(DotTypes::Light, DotColors::ForegroundGreen       ); }
    pub fn fg_yellow       () { printdot!(DotTypes::Light, DotColors::ForegroundYellow      ); }
    pub fn fg_blue         () { printdot!(DotTypes::Light, DotColors::ForegroundBlue        ); }
    pub fn fg_magenta      () { printdot!(DotTypes::Light, DotColors::ForegroundMagenta     ); }
    pub fn fg_cyan         () { printdot!(DotTypes::Light, DotColors::ForegroundCyan        ); }
    pub fn fg_white        () { printdot!(DotTypes::Light, DotColors::ForegroundWhite       ); }
    pub fn fg_default      () { printdot!(DotTypes::Light, DotColors::ForegroundDefault     ); }

    pub fn fg_light_black  () { printdot!(DotTypes::Light, DotColors::ForegroundLightBlack  ); }
    pub fn fg_light_red    () { printdot!(DotTypes::Light, DotColors::ForegroundLightRed    ); }
    pub fn fg_light_green  () { printdot!(DotTypes::Light, DotColors::ForegroundLightGreen  ); }
    pub fn fg_light_yellow () { printdot!(DotTypes::Light, DotColors::ForegroundLightYellow ); }
    pub fn fg_light_blue   () { printdot!(DotTypes::Light, DotColors::ForegroundLightBlue   ); }
    pub fn fg_light_magenta() { printdot!(DotTypes::Light, DotColors::ForegroundLightMagenta); }
    pub fn fg_light_cyan   () { printdot!(DotTypes::Light, DotColors::ForegroundLightCyan   ); }
    pub fn fg_light_white  () { printdot!(DotTypes::Light, DotColors::ForegroundLightWhite  ); }
    pub fn fg_light_default() { printdot!(DotTypes::Light, DotColors::ForegroundLightDefault); }

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
        colodot!(DotTypes::Blink,DotColors::ForegroundBlue, "Hello, foreground blink blue!\n", true);
    }

    #[test]
    fn play_with_lazys() {
        println!("{}{}", *dot_defineds::FG_BLUE_COLOR, "Hello, world!"); reset();
    }

    #[test]
    fn play_with_truecolor() {
        colodot!(DotTrueColor{r: 31, g: 69, b: 100}, "Hello, world!\n");
        colodottc!(DotTrueColor{r: 200, g: 100, b: 12}, "Hello, world!", true);
    }
}