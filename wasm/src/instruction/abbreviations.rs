use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref ABBREVIATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("sc", "single crochet");
        m.insert("hdc", "half double crochet");
        m.insert("dc", "double crochet");
        m.insert("inc", "increase (2 sc in same stitch)");
        m.insert("dec", "decrease (sc2tog)");
        m.insert("ch", "chain");
        m.insert("sl st", "slip stitch");
        m.insert("yo", "yarn over");
        m.insert("rep", "repeat");
        m
    };
}
