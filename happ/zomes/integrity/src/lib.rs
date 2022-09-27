use hdi::prelude::*;

#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq)]
pub struct Tag {
    pub background_color: String,
    pub text: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
#[derive(Clone)]
pub enum EntryTypes {
    #[entry_def]
    Tag(Tag),
}

#[hdk_link_types]
pub enum LinkTypes {
    All,
}