#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod methods;

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk_proc_macros::zome;
use meta_traits::{GlobalEntryRef, InterDNADao};

pub struct InterDNA();

#[zome]
pub mod shortform_expression {
    #[entry_def]
    pub fn expression_entry_def() -> ValidatingEntryType {
        entry!(
            name: "global_entry_ref",
            description: "Public Entry DNA Reference",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<GlobalEntryRef>| {
                Ok(())
            },

            links: [
                to!(
                    "global_entry_ref",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[init]
    pub fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    #[zome_fn("inter_dna")]
    pub fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        InterDNA::create_link(source, target)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("inter_dna")]
    pub fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        InterDNA::remove_link(source, target)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("inter_dna")]
    pub fn get_outgoing(source: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        InterDNA::get_outgoing(source)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("inter_dna")]
    pub fn get_incoming(target: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        InterDNA::get_incoming(target)
    }
}