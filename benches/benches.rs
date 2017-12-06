#![feature(test)]

extern crate kankyo;
extern crate test;

use self::test::Bencher;
use std::io::Cursor;

#[bench]
fn unload_from_reader(b: &mut Bencher) {
    let s = "KEY=VALUE\nKEY2=VALUE2\nKEY3=VALUE3\nKEY4=VALUE4#abc".to_owned();
    let mut cursor = Cursor::new(s);

    b.iter(|| {
        let _ = kankyo::unload_from_reader(&mut cursor);
        cursor.set_position(0);
    });
}

mod utils {
    use kankyo;
    use test::Bencher;

    #[bench]
    fn only_keys(b: &mut Bencher) {
        let lines = kankyo::utils::parse_lines("KEY=VALUE\nKEY2=VALUE2\nA=B\nC=D");

        b.iter(|| {
            kankyo::utils::only_keys(&lines, &mut Vec::new());
        });
    }

    #[bench]
    fn parse_line(b: &mut Bencher) {
        b.iter(|| {
            kankyo::utils::parse_line("KEY=VALUE");
        });
    }

    #[bench]
    fn parse_line_multi(b: &mut Bencher) {
        b.iter(|| {
            kankyo::utils::parse_line("KEY=VALUE");
            kankyo::utils::parse_line("KEY2=VALUE2");
            kankyo::utils::parse_line("KEY3=VALUE#abcdef");
            kankyo::utils::parse_line("KEY#4=VALUE");
            kankyo::utils::parse_line("#KEY=VALUE");
        });
    }

    #[bench]
    fn parse_lines(b: &mut Bencher) {
        let s = "KEY=VALUE\nKEY2=VALUE2\nKEY3=VALUE3\nKEY4=VALUE4#abc";

        b.iter(|| {
            kankyo::utils::parse_line(s);
        });
    }

    #[bench]
    fn set_variables(b: &mut Bencher) {
        let s = "KEY=VALUE\nKEY2=VALUE2\nKEY3=VALUE3\nKEY4=VALUE4#abc";
        let lines = kankyo::utils::parse_lines(s);

        b.iter(|| {
            kankyo::utils::set_variables(&lines);
        });
    }

    #[bench]
    fn unload(b: &mut Bencher) {
        let s = &["FOO", "BAR", "BAZ"];

        b.iter(|| {
            kankyo::utils::unload(s);
        });
    }

    #[bench]
    fn unload_from_parsed_lines(b: &mut Bencher) {
        let s = "KEY=VALUE\nKEY2=VALUE2\nKEY3=VALUE3\nKEY4=VALUE4#abc";
        let lines = kankyo::utils::parse_lines(s);

        b.iter(|| {
            kankyo::utils::unload_from_parsed_lines(&lines);
        });
    }
}
