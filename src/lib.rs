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

    pub enum DotTypes {
        Light     ,
        Bold      ,
        Dim       ,
        Underlined,
        Blink     ,
        Reverse   ,
        Hidden
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
    ($dot_type:expr, $dot_color:expr,  $args:expr) => {
        let data: &str = $args;

        print!("\x1b[{};{}m{}", $dot_color as u8, $dot_type as u8, data);
    };

    ($dot_type:expr, $dot_color:expr, $args:expr, $reset:expr) => {
        {
            let _force_to_bool: bool = $reset;

            colodot!($dot_type, $dot_color, $args);

            if _force_to_bool {
                print!("{}", "\x1b[0m");
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::dot::*;

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
}