#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod methods;

use hdk::holochain_core_types::{dna::entry_types::Sharing, signature::Provenance};
use hdk::prelude::{Address, GetEntryOptions, GetEntryResultType};
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk_proc_macros::zome;
use meta_traits::{GlobalEntryRef, InterDNADao};

pub struct InterDNA();

fn get_entry_provenances(address: &Address) -> Result<Vec<Provenance>, String> {
    match hdk::get_entry_result(
        address,
        GetEntryOptions {
            status_request: Default::default(),
            entry: false,
            headers: true,
            timeout: Default::default(),
        },
    )?
    .result
    {
        GetEntryResultType::Single(item) => {
            hdk::debug(format!("Got item: {:#?} for validation", item)).unwrap();
            item.meta
                .ok_or(String::from("Could not find link base/target"))?;
            Ok(item.headers[0].provenances().to_owned())
        }
        GetEntryResultType::All(_items) => Err(String::from("Reached all branch")),
    }
}

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
                    validation: | validation_data: hdk::LinkValidationData | {
                        match validation_data {
                            hdk::LinkValidationData::LinkAdd{link: _, validation_data: _} => Ok(()),
                            hdk::LinkValidationData::LinkRemove{link, validation_data: _} => {
                                let source_provenances = get_entry_provenances(link.link.base())?
                                    .into_iter()
                                    .map(|val| val.source())
                                    .collect::<Vec<Address>>();
                                let target_provenances = get_entry_provenances(link.link.target())?
                                    .into_iter()
                                    .map(|val| val.source())
                                    .collect::<Vec<Address>>();
                                let links_provenances = link.top_chain_header.provenances();

                                if source_provenances.contains(&links_provenances[0].source()) | target_provenances.contains(&links_provenances[0].source()) {
                                    Ok(())
                                } else {
                                    Err(String::from("Provenances on base/target of link do not match to link provenances"))
                                }
                            }
                        }
                    }
                ),
                from!(
                    "global_entry_ref",
                    link_type: "incoming",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | validation_data: hdk::LinkValidationData | {
                        match validation_data {
                            hdk::LinkValidationData::LinkAdd{link: _, validation_data: _} => Ok(()),
                            hdk::LinkValidationData::LinkRemove{link, validation_data: _} => {
                                let source_provenances = get_entry_provenances(link.link.base())?
                                    .into_iter()
                                    .map(|val| val.source())
                                    .collect::<Vec<Address>>();
                                let target_provenances = get_entry_provenances(link.link.target())?
                                    .into_iter()
                                    .map(|val| val.source())
                                    .collect::<Vec<Address>>();
                                let links_provenances = link.top_chain_header.provenances();

                                if source_provenances.contains(&links_provenances[0].source()) | target_provenances.contains(&links_provenances[0].source()) {
                                    Ok(())
                                } else {
                                    Err(String::from("Provenances on base/target of link do not match to link provenances"))
                                }
                            }
                        }
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
    pub fn get_outgoing(
        source: GlobalEntryRef,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        InterDNA::get_outgoing(source, count, page)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("inter_dna")]
    pub fn get_incoming(
        target: GlobalEntryRef,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        InterDNA::get_incoming(target, count, page)
    }
}
