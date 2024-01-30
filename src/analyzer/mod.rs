
pub mod domains{
    pub mod sign_domain;
    pub mod interval_domain;
    pub mod extended_num;
    pub mod bounded_interval_domain;
    pub mod extended_sign_domain;
}
pub mod analyzers {
    pub mod interval_analyzer;
    pub mod generic_analyzer;
}
pub mod types {
    pub mod program;
    pub mod state;
    pub mod domain;
    pub mod analyzer;
}
pub mod states {
    pub mod hashmap_state;
}
pub mod abstract_translator;
pub mod printers;
pub mod tests;


