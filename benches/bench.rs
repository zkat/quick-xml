#![feature(test)]

extern crate xml;
extern crate quick_xml;
extern crate test;

use test::{Bencher};
use quick_xml::{XmlReader, Event};
use xml::reader::{EventReader, XmlEvent};

#[bench]
fn bench_quick_xml(b: &mut Bencher) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    b.iter(|| {
        let r = XmlReader::from_reader(src);
        let mut count = test::black_box(0);
        for e in r {
            if let Ok(Event::Start(_)) = e {
                count += 1;
            }
        }
        assert!(count == 1550);
    });
}

#[bench]
fn bench_xml_rs(b: &mut Bencher) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    b.iter(|| {
        let r = EventReader::new(src);
        let mut count = test::black_box(0);
        for e in r {
            if let Ok(XmlEvent::StartElement { .. }) = e {
                count += 1;
            }
        }
        assert!(count == 1550);
    });
}