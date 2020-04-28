use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_json_api::json::JsonString,
    prelude::{Address, Entry},
};
use meta_traits::{GlobalEntryRef, InterDNADao};
use std::convert::TryInto;

use crate::InterDNA;

impl InterDNADao for InterDNA {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        let source_address: Address = JsonString::from(source.clone()).try_into()?;
        if hdk::get_entry(&source_address)?.is_none() {
            let source_entry = Entry::App("global_entry_ref".into(), source.into());
            hdk::commit_entry(&source_entry)?;
        };

        let target_address: Address = JsonString::from(target.clone()).try_into()?;
        if hdk::get_entry(&target_address)?.is_none() {
            let target_entry = Entry::App("global_entry_ref".into(), target.into());
            hdk::commit_entry(&target_entry)?;
        };

        hdk::link_entries(&source_address, &target_address, "", "")?;
        Ok(())
    }

    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        let source_address: Address = JsonString::from(source.clone()).try_into()?;
        hdk::get_entry(&source_address)?.ok_or(ZomeApiError::Internal(String::from(
            "Source entry does not exist",
        )))?;

        let target_address: Address = JsonString::from(target.clone()).try_into()?;
        hdk::get_entry(&target_address)?.ok_or(ZomeApiError::Internal(String::from(
            "Target entry does not exist",
        )))?;

        hdk::remove_link(&source_address, &target_address, "", "")?;
        Ok(())
    }

    fn get_outgoing(_source: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        Ok(vec![])
    }

    fn get_incoming(_target: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        Ok(vec![])
    }
}
